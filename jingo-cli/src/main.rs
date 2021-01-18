//! CLI frotnend for the [Jingo](https://github.com/owez/jingo) language

use jingo_lib::{frontend::scanner, meta::Meta};
use std::io::prelude::*;
use std::{env, fmt, fs::File, path::PathBuf, process};

/// Help infomation
const HELP_INFO: &str = "Usage: jingo [OPTIONS]\n\nA lightweight, high-level language designed for rapid prototyping\n\nOptions:\n  run [FILE] — Compiles & runs a file\n  compile [FILE] — Compiles a file\n  help — Shows this help\n\nAdvanced options:\n  scan [FILE] — Returns scanning stage only";

/// Command to run
#[derive(Debug, Clone, PartialEq)]
enum Command {
    Compile,
    Run,
    Scan,
}

/// Parsed cli
#[derive(Debug, Clone, PartialEq)]
struct Parsed {
    /// The [Command] to run
    command: Command,

    /// Data passed for a [Parsed::command]
    data: Vec<String>,
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
        if args.len() == 0 {
            help_exit("No arguments passed");
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
            "scan" => Self {
                command: Command::Scan,
                data: args[1..].to_vec(),
            },
            _ => help_exit(format!("Command '{}' not recognised", args[0])),
        }
    }
}

/// Shows message then exits with code 1
fn msg_exit(msg: impl fmt::Display) -> ! {
    eprintln!("{}", msg);
    process::exit(1)
}

/// Shows error message then exits with code 1
fn error_exit(msg: impl fmt::Display) -> ! {
    msg_exit(format!("Error in cli\n  {}", msg));
}

/// Shows error help message then exits with code 1
fn help_exit(msg: impl fmt::Display) -> ! {
    eprintln!("{}\n", HELP_INFO);
    error_exit(msg)
}

/// Opens file or errors with frontend error
fn open_file(filepath: impl Into<PathBuf>) -> String {
    let filepath = filepath.into();

    if !filepath.is_file() {
        error_exit(format!("File {:?} doesn't exist", filepath))
    }

    let mut file = match File::open(filepath.clone()) {
        Ok(x) => x,
        Err(err) => error_exit(format!("Could not open {:?}, {}", filepath, err)),
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(err) => error_exit(format!("Could not read {:?}, {}", filepath, err)),
    };

    contents
}

/// Runs [Command::Scan]
fn run_scan(parsed: Parsed) {
    if parsed.data.len() == 0 {
        help_exit("No files passed for scanning")
    } else if parsed.data.len() > 1 {
        help_exit("More then one file passed for scanning")
    }

    let filepath = PathBuf::from(parsed.data[0].clone());
    let scanned = scanner::launch(Meta::new(filepath.clone()), open_file(filepath));

    match scanned {
        Ok(output) => println!("Scanned output:\n{:#?}", output),
        Err((err, meta)) => msg_exit(meta.error(err, "scanning")),
    }
}

fn main() {
    let parsed = Parsed::new();

    match parsed.command {
        Command::Scan => run_scan(parsed),
        other => todo!("Finish ran '{:?}' command", other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_parse() {
        assert_eq!(
            Parsed::custom(vec!["scan".to_string(), "test".to_string()]),
            Parsed {
                command: Command::Scan,
                data: vec!["test".to_string()]
            }
        );
        assert_eq!(
            Parsed::custom(vec!["help".to_string()]),
            Parsed {
                command: Command::Scan,
                data: vec![]
            }
        )
    }
}
