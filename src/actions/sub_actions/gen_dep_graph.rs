use std::{cell::RefCell, collections::HashSet, fs, path, rc::Rc};

use crate::{
    action::{Action, Result},
    graph::Graph,
};

pub struct GenDepGraph {
    src_paths: Rc<RefCell<HashSet<path::PathBuf>>>,
    dep_graph: Rc<RefCell<Graph<path::PathBuf>>>,
    old_graph: Option<Graph<path::PathBuf>>,
}

impl GenDepGraph {
    pub fn new(
        src_paths: Rc<RefCell<HashSet<path::PathBuf>>>,
        dep_graph: Rc<RefCell<Graph<path::PathBuf>>>,
    ) -> Box<Self> {
        Box::new(Self {
            src_paths,
            dep_graph,
            old_graph: None,
        })
    }

    // this function removes indirection from a path found in the project src folder
    // relative to the project root
    fn root_path(&self, path: &path::Path) -> Option<path::PathBuf> {
        let src_root = path::PathBuf::from("src").canonicalize().ok()?;

        Some(
            path::PathBuf::from("src").join(path.canonicalize().ok()?.strip_prefix(src_root).ok()?),
        )
    }

    fn get_include_paths(&self, code: &str) -> Vec<String> {
        code.lines()
            .filter(|line| line.contains("#include"))
            .filter(|line| line.chars().any(|c| c == '"'))
            .map(|line| {
                line.replace("#include", "")
                    .replace('"', "")
                    .trim()
                    .to_string()
            })
            .collect()
    }

    fn create_nodes(&self) -> Result<()> {
        for path in self.src_paths.borrow().iter() {
            if let Some(path) = self.root_path(path) {
                self.dep_graph.borrow_mut().create_node(path);
            } else {
                println!(
                    "    -> failed to find {} relative to the project root",
                    path.to_str().unwrap()
                );
                return Err("Failed to simplify dependency path".into());
            }
        }

        Ok(())
    }

    fn generate_node_edges(&self, entry: &path::Path) -> Result<()> {
        println!("    -> dependencies of {}", entry.to_str().unwrap());

        // Get src code from file
        let code = match fs::read_to_string(entry) {
            Ok(val) => val,
            Err(_) => {
                println!("    -> failed to read {}", entry.to_str().unwrap());
                return Err("Failed to read src file".into());
            }
        };

        // get included files
        let included_files: Vec<_> = self.get_include_paths(&code);

        for path in &included_files {
            // path of the included file relative to the project root
            // it may or may not contain indirections in the form of '../'
            let path_rel_root_indirect = entry.parent().unwrap().join(path);

            // include located in src folder, generate dependency connection
            if let Some(path_rel_root) = self.root_path(&path_rel_root_indirect) {
                println!("        - {}", path_rel_root.to_str().unwrap());
                self.dep_graph
                    .borrow_mut()
                    .create_edge(&path_rel_root, &entry.to_path_buf());
            }
            // include not located in src folder, error out
            else {
                println!(
                    "    -> include file not located in src folder: {}",
                    path_rel_root_indirect.to_str().unwrap()
                );
                return Err(format!(
                    "Failed to validate include {} found in {}",
                    path_rel_root_indirect.to_str().unwrap(),
                    entry.to_str().unwrap()
                )
                .into());
            }
        }

        Ok(())
    }
}

impl Action for GenDepGraph {
    fn execute(&mut self) -> Result<()> {
        println!("=> Generating Dependency Graph...");
        self.old_graph = Some(self.dep_graph.borrow().clone());
        self.dep_graph.borrow_mut().clear();

        self.create_nodes()?;

        for entry in self.src_paths.borrow().iter() {
            self.generate_node_edges(entry)?;
        }

        Ok(())
    }

    fn undo(&mut self) -> Result<()> {
        if self.old_graph.is_some() {
            *self.dep_graph.borrow_mut() = self.old_graph.take().unwrap()
        }

        Ok(())
    }
}
