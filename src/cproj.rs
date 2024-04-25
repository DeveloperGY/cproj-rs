mod action;
mod actions;
mod arg_retriever;
mod config;
mod graph;

use std::env;

use action::Action;
use actions::*;

/// cproj new --name [name] --lang [c, cpp] (default = --lang c)
/// cproj init --name [name] --lang [c, cpp] (default = --lang c)
/// cproj run [--debug, --release] (default = --debug)
/// cproj build [--debug, --release] (default = --debug)
/// cproj clean
/// cproj help
///
fn main() {
    let (command_string, sub_command_strings) = get_command();
    let sub_commands: Vec<_> = sub_command_strings.iter().map(|val| val.as_str()).collect();

    if let Some(mut action) = validate_command(&command_string, &sub_commands) {
        if let Err(err) = action.execute() {
            eprintln!("Error: {}", err.get_msg());
            if let Err(err) = action.undo() {
                eprintln!("Error: {}", err.get_msg());
            }
        }
    } else {
        eprintln!("Invalid Command!");
    }
}

fn get_command() -> (String, Vec<String>) {
    let mut args: Vec<_> = env::args().collect();

    // This empty check is for the case in which the program name is not passed to the program as an
    // argument (https://doc.rust-lang.org/std/env/fn.args.html)
    if args.is_empty() {
        return ("help".to_string(), vec![]);
    }

    // using contains instead of == in case cproj is run via a relative path
    if args[0].contains("cproj") {
        args.remove(0);

        if args.is_empty() {
            return ("help".to_string(), vec![]);
        }
    }

    (args[0].clone(), args[1..].to_vec())
}

fn validate_command(command: &str, args: &[&str]) -> Option<Box<dyn Action>> {
    match command.to_lowercase().as_str() {
        "new" => Some(NewProject::new(args)),
        "init" => Some(InitProject::new(args)),
        "run" => Some(RunProject::new(args)),
        "build" => Some(BuildProject::new(args)),
        "clean" => Some(CleanProject::new()),
        "help" => Some(Help::new()),
        _ => None,
    }
}
