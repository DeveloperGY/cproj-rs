use std::{fs, io::Write, path};

use crate::action::{Action, Result};

enum CreateFileState {
    Unexecuted,
    FileCreated,
}

/// Creates a file and writes data to it
/// # Error
/// If `path` already exists or doesnt point to a file then [`CreateFile::execute()`] will
/// return an error
///
pub struct CreateFile {
    path: path::PathBuf,
    contents: Vec<u8>,
    state: CreateFileState,
}

impl CreateFile {
    pub fn new<P>(path: P, contents: &[u8]) -> Box<Self>
    where
        P: AsRef<path::Path>,
    {
        Box::new(Self {
            path: path.as_ref().to_path_buf(),
            contents: contents.to_vec(),
            state: CreateFileState::Unexecuted,
        })
    }
}

impl Action for CreateFile {
    fn execute(&mut self) -> Result<()> {
        let mut file = match fs::File::create_new(&self.path) {
            Ok(file) => file,
            Err(_) => {
                return Err(
                    format!("Failed to create {}", self.path.to_str().unwrap_or("file")).into(),
                );
            }
        };

        self.state = CreateFileState::FileCreated;

        if file.write_all(&self.contents).is_err() {
            Err(format!(
                "Failed to write to {}",
                self.path.to_str().unwrap_or("file"),
            )
            .into())
        } else {
            Ok(())
        }
    }

    fn undo(&mut self) -> Result<()> {
        match self.state {
            CreateFileState::Unexecuted => Ok(()),
            CreateFileState::FileCreated => {
                if fs::remove_file(&self.path).is_err() {
                    Err(format!("Failed to remove {}", self.path.to_str().unwrap_or("file")).into())
                } else {
                    self.state = CreateFileState::Unexecuted;
                    Ok(())
                }
            }
        }
    }
}
