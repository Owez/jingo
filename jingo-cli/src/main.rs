//! CLI frotnend for the [Jingo](https://github.com/owez/jingo) language

use std::{env, fmt, path::PathBuf, process};

/// Help infomation
const HELP_INFO: &str = "Usage: jingo [OPTIONS]\n\nOptions:\n  run [FILE] — Compiles & runs a file\n  compile [FILE] — Compiles a file\n  (--help | -h) — Shows this help";

/// Command to run
enum Command {
    Compile,
    Run,
}

/// Shows error help message then exits with code 1
fn err_help(msg: impl fmt::Display) -> ! {
    eprintln!("Error: {}\n\n{}", msg, HELP_INFO);
    process::exit(1)
}

/// Parsed cli
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
            err_help("No arguments passed");
        }

        match args[0].as_str() {
            "--help" | "-h" => {
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
            _ => err_help(format!("Command '{}' not recognised", args[0])),
        }
    }
}

fn main() {
    let parsed = Parsed::new();
}
