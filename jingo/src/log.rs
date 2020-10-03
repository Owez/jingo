//! Contains user-facing printing tools for errors and general compiler logs.

#![allow(dead_code)] // sparsely-used errors

use colored::*;
use std::process;

/// Displays a red error message for anything. If you want one with a line
/// number, see [error_line].
pub fn error<T: Into<String>>(message: T) {
    eprintln!("{} {}", "Error:".red(), message.into());
}

/// Displays a red fatal error message for anything then exists with code `1`.
/// If you want one with a line number, see [error_line].
pub fn fatal<T: Into<String>>(message: T) -> ! {
    eprintln!("{} {}", "Fatal:".red(), message.into());
    process::exit(1)
}

/// Displays a red error message for a specific line.
pub fn error_line<T: Into<String>>(line: i32, message: T) {
    let error_header = format!("Error [line {}]:", line);

    eprintln!("{} {}", error_header.red(), message.into());
}

/// Displays a simple blue info message.
pub fn info<T: Into<String>>(message: T) {
    println!("{} {}", "Info:".blue(), message.into());
}

/// Displays a success message in green.
pub fn success<T: Into<String>>(message: T) {
    println!("{} {}", "Success:".green(), message.into());
}

/// Displays a yellow warning message, indicating that something shouldn't happen
/// but program should compile successfully nontheless (minor error).
pub fn warn<T: Into<String>>(message: T) {
    eprintln!("{} {}", "Warning:".yellow(), message.into());
}
