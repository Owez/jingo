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

use cli::{parse_args, CLIResult, CLIStage};
use colored::*;
use jingo_lib::compile;
use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

/// Metadata structure to hand off info for downstream compilation tasks after
/// cleaning CLI results
struct CompileInfo {
    /// Code to compile
    code: String,
    /// Optional output path
    output: Option<PathBuf>,
    /// Stage to compile to
    stage: CLIStage,
}

impl CompileInfo {
    /// Matches [CompileInfo::stage] to a relevant compilation stage
    /// 
    /// Stages that may be used:
    /// 
    /// - [CompileInfo::run_full]
    /// - [CompileInfo::run_scanner]
    /// - [CompileInfo::run_parser]
    fn compile(&self) {
        match self.stage {
            CLIStage::Normal => self.run_full(),
            CLIStage::Scanner => self.run_scanner(),
            CLIStage::Parser => self.run_parser(),
        }
    }

    /// Wraps around the [jingo_lib::compile] function and displays any panics
    /// in userland. This is the "normal" run function compared to others that
    /// stop at a defined compilation stage
    fn run_full(&self) {
        match compile(&self.code, self.output.clone()) {
            // TODO: move compile() to lexer & replace with `unimplemented!()`
            Ok(_) => log::success("Compiler finished successfully".to_string()),
            Err(e) => log::fatal(e.to_string()),
        };
    }

    /// Compiles code to the lexer/scanner phase only, similar to [run_full] but more
    /// limited
    fn run_scanner(&self) {
        unimplemented!();
    }

    /// Compiles code to the parser phase only. Does not show stdout for lexing, only
    /// resulting AST (if parsing was successful)
    fn run_parser(&self) {
        unimplemented!();
    }
}

/// Gets content of given path and handles errors in a user-friendly manner.
fn read_path(path: PathBuf, file_name: &str) -> String {
    if !path.exists() {
        log::fatal(format!("The file {} does not exist", file_name.bold()))
    }

    let mut file = match File::open(path.clone()) {
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

    if path.extension() == Some(OsStr::new("jingo")) {
        log::warn(format!(
            "File {} is advised to use {} instead of the {} extension",
            file_name.bold(),
            ".jno".bold(),
            ".jingo".bold()
        ));
    }

    if contents.is_empty() {
        log::warn(format!(
            "File {} is empty so nothing will happen",
            file_name.bold()
        ))
    }

    contents
}

fn main() {
    let parsed_args = parse_args();

    match parsed_args.result {
        CLIResult::Fatal(e) => log::fatal(e),
        CLIResult::Direct(code, output) => {
            log::info("Compiling direct code..".to_string());

            if code.is_empty() {
                // should never happen due to cli's nature but safe to have anyway
                log::warn("No code given, nothing will happen".to_string());
            }

            CompileInfo {
                code: code,
                output: output,
                stage: parsed_args.stage,
            }.compile(); // TODO: tidy up
        }
        CLIResult::File(path, output) => {
            let file_name = path.file_name().unwrap().to_str().unwrap(); // thanks rust..
            log::info(format!("Compiling {}..", file_name.bold()));

            let code = read_path(path.clone(), file_name);

            CompileInfo {
                code: code,
                output: output,
                stage: parsed_args.stage,
            }.compile(); // TODO: tidy up
        }
        _ => (),
    }
}
