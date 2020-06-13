//! Contains user-facing printing tools for errors and general compiler logs.

use colored::*;
use std::process;

/// Displays a red error message for anything. If you want one with a line
/// number, see [error_line].
pub fn error(message: String) {
    eprintln!("{} {}", "Error:".red(), message);
}

/// Displays a red fatal error message for anything then exists with code `1`.
/// If you want one with a line number, see [error_line].
pub fn fatal(message: String) -> ! {
    eprintln!("{} {}", "Fatal:".red(), message);
    process::exit(1)
}

/// Displays a red error message for a specific line
pub fn error_line(line: i32, message: String) {
    let error_header = format!("Error [line {}]:", line);

    eprintln!("{} {}", error_header.red(), message);
}

/// Displays a simple blue info message
pub fn info(message: String) {
    println!("{} {}", "Info:".blue(), message);
}
