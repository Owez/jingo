//! Self-contained argument parser and result enums for easy downstream interaction

use crate::log;
use colored::*;
use std::env;
use std::option_env;
use std::path::PathBuf;

/// The possible outcomes of user-to-cli interaction
pub enum CLIResult {
    /// User gave a file to open, along with an optional output `-o` file
    File(PathBuf, Option<PathBuf>),

    /// A [String] of Jingo to parse directly without opening a file, along with
    /// an optional output `-o` file
    Direct(String, Option<PathBuf>),

    /// An error occured. The attached [String] is user-friendly info regarding
    /// what happened
    Error(String),

    /// Showed help or version message and returned, no need for further parsing.
    Handled,
}

/// Prints help info
fn show_help() -> CLIResult {
    let help_msg = "A lightweight, high-level language designed to be sleek and robust.

Usage:
    ./jingo <file>
    ./jingo <file> (-o | --output) <output>
    ./jingo (-i | --input) <code>
    ./jingo -h | --help
    ./jingo -v | --version

Options:
    -h --help       Show this screen.
    -v --version    Shows compiler version.
    -i --input      Feed direct Zypo code into compiler.
    -o --output     Output path for binary.";

    println!("{}", help_msg);

    CLIResult::Handled
}

/// Shows current version from the `Cargo.toml` file of the jingo cli crate (the
/// one you're in right now)
fn show_version() -> CLIResult {
    match option_env!("CARGO_PKG_VERSION") {
        Some(version) => {
            log::info(format!("Jingo compiler is at version {}", version.bold()));
            CLIResult::Handled
        }
        None => CLIResult::Error(format!(
            "Could not fetch version, please ensure {} is set",
            "CARGO_PKG_VERSION".bold()
        )),
    }
}

/// Parses arguments given in by user and returns a [CLIResult]
pub fn parse_args() -> CLIResult {
    if env::args().len() == 1 {
        return show_help();
    }

    let mut direct_buf = String::new(); // buffer for direct
    let mut output_buf = String::new(); // buffer for output
    let mut file_buf = String::new(); // buffer for file

    let mut direct_flag = false; // If flag was given for a [CLIResult::Direct]
    let mut output_flag = false; // If flag was given for -o/--output

    for (ind, argument) in env::args().enumerate() {
        if ind == 0 {
            continue;
        } else if ind == 1 && !argument.starts_with('-') {
            file_buf = argument;
        } else if &argument == "-h" || &argument == "--help" {
            return show_help();
        } else if &argument == "-v" || &argument == "--version" {
            return show_version();
        } else if &argument == "-o" || &argument == "--output" {
            output_flag = true;
        } else if &argument == "-d" || &argument == "--direct" {
            direct_flag = true;
        } else if direct_flag {
            direct_flag = false;
            direct_buf = argument;
        } else if output_flag {
            output_flag = false;
            output_buf = argument;
        }
    }

    let final_output = if output_buf.is_empty() {
        None
    } else {
        Some(PathBuf::from(output_buf))
    };

    if !direct_buf.is_empty() {
        CLIResult::Direct(direct_buf, final_output)
    } else if !file_buf.is_empty() {
        CLIResult::File(PathBuf::from(file_buf), final_output)
    } else {
        CLIResult::Error(format!(
            "Please provide a {} or a {}!",
            "<file>".bold(),
            "(-i | --input) <code>".bold()
        ))
    }
}
