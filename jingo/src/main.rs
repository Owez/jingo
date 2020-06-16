//! **You may be searching for [the repository](https://github.com/scOwez/jingo),
//! you are currently in the CLI code for Jingo.**
//! 
//! ---
//! 
//! This module is not available on crates.io or any other rust repositories, it
//! is just meant to be a self-contained CLI that is build directly from a `git
//! clone` of [the repository](https://github.com/scOwez/jingo/).

mod cli;
mod log;

use cli::{parse_args, CLIResult};
use colored::*;
use jingo_lib::compile;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

/// Wraps around the [jingo_lib::run] function and displays any panics in userland.
fn run_compiler(code: &str, output: Option<PathBuf>) {
    match compile(code, output) {
        Ok(_) => log::success("Compiler finished successfully".to_string()),
        Err(e) => log::fatal(e.to_string()),
    };
}

/// Gets content of given path and handles errors in a user-friendly manner.
fn read_path(path: PathBuf, file_name: &str) -> String {
    if !path.exists() {
        log::fatal(format!("The file {} does not exist", file_name.bold()))
    }

    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => log::fatal(format!(
            "Could not open {}, check permissions",
            file_name.bold()
        )),
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(_) => log::fatal(format!(
            "Could not read {}, check file formatting",
            file_name.bold()
        )),
    };

    contents
}

fn main() {
    match parse_args() {
        CLIResult::Fatal(e) => log::fatal(e),
        CLIResult::Direct(code_string, output) => {
            log::info("Compiling direct code..".to_string());

            run_compiler(&code_string, output);
        }
        CLIResult::File(path, output) => {
            let file_name = path.file_name().unwrap().to_str().unwrap(); // thanks rust..
            log::info(format!("Compiling {}..", file_name.bold()));

            let code_string = read_path(path.clone(), file_name);

            run_compiler(&code_string, output);
        }
        _ => (),
    }
}
