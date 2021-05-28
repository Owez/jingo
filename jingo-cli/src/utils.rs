//! Utility functions and data structures

use crate::HELP_INFO;

use ansi_term::Style;
use std::io::prelude::*;
use std::{fs::File, path::PathBuf, process};

/// Shows message then exits with code 1
pub(crate) fn msg_exit(msg: impl AsRef<str>) -> ! {
    eprintln!("{}", msg.as_ref());
    process::exit(1)
}

/// Shows error message then exits with code 1
pub(crate) fn error_exit(msg: impl AsRef<str>) -> ! {
    msg_exit(format!(
        "Error in cli ↴\n  {}",
        Style::new().bold().paint(msg.as_ref())
    ));
}

/// Shows error help message then exits with code 1
pub(crate) fn help_exit(msg: impl AsRef<str>) -> ! {
    eprintln!("{}\n", HELP_INFO);
    error_exit(msg.as_ref())
}

/// Opens file or errors with frontend error
pub fn open_file(filepath: impl Into<PathBuf>) -> String {
    let filepath = filepath.into();

    if !filepath.is_file() {
        error_exit(format!("File {:?} doesn't exist", filepath))
    }

    let mut file = match File::open(filepath.clone()) {
        Ok(x) => x,
        Err(err) => error_exit(format!("Could not open {:?} file, {}", filepath, err)),
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(err) => error_exit(format!("Could not read {:?} file, {}", filepath, err)),
    };

    contents
}
