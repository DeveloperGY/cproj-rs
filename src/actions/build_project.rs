use std::{cell::RefCell, collections::HashSet, rc::Rc};

use crate::{
    action::{Action, Result},
    arg_retriever::{ArgRetriever, ArgRule},
    config::{Config, Lang},
    graph::Graph,
    ActionChain, CompileFiles, FetchEditedFiles, GenDepGraph, GenSrcPaths, ReadConfig,
};

pub struct BuildProject {
    action_chain: Box<ActionChain>,
}

impl BuildProject {
    pub fn new(args: &[&str]) -> Box<Self> {
        // Create Argument Retriever
        let mut arg_retriever = Self::create_arg_retriever();
        arg_retriever.load(args);

        // Get Necessary Arguments
        let release_mode = Self::release_mode(&arg_retriever);

        // Create Action Arguments
        let config = Rc::new(RefCell::new(Config::new("", Lang::C)));
        let src_paths = Rc::new(RefCell::new(HashSet::new()));
        let dependency_graph = Rc::new(RefCell::new(Graph::new()));
        let changed_files = Rc::new(RefCell::new(HashSet::new()));

        // Create Action Chain
        let mut action_chain = ActionChain::new();

        action_chain
            .add(ReadConfig::new(Rc::clone(&config)))
            .add(GenSrcPaths::new(Rc::clone(&src_paths), Rc::clone(&config)))
            .add(GenDepGraph::new(
                Rc::clone(&src_paths),
                Rc::clone(&dependency_graph),
            ))
            .add(FetchEditedFiles::new(
                Rc::clone(&src_paths),
                Rc::clone(&dependency_graph),
                Rc::clone(&changed_files),
                release_mode,
            ))
            .add(CompileFiles::new(
                Rc::clone(&src_paths),
                Rc::clone(&changed_files),
                Rc::clone(&config),
                release_mode,
            ));

        Box::new(Self { action_chain })
    }

    fn create_arg_retriever() -> ArgRetriever {
        let rules = [ArgRule::new("--release", 0)];
        ArgRetriever::new(&rules)
    }

    fn release_mode(arg_ret: &ArgRetriever) -> bool {
        arg_ret.has_tag("--release")
    }
}

impl Action for BuildProject {
    fn execute(&mut self) -> Result<()> {
        self.action_chain
            .execute()
            .map_err(|err| err.prepend("Failed to build project: "))
    }

    fn undo(&mut self) -> Result<()> {
        self.action_chain
            .undo()
            .map_err(|err| err.prepend("Failed to undo project build: "))
    }
}
