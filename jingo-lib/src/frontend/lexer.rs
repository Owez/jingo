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

/// Scans single token, [usize] it provides is how many characters to skip over
/// in a parent loop that calls this everytime after a lookahead was used.
fn scan_token(tokens: &mut Vec<Token>, c: char, cur_line: &mut usize) -> Result<usize, JingoError> {
    let mut add_token = |token_type: TokenType, raw: String| {
        tokens.push(Token::new(token_type, raw, *cur_line));
    }; // shortcut to add token

    match c {
        '(' => add_token(TokenType::LeftBrak, c.to_string()),
        ')' => add_token(TokenType::RightBrack, c.to_string()),
        '{' => add_token(TokenType::LeftBrace, c.to_string()),
        '}' => add_token(TokenType::RightBrace, c.to_string()),
        '+' => add_token(TokenType::Plus, c.to_string()),
        '-' => add_token(TokenType::Minus, c.to_string()),
        ';' => add_token(TokenType::Minus, c.to_string()),
        '*' => add_token(TokenType::Minus, c.to_string()),
        '\n' => *cur_line += 1,
        _ => {
            return Err(JingoError::Unimplemented(Some(
                "token matching".to_string(),
            )))
        }
    }

    Ok(0)
}

/// Lexes code into [Vec]<Token> or provides an error in the form of [JingoError].
pub fn scan_code(code: &str) -> Result<Vec<Token>, JingoError> {
    let mut tokens = vec![]; // resulting tokens
    let mut cur_line = 1; // current line, appended as `\n` is found
    let mut skip_next_iters = 0; // how many times to skip

    for c in code.chars().peekable() {
        if skip_next_iters > 0 {
            skip_next_iters -= 1;
            continue;
        }

        skip_next_iters = scan_token(&mut tokens, c, &mut cur_line)?;
    }

    Ok(tokens)
}
