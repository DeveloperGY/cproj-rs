use std::{env, path};

use crate::action::{Action, Result};
use crate::actions::{ActionChain, CreateDirectory};
use crate::arg_retriever::{ArgRetriever, ArgRule};
use crate::config::{Config, Lang};
use crate::CreateFile;

/// cproj new --name [name] --lang [c, cpp] (default = --lang c)

enum InitProjectState {
    ValidArguments,
    InvalidLang,
}

pub struct InitProject {
    action_chain: Box<ActionChain>,
    state: InitProjectState,
}

impl InitProject {
    pub fn new(args: &[&str]) -> Box<Self> {
        // Create and Load Argument Retriever
        let mut arg_retriever = Self::create_arg_retriever();
        arg_retriever.load(args);

        // Get Necessary Arguments
        let mut state = InitProjectState::ValidArguments;
        let name = Self::get_project_name(&arg_retriever);
        let lang = Self::get_project_lang(&arg_retriever).unwrap_or_else(|| {
            state = InitProjectState::InvalidLang;
            Lang::C
        });

        // Create Action Arguments
        let project_root = path::PathBuf::from(".");

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
            .add(CreateDirectory::new(project_root.join("bin")))
            .add(CreateDirectory::new(project_root.join("include")))
            .add(CreateDirectory::new(project_root.join("lib")))
            .add(CreateDirectory::new(project_root.join("src")))
            .add(CreateDirectory::new(project_root.join("bin/log")))
            .add(CreateDirectory::new(project_root.join("bin/obj")))
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

    fn get_project_name(arg_ret: &ArgRetriever) -> String {
        arg_ret
            .get_tag_args("--name")
            .map(|args| args[0].clone())
            .or_else(|| arg_ret.get_untagged().first().cloned())
            .unwrap_or_else(|| {
                env::current_dir()
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
            })
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

impl Action for InitProject {
    fn execute(&mut self) -> Result<()> {
        match self.state {
            InitProjectState::ValidArguments => self.action_chain.execute(),
            InitProjectState::InvalidLang => Err("Invalid lang argument".into()),
        }
        .map_err(|err| err.prepend("Failed to initialize project: "))
    }

    fn undo(&mut self) -> Result<()> {
        self.action_chain
            .undo()
            .map_err(|err| err.prepend("Failed to undo project initialization: "))
    }
}
