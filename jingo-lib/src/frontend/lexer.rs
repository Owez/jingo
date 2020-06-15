//! Self-contained lexer for the Jingo compiler.

/// The token type, represents each scanned part of some Jingo.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {}

/// Primary entrypoint to lexeing, returns a [Vec]<[Token]> from given &[str] input.
pub fn scan_tokens(code: &str) -> Vec<Token> {
    unimplemented!();
}
