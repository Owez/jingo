//! Contains user-facing printing tools for errors and general compiler logs

use colored::*;
use jingo_lib::error::{JingoError, ParsingError, ScanningError};
use std::process;

/// Displays a red fatal error message for anything then exists with code `1`.
/// If you want one with a line number, see [error_line]
pub fn fatal(message: String) -> ! {
    eprintln!("{} {}", "Fatal:".red(), message);
    process::exit(1)
}

/// Displays a simple blue info message
pub fn info(message: String) {
    println!("{} {}", "Info:".blue(), message);
}

/// Displays a success message in green
pub fn success(message: String) {
    println!("{} {}", "Success:".green(), message);
}

/// Displays a yellow warning message, indicating that something shouldn't happen
/// but program should compile successfully nontheless (minor error)
pub fn warn(message: String) {
    eprintln!("{} {}", "Warning:".yellow(), message);
}

/// Downstream helping fun for [get_err_msg], makes error messages for lexing
/// errors
fn get_scan_error_msg(error: ScanningError) -> String {
    match error {
        ScanningError::UnterminatedString(line) => format!(
            "A string starting on line {} was opened but never closed (unterminated string)",
            line
        ),
        ScanningError::InvalidNumber(line) => {
            format!("Invalid number found on line {} (bad int)", line)
        }
        ScanningError::InvalidFloat(line) => {
            format!("Invalid float found on line {} (bad float)", line)
        }
        ScanningError::UnknownToken(line, c) => {
            format!("Unknown token '{}' found on line {} ", c, line)
        }
        ScanningError::UnknownEscape(line, c) => {
            format!("Unknown escape sequence '\\{}' found on line {}", c, line)
        }
        ScanningError::Unknown => String::from("Unknown error whilst scanning"),
    }
}

/// Downstream helping fun for [get_err_msg], makes error messages for parsing
/// errors
fn get_parse_error_msg(error: ParsingError) -> String {
    match error {
        ParsingError::Unknown => String::from("Unknown error whilst parsing"),
    }
}

/// Makes a fully formed userland error message from a given [JingoError]
pub fn get_err_msg(error: JingoError) -> String {
    match error {
        JingoError::ScanningError(e) => get_scan_error_msg(e),
        JingoError::ParsingError(e) => get_parse_error_msg(e),
        JingoError::Unimplemented(info) => match info {
            Some(x) => format!(
                "A part of the compiler accessed has not yet been made ({})",
                x
            ),
            None => String::from("A part of the compiler accessed has not yet been made"),
        },
        JingoError::Unknown => String::from("General unknown error"),
    }
}
