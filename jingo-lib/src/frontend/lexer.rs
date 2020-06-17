//! Self-contained lexer for the Jingo compiler. See [scan_code] for main
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
    /// `=`
    Equal,
    /// `<`
    Less,
    /// `>`
    Greater,
    /// `!=`
    NotEqual,
    /// `==`
    EqualEqual,
    /// `>=`
    GreaterEqual,
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
fn scan_next_token(
    tokens: &mut Vec<Token>,
    chars: &mut std::iter::Peekable<std::str::Chars>,
    cur_line: &mut usize,
) -> Result<bool, JingoError> {
    let c = match chars.next() {
        Some(x) => x,
        None => return Ok(true),
    };

    let mut add_token = |token_type: TokenType, raw: String| {
        tokens.push(Token::new(token_type, raw, *cur_line));
    }; // shortcut to add token

    let mut peek_next = |next_char: char| -> bool {
        if chars.peek() == Some(&next_char) {
            chars.next();
            return true;
        }

        false
    }; // peeks for next item and if it exists, consumes it

    match c {
        '(' => add_token(TokenType::LeftBrak, c.to_string()),
        ')' => add_token(TokenType::RightBrack, c.to_string()),
        '{' => add_token(TokenType::LeftBrace, c.to_string()),
        '}' => add_token(TokenType::RightBrace, c.to_string()),
        ';' => add_token(TokenType::Semicolon, c.to_string()),
        '/' => add_token(TokenType::FSlash, c.to_string()),
        '*' => add_token(TokenType::Star, c.to_string()),
        '+' => add_token(TokenType::Plus, c.to_string()),
        '-' => {
            if peek_next('-') {
                if peek_next('-') {
                    add_token(TokenType::DocComment, "---".to_string())
                } else {
                    add_token(TokenType::Comment, "--".to_string())
                }
            } else {
                add_token(TokenType::Minus, c.to_string())
            }
        } // `-` for minus, `--` for comment or `---` for docstring
        '=' => {
            if peek_next('=') {
                add_token(TokenType::EqualEqual, "==".to_string())
            } else {
                add_token(TokenType::Equal, c.to_string())
            }
        } // `=` for equals, `==` for equal to
        '<' => {
            if peek_next('=') {
                add_token(TokenType::LessEqual, "<=".to_string())
            } else {
                add_token(TokenType::Less, c.to_string())
            }
        } // `<` for less than, `<=` for less than or equal to
        '>' => {
            if peek_next('=') {
                add_token(TokenType::GreaterEqual, ">=".to_string())
            } else {
                add_token(TokenType::Greater, c.to_string())
            }
        } // `>` for greater than, `>=` for greater than or equal to
        '!' => {
            if peek_next('=') {
                add_token(TokenType::NotEqual, "!=".to_string())
            } else {
                add_token(TokenType::Not, c.to_string())
            }
        } // `!` for not, `!=` for not equal
        '\n' => *cur_line += 1,  // add line
        '\r' | '\t' | ' ' => (), // ignore whitespace
        _ => {
            return Err(JingoError::Unimplemented(Some(format!(
                "token matching for `{}`",
                c
            ))))
        } // not implemented more
    }

    Ok(false)
}

/// Lexes code into [Vec]<[Token]> or provides an error in the form of [JingoError].
pub fn scan_code(code: &str) -> Result<Vec<Token>, JingoError> {
    let mut tokens = vec![]; // resulting tokens
    let mut cur_line = 1; // current line, appended as `\n` is found

    let mut chars = code.chars().peekable();

    loop {
        if scan_next_token(&mut tokens, &mut chars, &mut cur_line)? {
            break;
        }
    }

    Ok(tokens)
}
