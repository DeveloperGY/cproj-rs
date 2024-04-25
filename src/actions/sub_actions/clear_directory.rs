use std::{fs, path};

use crate::action::{Action, Result};

pub struct ClearDirectory {
    path: path::PathBuf,
}

impl ClearDirectory {
    pub fn new<P>(path: P) -> Box<Self>
    where
        P: AsRef<path::Path>,
    {
        Box::new(Self {
            path: path.as_ref().to_path_buf(),
        })
    }
}

impl Action for ClearDirectory {
    fn execute(&mut self) -> Result<()> {
        if fs::remove_dir_all(&self.path).is_err() {
            Err(format!(
                "failed to clear {}",
                self.path.to_str().unwrap_or("directory")
            )
            .into())
        } else {
            Ok(())
        }
    }

    fn undo(&mut self) -> Result<()> {
        Ok(())
    }
}
