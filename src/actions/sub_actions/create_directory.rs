use std::{fs, path};

use crate::action::{Action, Result};

enum CreateDirectoryState {
    Unexecuted,
    DirectoryCreated,
}

/// Creates a directory
/// # Error
/// If `path` already exists or doesnt point to a directory then [`CreateDirectory::execute()`] will
/// return an error
///
pub struct CreateDirectory {
    path: path::PathBuf,
    state: CreateDirectoryState,
}

impl CreateDirectory {
    pub fn new<P>(path: P) -> Box<Self>
    where
        P: AsRef<path::Path>,
    {
        Box::new(Self {
            path: path.as_ref().to_path_buf(),
            state: CreateDirectoryState::Unexecuted,
        })
    }
}

impl Action for CreateDirectory {
    fn execute(&mut self) -> Result<()> {
        if fs::create_dir(&self.path).is_err() {
            Err(format!(
                "Failed to create {}",
                self.path.to_str().unwrap_or("directory")
            )
            .into())
        } else {
            self.state = CreateDirectoryState::DirectoryCreated;
            Ok(())
        }
    }

    fn undo(&mut self) -> Result<()> {
        match self.state {
            CreateDirectoryState::Unexecuted => Ok(()),
            CreateDirectoryState::DirectoryCreated => {
                if fs::remove_dir(&self.path).is_err() {
                    Err(format!(
                        "Failed to remove {}",
                        self.path.to_str().unwrap_or("directory")
                    )
                    .into())
                } else {
                    self.state = CreateDirectoryState::Unexecuted;
                    Ok(())
                }
            }
        }
    }
}
