//! Various enums and implementations of errors for downstream use.
//!
//! Most error enums include an `Unknown` field, this is used for when there was
//! a fatal unknown error whilst doing something in the jurastiction of that error
//! enum, for example if there was an unknown error lexing then a
//! [ScanningError::Unknown] would show.
//!
//! ## Error Hierarchy
//!
//! Here is a chart of what errors are also an instance of other errors (e.g.
//! the x error enum is inside of y error enum as `X(x)`):
//!
//! ```none
//! JingoError (public)
//!     ScanningError (public)
//! ```

use std::fmt;

/// Main error enum for all of jingo-lib, containing mostly module-level error
/// enums.
///
/// The goal of this enum is to provide a single overall representation for all
/// errors in this compiler along with a guarantee that any error can easily be
/// represented as a concise string with [fmt::Display].
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum JingoError {
    /// Downstream error when scanning, see [ScanningError] for more infomation.
    ScanningError(ScanningError),

    /// A part of the compiler is unfinished that the user tried to access with
    /// some extra info in the form of an optional [String] appended onto the end
    /// in brackets.
    ///
    /// **This shouldn't ever happen in public releases without docs saying it
    /// will happen!**
    Unimplemented(Option<String>),

    /// See [crate::error] documentation for more on this.
    Unknown,
}

impl fmt::Display for JingoError {
    /// String representation of any compiler error.
    ///
    /// Each lower-level enum contained within [JingoError] should provide it's own
    /// string representation for this enum to hook onto if it is not included in
    /// the `match self` statement.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JingoError::Unknown => write!(f, "General unknown error"),
            JingoError::Unimplemented(info) => match info {
                Some(x) => write!(
                    f,
                    "A part of the compiler accessed has not yet been made ({})",
                    x
                ),
                None => write!(f, "A part of the compiler accessed has not yet been made"),
            },
            JingoError::ScanningError(e) => write!(f, "{}", e), // other errors
        }
    }
}

/// Errors for the [crate::frontend::lexer] module.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ScanningError {
    /// When a string was started but jingo reached the end of the file without
    /// it being closed.
    ///
    /// # Examples
    ///
    /// ```jingo
    /// fn awesome(input) {
    ///     return input * 2;
    /// }
    ///
    /// var x = "following string never closes
    ///
    /// fn broken_func(fix_string) {
    ///     print fix_string;
    /// }
    /// ```
    UnterminatedString(usize),

    /// See [crate::error] documentation for more on this.
    Unknown,
}

impl fmt::Display for ScanningError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScanningError::UnterminatedString(line) => write!(
                f,
                "A string starting on line {} was opened but never closed (unterminated string)",
                line
            ),
            ScanningError::Unknown => write!(f, "Unknown error whilst scanning"),
        }
    }
}
