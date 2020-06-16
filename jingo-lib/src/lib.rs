//! **You may be searching for [the repository](https://github.com/scOwez/jingo),
//! you are currently in the backend code for Jingo.**
//! 
//! ---
//! 
//! The central library for Jingo, containing the core of the compiler.
//!
//! This library is designed to be used downstream for the official CLI or any
//! future language servers/other tooling utilising the compiler without wanting
//! the added bulk of CLI dependencies.
//!
//! ## Usage
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! jingo-lib = "0.1"
//! ```
//!
//! ## Developer Notes
//!
//! - All frontend (e.g. lexing, parsing, ast) are contained in the [frontend]
//! module and all backend parts (e.g. codegen) are contained in [backend]
//! if you need to interact with a specific part of the compiler.

pub mod backend;
pub mod error;
pub mod frontend;

use error::*;
use std::path::PathBuf;

/// Compiles code to the best of the compilers (current) ability, e.g. lexing.
pub fn compile(code: String, _output: Option<PathBuf>) -> Result<(), JingoError> {
    let mut scanner = frontend::lexer::Scanner::new();

    scanner.scan_code(code)?;

    for token in scanner.tokens {
        println!("{}", token);
    }

    Ok(())
}
