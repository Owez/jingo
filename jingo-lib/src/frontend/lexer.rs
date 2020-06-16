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
    /// `--`, normal eol comment
    Comment,
    /// `---`, documentation comment, like
    /// [rusts](https://doc.rust-lang.org/rust-by-example/meta/doc.html#doc-comments)
    DocComment,
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
    /// Outputted tokens, ususally from [Scanner::scan_code].
    ///
    /// *This will be empty until the scanner is used once.*
    pub tokens: Vec<Token>,

    /// Code input represented as Vec<chars> for easier usage within [Scanner]'s
    /// contents.
    code: Vec<char>,

    /// First char in the current lexeme scanned.
    start: usize,

    /// Character currently considering.
    current: usize,

    /// Current line.
    line: usize,
}

impl Scanner {
    /// Creates a new [Scanner] from code.
    pub fn new() -> Self {
        Self {
            code: vec![],
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    /// Scans provided code and saves lexed tokens into [Scanner::tokens].
    pub fn scan_code(&mut self, code: String) -> Result<(), JingoError> {
        self.setup_scanner(code);

        // NOTE: should be moved into a continuous scanner
        self.scan_token()?;

        // Add custom empty EOF token
        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), self.line));

        Ok(())
    }

    /// Scans next token in [Scanner::code].
    fn scan_token(&mut self) -> Result<(), JingoError> {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftBrak),
            ')' => self.add_token(TokenType::RightBrack),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '+' => self.add_token(TokenType::Plus),
            '-' => self.add_token(TokenType::Minus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            _ => return Err(JingoError::Unimplemented(Some("token match".to_string()))),
        }

        Ok(())
    }

    /// Sets [Scanner::start], [Scanner::current] and [Scanner::line] offsets
    /// back to original `0, 0, 1` and sets [Scanner::code] to given `code`.
    fn setup_scanner(&mut self, code: String) {
        self.code = code.chars().collect();

        self.start = 0;
        self.current = 0;
        self.line = 0;
    }

    /// Advances through sourcecode onto the next char and returns.
    fn advance(&mut self) -> char {
        self.current += 1;

        self.code[self.current - 1]
    }

    /// Adds new token to [Scanner::tokens] from private metadata stored in [Scanner].
    fn add_token(&mut self, token_type: TokenType) {
        let raw: String = self
            .code
            .iter()
            .skip(self.start)
            .take(self.current)
            .collect();

        self.tokens.push(Token::new(token_type, raw, self.line));
    }
}
