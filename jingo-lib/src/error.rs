//! Various enums and implementations of errors for downstream use
//!
//! Most error enums include an `Unknown` field, this is used for when there was
//! a fatal unknown error whilst doing something in the jurastiction of that error
//! enum, for example if there was an unknown error lexing then a
//! [ScanningError::Unknown] would show
//!
//! ## Error Hierarchy
//!
//! Here is a chart of what errors are also an instance of other errors (e.g.
//! the x error enum is inside of y error enum as `X(x)`):
//!
//! ```none
//! JingoError
//!     ScanningError
//!     ParsingError
//! ```

/// Main error enum for all of jingo-lib, containing mostly module-level error
/// enums
///
/// The goal of this enum is to provide a single overall representation for all
/// errors in this compiler to make much easier downstream compatibility whilst
/// still having seperate logical groups (like lexing with [ScanningError]) seperated
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum JingoError {
    /// Downstream error when scanning, see [ScanningError] for more infomation.
    ScanningError(ScanningError),

    /// Downstream error when parsing, see [ParsingError] for more infomation.
    ParsingError(ParsingError),

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

    /// A number was given that was not valid, possibly looking like `0-2-30`.
    InvalidNumber(usize),

    /// A float was given that was not valid, possibly looking like `0...3221.`.
    InvalidFloat(usize),

    /// An unknown token was given, user error (`usize` is line num, `char` is
    /// bad token).
    UnknownToken(usize, char),

    /// Unknown escape sequence (e.g. `\9` isn't an escape sequence like `\n`).
    UnknownEscape(usize, char),

    /// See [crate::error] documentation for more on this.
    Unknown,
}

/// Errors regarding the parsing flow inside of [crate::frontend::parser] (also
/// linked to [crate::frontend::ast]).
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ParsingError {
    /// See [crate::error] documentation for more on this.
    Unknown,
}
