//! Self-contained lexer for the Jingo compiler.

use crate::error::JingoError;
use std::fmt;

/// The token type, represents the type of token after scanning.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum TokenType {
    /// `(`
    LeftBrak,
    /// `)`
    RightBrack,
    /// `{`
    LeftBrace,
    /// `}`
    RightBrace,
    /// `,`
    Comma,
    /// `.`
    Dot,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `;`
    Semicolon,
    /// `/`
    FSlash,
    /// `*`
    Star,
    /// `!`
    Not,
    /// `!=`
    NotEqual,
    /// `=`
    Equal,
    /// `==`
    EqualEqual,
    /// `>`
    Greater,
    /// `>=`
    GreaterEqual,
    /// `<`
    Less,
    /// `<=`
    LessEqual,
    /// Identifier, some non-token set of chars, e.g. `hi` in `fn hi () {}`
    Identifier,
    /// A string literal, e.g. `"x"`
    StringLit,
    /// A number literal, e.g. `5`
    NumLit,
    /// `and`
    And,
    /// `class`
    Class,
    /// `else`
    Else,
    /// `false`
    False,
    /// `fn`
    Func,
    /// `for`
    For,
    /// `if`
    If,
    /// `null`
    Null,
    /// `or`
    Or,
    /// `print`, **NOTE: may be changed to stdlib func**
    Print,
    /// `return`
    Return,
    /// `super`
    Super,
    /// `this`
    This,
    /// `true`
    True,
    /// `var`
    Var,
    /// `while`
    While,
    /// End-of-file
    Eof,
}

/// Main token representation, combining [TokenType] with metadata.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Token {
    /// Type of token an instance of this stuct is referring to.
    token_type: TokenType,

    /// The raw character(s) used to make this token (also known as a
    /// [Lexeme](https://en.wikipedia.org/wiki/Lexeme)).
    raw: String,

    /// Line number this token is found on.
    line: u32,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: `{}`", self.token_type, self.raw)
    }
}

/// Primary entrypoint to lexeing, returns a [Vec]<[Token]> from given &[str] input.
pub fn scan_tokens(_code: &str) -> Result<Vec<Token>, JingoError> {
    Err(JingoError::Unimplemented(Some("scanner".to_string())))
}
