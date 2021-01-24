//! File positioning, see [FilePos] docs for more info

use std::{fmt, path::PathBuf};

/// Represents a specific position in a file with line and column numbers taken
/// from a range and an optional file for [fmt::Display] usage
pub struct FilePos {
    pub path: Option<PathBuf>,
    pub line: usize,
    pub col: usize,
}

impl FilePos {
    /// Gets a new [FilePos] from given input, the ind to find and the filepath
    /// to display. If this returns [None], the file ended before expected
    pub fn new(path: impl Into<Option<PathBuf>>, input: &str, ind: usize) -> Option<Self> {
        let mut line: usize = 1;
        let mut col: usize = 1;

        for (input_ind, c) in input.chars().enumerate() {
            if input_ind == ind {
                return Some(Self {
                    path: path.into(),
                    line,
                    col,
                });
            } else if c == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }

        None
    }
}

impl fmt::Display for FilePos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.path {
            Some(path) => write!(
                f,
                "{}:{}:{}",
                std::env::current_dir().unwrap().join(path).display(),
                self.line,
                self.col
            ),
            None => write!(f, "unknown file {}:{}", self.line, self.col),
        }
    }
}
