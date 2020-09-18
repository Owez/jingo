//! Self-contained argument parser and result enums for easy downstream interaction.

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
        &['o'],
        &["output"],
        Some("Output path for binary"),
        DataType::Text,
    )
    .unwrap();
    let direct_arg = Argument::new(
        &['d'],
        &["direct"],
        Some("Feed direct Zypo code into compiler"),
        DataType::File,
    )
    .unwrap();

    let args = &[output_arg.clone(), direct_arg.clone()];

    let cli = CLIMake::new(
        args,
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
