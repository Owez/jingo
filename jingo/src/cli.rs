//! Contains [climake]-based cli parsing for the jingo cli

use crate::log;
use climake::{Argument, CliMake, DataType, PassedData, crate_version};
use std::path::PathBuf;

/// Gets a single file from `Vec<PathBuf>` or returns an error for too little or
/// too many
fn file_from_files<T: Into<String>>(files: Vec<PathBuf>, err_msg: T) -> PathBuf {
    if files.len() != 1 {
        log::fatal(err_msg)
    }

    files[0].clone()
}

/// Infomation provided back from climake, formatted into as simple as possible
/// datatypes inside of this structure
pub struct CliData {
    /// Found code from file or plaintext to compile
    pub code: String,

    /// Output location
    pub output: PathBuf,

    /// If the input was given as plaintext, not a file (e.g. `-i hi` not `-f
    /// file.txt`)
    pub is_plaintext_input: bool,
}

impl CliData {
    fn new(input_type: Option<InputType>, output_path: Option<PathBuf>) -> Self {
        unimplemented!(); // TODO: create struct from raw inputs
    }
}

/// Used inside [launch_cli] to easily detect errors if one is already given
enum InputType {
    File(PathBuf),
    Text(String),
}

/// Creates cli and parses into a final [CliData] and provides any errors in
/// userland
pub fn launch_cli() -> CliData {
    let arg_output = Argument::new(
        &['o'],
        &["output"],
        Some("Output location"),
        DataType::Files,
    )
    .unwrap();
    let arg_file = Argument::new(
        &['f'],
        &["file"],
        Some("File to operate from"),
        DataType::Files,
    )
    .unwrap();
    let arg_input = Argument::new(
        &['i', 't'],
        &["input", "text"],
        Some("Plaintext to operate from"),
        DataType::Text,
    )
    .unwrap();

    let args = &[arg_output.clone(), arg_file.clone(), arg_input.clone()];
    let cli = CliMake::new(
        args,
        Some("A lightweight, high-level language designed to be sleek and robust"),
        Some(crate_version()),
    )
    .unwrap();

    let mut input_type: Option<InputType> = None;
    let mut output_path: Option<PathBuf> = None;

    for used_arg in cli.parse() {
        if used_arg.argument == arg_output {
            match used_arg.passed_data {
                PassedData::Files(f) => {
                    output_path = Some(file_from_files(f, "Please provide just 1 output path"))
                }
                _ => log::fatal("Please provide a path to output"),
            }
        } else if used_arg.argument == arg_file {
            match used_arg.passed_data {
                PassedData::Files(f) => match input_type {
                    Some(_) => {
                        log::fatal("Please provide a single input of either a file or plaintext")
                    }
                    None => {
                        input_type = Some(InputType::File(file_from_files(
                            f,
                            "Please provide just 1 input path",
                        )))
                    }
                },
                _ => log::fatal("Please provide a path to input"),
            }
        }
    }

    CliData::new(input_type, output_path)
}
