// //! Self-contained argument parser and result enums for easy downstream interaction.

// use crate::log;
// use colored::*;
// use std::env;
// use std::option_env;
// use std::path::PathBuf;

// /// Prints help info, `error` is for if it should display to stderr (true) or just stdout (false).
// fn show_help(error: bool) -> CLIResult {
//     let help_msg = "A lightweight, high-level language designed to be sleek and robust.

// Usage:
//     ./jingo <file>
//     ./jingo <file> (-o | --output) <output>
//     ./jingo (-d | --direct) <code>
//     ./jingo (-h | --help)
//     ./jingo (-v | --version)

// Options:
//     -h, --help       Show this screen.
//     -v, --version    Shows compiler version.
//     -d, --direct      Feed direct Zypo code into compiler.
//     -o, --output     Output path for binary.";

//     if error {
//         eprintln!("{}", help_msg);
//     } else {
//         println!("{}", help_msg);
//     }

//     CLIResult::Handled
// }

// /// Shows current version from the `Cargo.toml` file of the jingo cli crate (the
// /// one you're in right now).
// fn show_version() -> CLIResult {
//     match option_env!("CARGO_PKG_VERSION") {
//         Some(version) => {
//             log::info(format!("Jingo compiler is at version {}", version.bold()));
//             CLIResult::Handled
//         }
//         None => CLIResult::Fatal(format!(
//             "Could not fetch version, please ensure {} is set",
//             "CARGO_PKG_VERSION".bold()
//         )),
//     }
// }

// /// Parses arguments given in by user and returns a [CLIResult].
// pub fn parse_args() -> CLI {
//     if env::args().len() == 1 {
//         return CLI::norm(show_help(true));
//     }

//     let mut direct_buf = String::new(); // buffer for direct
//     let mut output_buf = String::new(); // buffer for output
//     let mut file_buf = String::new(); // buffer for file

//     let mut direct_arg = false; // If arg was given for a [CLIResult::Direct]
//     let mut output_arg = false; // If arg was given for -o/--output

//     let mut cli_stage = CLIStage::Normal; // stage of cli (defaults to [CLIStage::Normal])

//     for (ind, argument) in env::args().enumerate() {
//         if ind == 0 {
//             continue;
//         } else if ind == 1 && !argument.starts_with('-') {
//             file_buf = argument;
//         } else if &argument == "-h" || &argument == "--help" {
//             return CLI::norm(show_help(false));
//         } else if &argument == "-v" || &argument == "--version" {
//             return CLI::norm(show_version());
//         } else if &argument == "-o" || &argument == "--output" {
//             output_arg = true;
//         } else if &argument == "-d" || &argument == "--direct" {
//             direct_arg = true;
//         } else if direct_arg {
//             direct_arg = false;

//             if direct_buf.is_empty() {
//                 direct_buf = argument;
//             } else {
//                 return CLI::norm(CLIResult::Fatal(
//                     "Please only provide 1 direct argument".to_string(),
//                 ));
//             }
//         } else if output_arg {
//             output_arg = false;

//             if output_buf.is_empty() {
//                 output_buf = argument;
//             } else {
//                 return CLI::norm(CLIResult::Fatal(
//                     "Please only provide 1 output location".to_string(),
//                 ));
//             }
//         }
//     }

//     let final_output = if output_buf.is_empty() {
//         None
//     } else {
//         Some(PathBuf::from(output_buf))
//     };

//     let cli_result = if !direct_buf.is_empty() {
//         CLIResult::Direct(direct_buf, final_output)
//     } else if !file_buf.is_empty() {
//         CLIResult::File(PathBuf::from(file_buf), final_output)
//     } else {
//         CLIResult::Fatal(format!(
//             "Please provide a {} or a {}!",
//             "<file>".bold(),
//             "(-d | --direct) <code>".bold()
//         ))
//     };

//     CLI {
//         stage: cli_stage,
//         result: cli_result,
//     }
// }

use climake::{Argument, CLIMake, DataType, PassedData};
use std::{path::PathBuf, process};

/// The possible outcomes of user-to-cli interaction.
pub enum CLIResult {
    /// User gave a file to open, along with an optional output `-o` file.
    File(PathBuf, Option<PathBuf>),

    /// A [String] of Jingo to parse directly without opening a file, along with
    /// an optional output `-o` file.
    Direct(String, Option<PathBuf>),

    /// A fatal error occured. The attached [String] is user-friendly info regarding
    /// what happened.
    Fatal(String),

    /// Showed help or version message and returned, no need for further parsing..
    Handled,
}

/// The stage of compilation user would like (made into this enum by figuring out
/// flags automatically)
pub enum CLIStage {
    /// Standard, full compilation from beginning to end, as much as the compiler
    /// can offer without major time loss with over-optimisations
    Normal,

    /// Scanner/lexing only, printing each lex result to stdout
    Scanner,

    /// Hidden lexing process then stdout for all parsed AST
    Parser,
}

/// Encompasses [CLIStage] and [CLIResult] to be used in returns from CLI-related
/// tasks
pub struct CLI {
    /// Compilation stage requested
    pub stage: CLIStage,

    /// Result of overall cli, to be used in conjunction with [CLI::stage] if
    /// it's not an error
    pub result: CLIResult,
}

impl CLI {
    /// Shortcut to creating a CLI with a simple [CLIStage::Normal] stage
    pub fn norm(result: CLIResult) -> Self {
        Self {
            stage: CLIStage::Normal,
            result: result,
        }
    }
}

/// Gets properly-formatted version
fn crate_version() -> String {
    format!(
        "{}.{}.{}{}",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH"),
        option_env!("CARGO_PKG_VERSION_PRE").unwrap_or("")
    )
}

pub fn parse_args() -> CLI {
    let output_arg = Argument::new(
        vec!['o'],
        vec![String::from("output")],
        Some("Output path for binary"),
        DataType::Text,
    )
    .unwrap();
    let direct_arg = Argument::new(
        vec!['d'],
        vec![String::from("direct")],
        Some("Feed direct Zypo code into compiler"),
        DataType::File,
    )
    .unwrap();

    let cli = CLIMake::new(
        vec![output_arg.clone(), direct_arg.clone()],
        Some("A lightweight, high-level language"),
        Some(crate_version()),
    )
    .unwrap();
    let mut direct_code: Option<String> = None; // direct code given if any
    let mut output_path: Option<PathBuf> = None; // given output path if any

    for used_arg in cli.parse() {
        if used_arg.argument == output_arg {
            match used_arg.passed_data {
                PassedData::File(f) => output_path = Some(f[0].clone()),
                _ => {
                    eprintln!("Please provide a path to output if using (-o, --output)!");
                    process::exit(1);
                }
            }
        } else if used_arg.argument == direct_arg {
            match used_arg.passed_data {
                PassedData::Text(t) => {
                    if t.len() > 1 {
                        eprintln!("Please provide just 1 direct string to compile if using (-d, --direct)!");
                        process::exit(1)
                    } else {
                        direct_code = Some(t[0].clone())
                    }
                }
                PassedData::None => direct_code = Some(String::new()), // assume user wanted this
                _ => panic!("CLI library returned unexpected results, this shouldn't happen!"),
            }
        }
    }

    unimplemented!();
}
