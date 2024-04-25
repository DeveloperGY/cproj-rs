use std::{
    cell::RefCell,
    collections::HashSet,
    fs,
    path::{self, PathBuf},
    rc::Rc,
    time,
};

use crate::{
    action::{Action, Result},
    graph::Graph,
};

/// Gets the changed files and their dependencies
pub struct FetchEditedFiles {
    changed_files: Rc<RefCell<HashSet<path::PathBuf>>>,
    src_files: Rc<RefCell<HashSet<path::PathBuf>>>,
    dependency_graph: Rc<RefCell<Graph<path::PathBuf>>>,
    old_changed_files: Option<HashSet<path::PathBuf>>,
    release_mode: bool,
}

impl FetchEditedFiles {
    pub fn new(
        src_files: Rc<RefCell<HashSet<path::PathBuf>>>,
        dependency_graph: Rc<RefCell<Graph<path::PathBuf>>>,
        changed_files: Rc<RefCell<HashSet<path::PathBuf>>>,
        release_mode: bool,
    ) -> Box<Self> {
        Box::new(Self {
            changed_files,
            src_files,
            dependency_graph,
            old_changed_files: None,
            release_mode,
        })
    }

    fn get_edited_entries(&self, compile_mtime: time::SystemTime) -> HashSet<PathBuf> {
        self.src_files
            .borrow()
            .iter()
            .filter(|path| {
                if let Ok(meta) = fs::metadata(*path) {
                    if let Ok(mtime) = meta.modified() {
                        compile_mtime.duration_since(mtime).is_err()
                    } else {
                        true
                    }
                } else {
                    true
                }
            })
            .cloned()
            .collect()
    }
}

impl Action for FetchEditedFiles {
    fn execute(&mut self) -> Result<()> {
        println!("=> Detecting Changed Files...");

        self.old_changed_files = Some(self.changed_files.borrow().clone());
        self.changed_files.borrow_mut().clear();

        let timestamp_path = if self.release_mode {
            "bin/release/timestamp"
        } else {
            "bin/debug/timestamp"
        };

        let compile_mtime = fs::metadata(timestamp_path)
            .map(|meta| meta.modified().unwrap_or(time::UNIX_EPOCH))
            .unwrap_or(time::UNIX_EPOCH);

        // Mark all files as changed
        if compile_mtime == time::UNIX_EPOCH {
            println!("    -> Timestamp not found, marking all files as changed");
            *self.changed_files.borrow_mut() = self.src_files.borrow().clone();
        }
        // Mark only modified files and their dependents as changed
        else {
            println!("    -> Timestamp found, filtering unchanged files...");
            let mut entries_to_mark: Vec<_> =
                self.get_edited_entries(compile_mtime).into_iter().collect();

            let mut marked_entries = HashSet::new();

            while let Some(entry) = entries_to_mark.pop() {
                // get entry dependents
                let deps = self
                    .dependency_graph
                    .borrow()
                    .get_connected(&entry)
                    .cloned();

                // mark entry if it isnt already
                if !marked_entries.contains(&entry) {
                    // ensure entry dependents get marked
                    if let Some(deps) = deps {
                        let deps: Vec<_> = deps
                            .into_iter()
                            .filter(|val| {
                                !entries_to_mark.contains(val) && !marked_entries.contains(val)
                            })
                            .collect();

                        for dep in deps {
                            entries_to_mark.push(dep);
                        }
                    }

                    // mark entry
                    marked_entries.insert(entry.clone());
                    println!("        - changes found in {}", entry.to_str().unwrap());
                }
            }

            *self.changed_files.borrow_mut() = marked_entries;
        }

        Ok(())
    }

    fn undo(&mut self) -> Result<()> {
        if self.old_changed_files.is_some() {
            *self.changed_files.borrow_mut() = self.old_changed_files.take().unwrap();
        }

        Ok(())
    }
}
