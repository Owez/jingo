//! Self-contained lexer for the Jingo compiler. See [Scanner::scan_code] for main
//! lexing capabilities.

use crate::error::JingoError;
use std::fmt;

/// The token type, represents the type of a [Token] after scanning.
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
    pub token_type: TokenType,

    /// The raw character(s) used to make this token (also known as a
    /// [Lexeme](https://en.wikipedia.org/wiki/Lexeme)).
    pub raw: String,

    /// Line number this token is found on.
    pub line: usize,
}

impl Token {
    /// Shortcut for adding a token, taking in the same parameters as normal but
    /// in a more consise manner.
    pub fn new(token_type: TokenType, raw: String, line: usize) -> Self {
        Self {
            token_type,
            raw,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: `{}`", self.token_type, self.raw)
    }
}

/// The main entrypoint into lexing, using primarily [Scanner::new] and
/// [Scanner::scan_code], you are able to fully tokenize (into [Vec]<[Token]>)
/// inputted Jingo.
pub struct Scanner {
    /// Code input as [String].
    ///
    /// *This will be empty until scanner is used once.*
    pub code: String,

    /// Outputted tokens, ususally from [Scanner::scan_code].
    ///
    /// *This will be empty until the scanner is used once, like [Scanner::code].*
    pub tokens: Vec<Token>,

    /// First char in the current lexeme scanned.
    start: usize,

    /// Character currently considering.
    current: usize,

    /// Current line.
    line: usize,
}

impl Scanner {
    /// Creates a new [Scanner] from code.
    pub fn new(code: String) -> Self {
        Self {
            code: code,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 0,
        }
    }

    /// Scans provided code stored in [Scanner::code] and saves tokens into [Scanner::tokens]
    pub fn scan_code(&mut self) -> Result<(), JingoError> {
        self.add_token(TokenType::Eof);

        Ok(())
    }

    /// Adds new token to [Scanner::tokens] from private metadata stored in [Scanner].
    fn add_token(&mut self, token_type: TokenType) {
        let raw: String = self
            .code
            .chars()
            .skip(self.start)
            .take(self.current)
            .collect();

        self.tokens.push(Token::new(token_type, raw, self.line));
    }
}
