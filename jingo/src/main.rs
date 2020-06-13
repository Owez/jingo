//! The official CLI for Jingo (code for the `./jingo [file]` program), if you
//! are looking for the main documentation, [go here](https://github.com/scOwez/jingo)

mod cli;
mod log;

use cli::{parse_args, CLIResult};
use colored::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

/// Gets content of given path and handles errors in a user-friendly manner
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
        CLIResult::Direct(code, _) => {
            log::info("Compiling direct code..".to_string())

            // TODO use `code`
        }
        CLIResult::File(path, _) => {
            let file_name = path.file_name().unwrap().to_str().unwrap(); // thanks rust..
            log::info(format!("Compiling {}..", file_name.bold()));

            let code = read_path(path.clone(), file_name);

            // TODO use `code`
        }
        _ => (),
    }
}
