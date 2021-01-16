//! Official compiler implementation of Jingo built in rust

pub mod frontend;

use std::{fmt, path::PathBuf};

/// Specifies the current line/column anything is sat on, for use in an
/// upstream/downstream [Meta] structure
///
/// This structure is commonly utilised with the [fmt::Display] implementation
/// in order to show the current line and column numbers to a user
#[derive(Debug, Clone, PartialEq)]
pub struct MetaPos {
    /// Current line number
    pub line: usize,

    /// Current column number
    pub col: usize,
}

impl MetaPos {
    /// Creates a new [MetaPos] with appropriate blank values
    pub fn new() -> Self {
        Self { line: 1, col: 0 }
    }

    /// Resets for newline
    pub fn newline(&mut self, times: usize) {
        self.line += times;
        self.col = 0;
    }
}

impl fmt::Display for MetaPos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

/// Metadata container for representing the state of compilation at any given
/// moment, typically used for standardized error reporting
///
/// For each stage of compilation, this should be directly handed over as a mutable
/// which should be cloned from a "blank" [Meta], as no stage resets items such
/// as [Meta::line]
#[derive(Debug, Clone, PartialEq)]
pub struct Meta {
    /// Path that is currently being parsed
    pub path: Option<PathBuf>,

    /// Current position number
    pub pos: MetaPos,
}

impl Meta {
    /// Creates a new [Meta] with appropriate blank values
    pub fn new(path: impl Into<Option<PathBuf>>) -> Self {
        Self {
            path: path.into(),
            pos: MetaPos::new(),
        }
    }

    /// Returns a standardized compilation error
    pub fn error(&self, err: impl fmt::Display) -> String {
        match &self.path {
            Some(path) => format!(
                "Error {}:{}\n  {}",
                std::env::current_dir().unwrap().join(path).display(),
                self.pos,
                err
            ),
            None => format!("Error in unknown file {}\n  {}", self.pos, err),
        }
    }

    /// Resets [Meta::pos] for newline
    pub fn newline(&mut self, times: usize) {
        self.pos.newline(times)
    }
}

impl From<Meta> for Option<PathBuf> {
    fn from(meta: Meta) -> Self {
        meta.path
    }
}

impl From<Meta> for MetaPos {
    fn from(meta: Meta) -> Self {
        meta.pos
    }
}
