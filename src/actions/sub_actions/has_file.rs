use std::path::{self, PathBuf};

use crate::action::{Action, Result};

pub struct HasFile {
    path: PathBuf,
}

impl HasFile {
    pub fn new<P>(path: P) -> Box<Self>
    where
        P: AsRef<path::Path>,
    {
        Box::new(Self {
            path: path.as_ref().to_path_buf(),
        })
    }
}

impl Action for HasFile {
    fn execute(&mut self) -> Result<()> {
        if self.path.exists() {
            Ok(())
        } else {
            Err(format!("{} doesnt exist", self.path.to_str().unwrap()).into())
        }
    }

    fn undo(&mut self) -> Result<()> {
        // nothing to undo
        Ok(())
    }
}
