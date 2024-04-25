use std::path;

use crate::action::{Action, Result};
use crate::actions::{ActionChain, CreateDirectory};
use crate::arg_retriever::{ArgRetriever, ArgRule};
use crate::config::{Config, Lang};
use crate::CreateFile;

/// cproj new --name [name] --lang [c, cpp] (default = --lang c)

enum NewProjectState {
    ValidArguments,
    NameNotFound,
    InvalidLang,
}

pub struct NewProject {
    action_chain: Box<ActionChain>,
    state: NewProjectState,
}

impl NewProject {
    pub fn new(args: &[&str]) -> Box<Self> {
        // Create and Load Argument Retriever
        let mut arg_retriever = Self::create_arg_retriever();
        arg_retriever.load(args);

        // Get Necessary Arguments
        let mut state = NewProjectState::ValidArguments;
        let name = Self::get_project_name(&arg_retriever).unwrap_or_else(|| {
            state = NewProjectState::NameNotFound;
            String::new()
        });
        let lang = Self::get_project_lang(&arg_retriever).unwrap_or_else(|| {
            state = NewProjectState::InvalidLang;
            Lang::C
        });

        // Create Action Arguments
        let project_root = path::PathBuf::from(&name);

        let config_string = serde_json::to_string_pretty(&Config::new(&name, lang)).unwrap();
        let config_path = project_root.join("cproj.json");
        let config_contents = config_string.as_bytes();

        let entry_point = match lang {
            Lang::C => project_root.join("src").join("main.c"),
            Lang::Cpp => project_root.join("src").join("main.cpp"),
        };
        let entry_contents = b"int main(int argc, char *argv[])\n{\n\treturn 0;\n}\n";

        // Create Action Chain
        let mut action_chain = ActionChain::new();

        action_chain
            .add(CreateDirectory::new(&project_root))
            .add(CreateDirectory::new(project_root.join("bin")))
            .add(CreateDirectory::new(project_root.join("include")))
            .add(CreateDirectory::new(project_root.join("lib")))
            .add(CreateDirectory::new(project_root.join("src")))
            .add(CreateDirectory::new(project_root.join("bin/debug")))
            .add(CreateDirectory::new(project_root.join("bin/release")))
            .add(CreateDirectory::new(project_root.join("bin/debug/log")))
            .add(CreateDirectory::new(project_root.join("bin/debug/obj")))
            .add(CreateDirectory::new(project_root.join("bin/release/log")))
            .add(CreateDirectory::new(project_root.join("bin/release/obj")))
            .add(CreateFile::new(entry_point, entry_contents))
            .add(CreateFile::new(config_path, config_contents));

        Box::new(Self {
            action_chain,
            state,
        })
    }

    fn create_arg_retriever() -> ArgRetriever {
        let rules = [ArgRule::new("--name", 1), ArgRule::new("--lang", 1)];
        ArgRetriever::new(&rules)
    }

    fn get_project_name(arg_ret: &ArgRetriever) -> Option<String> {
        arg_ret
            .get_tag_args("--name")
            .map(|args| args[0].clone())
            .or_else(|| arg_ret.get_untagged().first().cloned())
    }

    fn get_project_lang(arg_ret: &ArgRetriever) -> Option<Lang> {
        match arg_ret.get_tag_args("--lang") {
            None => Some(Lang::C),
            Some(args) => match args[0].to_lowercase().as_str() {
                "c" => Some(Lang::C),
                "cpp" | "c++" => Some(Lang::Cpp),
                _ => None,
            },
        }
    }
}

impl Action for NewProject {
    fn execute(&mut self) -> Result<()> {
        match self.state {
            NewProjectState::ValidArguments => self.action_chain.execute(),
            NewProjectState::InvalidLang => Err("Invalid lang argument".into()),
            NewProjectState::NameNotFound => Err("Project name not passed".into()),
        }
        .map_err(|err| err.prepend("Failed to create project: "))
    }

    fn undo(&mut self) -> Result<()> {
        // action chain keeps track of what it executed, so if the chain was
        // never executed or it already undid its actions, this wont do anything
        self.action_chain
            .undo()
            .map_err(|err| err.prepend("Failed to undo project creation: "))
    }
}
