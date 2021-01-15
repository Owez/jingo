//! Official compiler implementation of Jingo built in rust

pub mod frontend;

use std::{fmt, path::PathBuf};

/// Metadata container for representing the state of compilation at any given
/// moment, typically used for standardized error reporting
///
/// For each stage of compilation, this should be directly handed over as a mutable
/// which should be cloned from a "blank" [Meta], as no stage resets items such
/// as [Meta::line]
#[derive(Debug, Clone, PartialEq)]
pub struct Meta {
    /// File that is currently being parsed
    file: Option<PathBuf>,

    /// Line that the error occured on
    line: usize,

    /// Start column of the error
    col: usize,
}

impl Meta {
    /// Creates a new [Meta] with appropriate blank values
    pub fn new(file: impl Into<Option<PathBuf>>) -> Self {
        Self {
            file: file.into(),
            line: 1,
            col: 0,
        }
    }

    /// Returns a standardized compilation error
    pub fn error(&self, err: impl fmt::Display) -> String {
        match &self.file {
            Some(file) => format!(
                "Error {}:{}:{}:\n  {}",
                std::env::current_dir().unwrap().join(file).display(),
                self.line,
                self.col,
                err
            ),
            None => format!(
                "Error in unknown file {}:{}:\n  {}",
                self.line, self.col, err
            ),
        }
    }

    /// Resets for newline
    pub(crate) fn newline(&mut self) {
        self.line += 1;
        self.col = 0;
    }
}
