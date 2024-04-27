use std::{cell::RefCell, collections::HashSet, fs, path, rc::Rc};

use crate::{
    action::{Action, Result},
    config::{Config, Lang},
};

pub struct GenSrcPaths {
    config: Rc<RefCell<Config>>,
    src_exts: Vec<String>,
    src_paths: Rc<RefCell<HashSet<path::PathBuf>>>,
    old_src_paths: Option<HashSet<path::PathBuf>>,
}

impl GenSrcPaths {
    pub fn new(
        src_paths: Rc<RefCell<HashSet<path::PathBuf>>>,
        config: Rc<RefCell<Config>>,
    ) -> Box<Self> {
        Box::new(Self {
            config,
            src_exts: Self::get_extensions(Lang::C),
            src_paths,
            old_src_paths: None,
        })
    }

    fn get_extensions(lang: Lang) -> Vec<String> {
        match lang {
            Lang::C => ["c", "i", "h"]
                .iter_mut()
                .map(|val| val.to_string())
                .collect(),
            Lang::Cpp => [
                "c", "cc", "cp", "cxx", "cpp", "CPP", "c++", "C", "i", "ii", "h", "hh", "H", "hp",
                "hxx", "hpp", "HPP", "h++", "tcc",
            ]
            .iter_mut()
            .map(|val| val.to_string())
            .collect(),
        }
    }

    fn get_file_paths(&self) -> Result<HashSet<path::PathBuf>> {
        let mut file_paths = HashSet::new();
        let mut directories = vec![path::PathBuf::from("src")];

        while let Some(path) = directories.pop() {
            let entries = match fs::read_dir(&path) {
                Ok(val) => val,
                Err(_) => {
                    return Err(
                        format!("Failed to read {}", path.to_str().unwrap_or("directory")).into(),
                    );
                }
            };

            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    directories.push(entry.path());
                } else {
                    file_paths.insert(entry.path());
                }
            }
        }

        Ok(file_paths)
    }
}

impl Action for GenSrcPaths {
    fn execute(&mut self) -> Result<()> {
        println!("=> Fetching Source Files...");
        self.old_src_paths = Some(self.src_paths.borrow().clone());

        self.src_exts = Self::get_extensions(self.config.borrow().lang);

        *self.src_paths.borrow_mut() = self
            .get_file_paths()?
            .into_iter()
            .filter(|path| {
                // filter out files that arent source or header files
                if let Some(ext) = path.extension() {
                    let ext = ext.to_string_lossy().to_string();
                    self.src_exts.contains(&ext)
                } else {
                    false
                }
            })
            .map(|path| {
                println!("    -> found {}", path.as_path().to_str().unwrap());
                path
            })
            .collect();

        Ok(())
    }

    fn undo(&mut self) -> Result<()> {
        if self.old_src_paths.is_some() {
            *self.src_paths.borrow_mut() = self.old_src_paths.take().unwrap();
        }

        Ok(())
    }
}
