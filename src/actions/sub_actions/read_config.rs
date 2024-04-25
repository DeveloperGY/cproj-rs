use std::{cell::RefCell, fs, path, rc::Rc};

use crate::{
    action::{Action, Result},
    config::Config,
};

pub struct ReadConfig {
    config: Rc<RefCell<Config>>,
    old_config: Option<Config>,
}

impl ReadConfig {
    pub fn new(config: Rc<RefCell<Config>>) -> Box<Self> {
        Box::new(Self {
            config,
            old_config: None,
        })
    }
}

impl Action for ReadConfig {
    fn execute(&mut self) -> Result<()> {
        println!("=> Reading Project Config...");

        self.old_config = Some(self.config.borrow().clone());
        let config_root = path::PathBuf::from("cproj.json");

        let config_string = match fs::read_to_string(config_root) {
            Ok(val) => val,
            Err(_) => {
                println!("    -> failed to read cproj.json");
                return Err("Failed to read config".into());
            }
        };

        let config = match serde_json::from_str(&config_string) {
            Ok(val) => val,
            Err(_) => {
                println!("    -> failed to parse cproj.json");
                return Err("Failed to parse config".into());
            }
        };

        *self.config.borrow_mut() = config;
        Ok(())
    }

    fn undo(&mut self) -> Result<()> {
        if self.old_config.is_some() {
            *self.config.borrow_mut() = self.old_config.take().unwrap();
        }
        Ok(())
    }
}
