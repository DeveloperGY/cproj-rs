use std::{cell::RefCell, process, rc::Rc};

use crate::{
    action::{Action, Result},
    arg_retriever::{ArgRetriever, ArgRule},
    config::Config,
    BuildProject, ReadConfig,
};

pub struct RunProject {
    build_action: Box<BuildProject>,
    fetch_config_action: Box<ReadConfig>,
    release_mode: bool,
    config: Rc<RefCell<Config>>,
    arg_retriever: ArgRetriever,
}

impl RunProject {
    pub fn new(args: &[&str]) -> Box<Self> {
        // Create Argument Retriever
        let mut arg_retriever = Self::create_arg_retriever();
        arg_retriever.load(args);

        // Get Necessary Arguments
        let release_mode = Self::release_mode(&arg_retriever);
        let config = Rc::new(RefCell::new(Config::new("", crate::config::Lang::C)));

        Box::new(Self {
            build_action: BuildProject::new(args),
            fetch_config_action: ReadConfig::new(Rc::clone(&config)),
            config,
            release_mode,
            arg_retriever,
        })
    }

    fn create_arg_retriever() -> ArgRetriever {
        let rules = [ArgRule::new("--release", 0)];
        ArgRetriever::new(&rules)
    }

    fn release_mode(arg_ret: &ArgRetriever) -> bool {
        arg_ret.has_tag("--release")
    }
}

impl Action for RunProject {
    fn execute(&mut self) -> Result<()> {
        self.build_action.execute()?;
        self.fetch_config_action.execute()?;

        let bin_path = if self.release_mode {
            format!("bin/release/{}", &self.config.borrow().name)
        } else {
            format!("bin/debug/{}", &self.config.borrow().name)
        };

        println!("=> Running executable\n");

        let mut bin = process::Command::new(bin_path);
        bin.args(&self.arg_retriever.get_untagged());

        bin.spawn().unwrap();

        Ok(())
    }

    fn undo(&mut self) -> Result<()> {
        self.fetch_config_action.undo()?;
        self.build_action.undo()?;

        Ok(())
    }
}
