//! CLI frotnend for the [Jingo](https://github.com/owez/jingo) language

#![deny(unsafe_code)]

mod file_pos;
mod lex;
mod utils;

use file_pos::FilePos;
use std::{env, process};

/// Help infomation
const HELP_INFO: &str = "Usage: jingo [OPTIONS]\n\nA lightweight, high-level language designed for rapid prototyping\n\nOptions:\n  run [FILE] — Compiles & runs a file\n  build [FILE] — Compiles a file\n  help — Shows this help\n\nAdvanced options:\n  lex [FILE] — Returns lexing stage only";

/// Command to run
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Compile,
    Run,
    Lex,
}

/// Parsed cli
#[derive(Debug, Clone, PartialEq)]
pub struct Parsed {
    /// The [Command] to run
    pub command: Command,

    /// Data passed for a [Parsed::command]
    pub data: Vec<String>,
}

impl Parsed {
    /// Creates new [Parsed] using default [env::args]
    fn new() -> Self {
        let mut env_args = env::args().into_iter();
        env_args.next(); // skip over first
        Parsed::custom(env_args.collect())
    }

    /// Parses custom arguments
    fn custom(args: Vec<String>) -> Self {
        // TODO: use [OsString]
        if args.len() == 0 {
            utils::help_exit("No arguments passed");
        }

        match args[0].as_str() {
            "help" | "--help" | "-h" => {
                println!("{}", HELP_INFO);
                process::exit(0)
            }
            "run" => Self {
                command: Command::Run,
                data: args[1..].to_vec(),
            },
            "compile" => Self {
                command: Command::Compile,
                data: args[1..].to_vec(),
            },
            "lex" => Self {
                command: Command::Lex,
                data: args[1..].to_vec(),
            },
            _ => utils::help_exit(format!("Command '{}' not recognised", args[0])),
        }
    }
}

fn main() {
    let parsed = Parsed::new();

    match parsed.command {
        Command::Lex => lex::launch(parsed),
        other => todo!("Finish ran '{:?}' command", other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_parse() {
        assert_eq!(
            Parsed::custom(vec!["lex".to_string(), "test".to_string()]),
            Parsed {
                command: Command::Lex,
                data: vec!["test".to_string()]
            }
        );
    }
}
