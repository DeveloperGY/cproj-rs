use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Lang {
    C,
    Cpp,
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Lang::C => write!(f, "c"),
            Lang::Cpp => write!(f, "cpp"),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub name: String,
    pub cc: String,
    pub include: Vec<String>,
    pub lib: Vec<String>,
    pub link_flags: Vec<String>,
    pub debug_flags: Vec<String>,
    pub release_flags: Vec<String>,
    pub lang: Lang,
}

impl Config {
    pub fn new(name: &str, lang: Lang) -> Self {
        let cc = match lang {
            Lang::C => "gcc",
            Lang::Cpp => "g++",
        };

        Self {
            name: name.to_string(),
            cc: cc.to_string(),
            include: vec!["include".to_string()],
            lib: vec!["lib".to_string()],
            link_flags: vec![],
            debug_flags: vec!["-Wall".to_string(), "-Wextra".to_string()],
            release_flags: vec![
                "-Wall".to_string(),
                "-Wextra".to_string(),
                "-O2".to_string(),
            ],
            lang,
        }
    }
}
