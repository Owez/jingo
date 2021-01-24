//! Utility functions and data structures

use crate::HELP_INFO;

use std::io::prelude::*;
use std::{fmt, fs::File, path::PathBuf, process};

/// Shows message then exits with code 1
pub fn msg_exit(msg: impl fmt::Display) -> ! {
    eprintln!("{}", msg);
    process::exit(1)
}

/// Shows error message then exits with code 1
pub fn error_exit(msg: impl fmt::Display) -> ! {
    msg_exit(format!("Error in cli\n  {}", msg));
}

/// Shows error help message then exits with code 1
pub fn help_exit(msg: impl fmt::Display) -> ! {
    eprintln!("{}\n", HELP_INFO);
    error_exit(msg)
}

/// Opens file or errors with frontend error
pub fn open_file(filepath: impl Into<PathBuf>) -> String {
    let filepath = filepath.into();

    if !filepath.is_file() {
        error_exit(format!("File {:?} doesn't exist", filepath))
    }

    let mut file = match File::open(filepath.clone()) {
        Ok(x) => x,
        Err(err) => error_exit(format!("Could not open {:?}, {}", filepath, err)),
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(err) => error_exit(format!("Could not read {:?}, {}", filepath, err)),
    };

    contents
}
