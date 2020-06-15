//! Self-contained lexer for the Jingo compiler.

use crate::error::JingoError;

/// The token type, represents each scanned part of some Jingo.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {}

/// Primary entrypoint to lexeing, returns a [Vec]<[Token]> from given &[str] input.
pub fn scan_tokens(_code: &str) -> Result<Vec<Token>, JingoError> {
    Err(JingoError::Unimplemented(Some("scanner".to_string())))
}
