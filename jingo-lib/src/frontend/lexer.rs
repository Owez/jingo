//! Self-contained lexer for the Jingo compiler. See [scan_code] for main
//! lexing capabilities.

use crate::error::{JingoError, ScanningError};
use std::fmt;

/// The token type, represents the type of a [Token] after scanning.
///
/// All token types are guaranteed to be displayed using [fmt::Display] in a
/// human-readable fashion, please reference that if you would like to see what
/// each type looks like in chars (it is found in `Trait Implementations`).
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum TokenType {
    LeftBrak,
    RightBrack,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Plus,
    Minus,
    Semicolon,
    FSlash,
    Star,
    Not,
    Equal,
    Less,
    Greater,
    NotEqual,
    EqualEqual,
    GreaterEqual,
    LessEqual,
    Comment,
    DocComment(String),    // TODO: do string interning
    HeaderComment(String), // TODO: do string interning
    Identifier(String),    // TODO: do string interning
    StringLit(String),     // TODO: do string interning
    NumLit(i64),
    FloatLit(f64),
    And,
    Class,
    Else,
    False,
    Func,
    For,
    If,
    Null,
    Or,
    Print, // TODO: change to stdlib func
    Return,
    Super,
    This,
    True,
    Var,
    While,
    /// Special eof used for end of scan runs
    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_rep = match self {
            TokenType::LeftBrak => '('.to_string(),
            TokenType::RightBrack => ')'.to_string(),
            TokenType::LeftBrace => '{'.to_string(),
            TokenType::RightBrace => '}'.to_string(),
            TokenType::Comma => ','.to_string(),
            TokenType::Dot => '.'.to_string(),
            TokenType::Plus => '+'.to_string(),
            TokenType::Minus => '-'.to_string(),
            TokenType::Semicolon => ';'.to_string(),
            TokenType::FSlash => '/'.to_string(),
            TokenType::Star => '*'.to_string(),
            TokenType::Not => '!'.to_string(),
            TokenType::Equal => '='.to_string(),
            TokenType::Less => '<'.to_string(),
            TokenType::Greater => '>'.to_string(),
            TokenType::NotEqual => "!=".to_string(),
            TokenType::EqualEqual => "==".to_string(),
            TokenType::GreaterEqual => ">=".to_string(),
            TokenType::LessEqual => "<=".to_string(),
            TokenType::Comment => "--".to_string(),
            TokenType::DocComment(content) => format!("---{}", content),
            TokenType::HeaderComment(content) => format!("-!-{}", content),
            TokenType::Identifier(content) => content.clone(),
            TokenType::StringLit(content) => format!("\"{}\"", content),
            TokenType::NumLit(number) => number.to_string(),
            TokenType::FloatLit(float) => float.to_string(),
            TokenType::And => "and".to_string(),
            TokenType::Class => "class".to_string(),
            TokenType::Else => "else".to_string(),
            TokenType::False => "false".to_string(),
            TokenType::Func => "fun".to_string(),
            TokenType::For => "for".to_string(),
            TokenType::If => "if".to_string(),
            TokenType::Null => "null".to_string(),
            TokenType::Or => "or".to_string(),
            TokenType::Print => "print".to_string(),
            TokenType::Return => "return".to_string(),
            TokenType::Super => "super".to_string(),
            TokenType::This => "this".to_string(),
            TokenType::True => "true".to_string(),
            TokenType::Var => "var".to_string(),
            TokenType::While => "while".to_string(),
            TokenType::Eof => "[eof]".to_string(),
        };

        write!(f, "{}", str_rep)
    }
}

/// Main token representation, combining [TokenType] with metadata.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Token {
    /// Type of token an instance of this stuct is referring to.
    pub token_type: TokenType,

    /// Line number this token is found on.
    pub line: usize,
}

impl Token {
    /// Shortcut for adding a token, taking in the same parameters as normal but
    /// in a more consise manner.
    pub fn new(token_type: TokenType, line: usize) -> Self {
        Self { token_type, line }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line {}: {}", self.line, self.token_type)
    }
}

/// Checks if char is a digit (0-9).
fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

/// Similar to [is_digit] but checks if the character is an underscore or in the
/// alphabet.
fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

/// Matches keywords like `and` to [TokenType::And] for easy keyword recognition
/// outside of this lexer.
pub fn keyword_match(content: &str) -> Option<TokenType> {
    match content {
        "and" => Some(TokenType::And),
        "class" => Some(TokenType::Class),
        "else" => Some(TokenType::Else),
        "false" => Some(TokenType::False),
        "fun" => Some(TokenType::Func),
        "for" => Some(TokenType::For),
        "if" => Some(TokenType::If),
        "null" => Some(TokenType::Null),
        "return" => Some(TokenType::Return),
        "super" => Some(TokenType::Super),
        "this" => Some(TokenType::This),
        "true" => Some(TokenType::True),
        "var" => Some(TokenType::Var),
        "while" => Some(TokenType::While),
        _ => None,
    }
}

/// Combines [is_digit] and [is_alpha] to check if a char is a digit, in the
/// alphabet or is an `_`.
fn is_alphanumeric(c: char) -> bool {
    is_digit(c) || is_alpha(c)
}

/// Gets a string literal by peeking until next `"` is found or returns
/// [JingoError::ScanningError]([ScanningError::UnterminatedString]) if the string
/// was never closed.
fn get_strlit_data(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    cur_line: &mut usize,
) -> Result<Token, JingoError> {
    let start_line = cur_line.clone(); // line started on
    let mut content = String::new(); // string innards

    loop {
        match chars.next() {
            Some(c) => {
                if c == '"' {
                    return Ok(Token::new(TokenType::StringLit(content), start_line));
                } else if c == '\n' {
                    *cur_line += 1;
                }

                content.push(c);
            }
            None => {
                return Err(JingoError::ScanningError(
                    ScanningError::UnterminatedString(start_line),
                ))
            }
        }
    }
}

/// Similar to [get_strlit_data] but for number literals, mapping to
/// [TokenType::NumLit] or [TokenType::FloatLit] if it has a `.`.
fn get_numlit_data(
    start_digit: char,
    chars: &mut std::iter::Peekable<std::str::Chars>,
    cur_line: &usize,
) -> Result<Token, JingoError> {
    let mut is_float = false; // once a `.` is detected, becomes a float
    let mut content = start_digit.to_string();

    loop {
        let c = match chars.peek() {
            Some(x) => *x,
            None => break,
        };

        if c == '.' {
            is_float = true;
            content.push(c);
        } else if is_digit(c) {
            content.push(c);
        } else {
            break;
        }

        chars.next();
    }

    if is_float {
        match content.parse::<f64>() {
            Ok(f) => Ok(Token::new(TokenType::FloatLit(f), cur_line.clone())),
            Err(_) => Err(JingoError::ScanningError(ScanningError::InvalidFloat(
                cur_line.clone(),
            ))),
        }
    } else {
        match content.parse::<i64>() {
            Ok(f) => Ok(Token::new(TokenType::NumLit(f), cur_line.clone())),
            Err(_) => Err(JingoError::ScanningError(ScanningError::InvalidNumber(
                cur_line.clone(),
            ))),
        }
    }
}

/// Gets identifier or keyword token from given chars and a `start_char` (due to
/// how lexing works).
fn get_identifier_kw_data(
    start_char: char,
    chars: &mut std::iter::Peekable<std::str::Chars>,
    cur_line: &usize,
) -> Result<Token, JingoError> {
    let mut content = start_char.to_string();

    loop {
        let c = match chars.peek() {
            Some(x) => *x,
            None => break,
        };

        if is_alphanumeric(c) {
            content.push(c);
        } else {
            break;
        }

        chars.next();
    }

    // match to an existing keyword, if not found make it an identifier
    match keyword_match(&content[..]) {
        Some(x) => Ok(Token::new(x, cur_line.clone())),
        None => Ok(Token::new(TokenType::Identifier(content), cur_line.clone())),
    }
}

/// Moves all characters in an iterator to a [String] until it hits a `\n`. This
/// is useful for comments, especially doc comments.
fn get_comment_data(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut comment = String::new();

    loop {
        match chars.next() {
            Some(c) => {
                if c == '\n' {
                    break;
                } else {
                    comment.push(c);
                }
            }
            None => break,
        }
    }

    comment
}

/// Gets next token in line of peekable `chars` (as this lexer is lookahead, it
/// needs `chars.peekable()` enabled).
pub fn scan_next_token(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    cur_line: &mut usize,
) -> Result<Option<Token>, JingoError> {
    let c = match chars.next() {
        Some(x) => x,
        None => return Ok(Some(Token::new(TokenType::Eof, *cur_line))),
    };

    let add_token =
        |token_type: TokenType| -> Option<Token> { Some(Token::new(token_type, cur_line.clone())) }; // shortcut to add token, always returns Some(token)

    let mut peek_next = |next_char: char| -> bool {
        if chars.peek() == Some(&next_char) {
            chars.next();
            return true;
        }

        false
    }; // peeks for next item and if it exists, consumes it

    let token = match c {
        '(' => add_token(TokenType::LeftBrak),
        ')' => add_token(TokenType::RightBrack),
        '{' => add_token(TokenType::LeftBrace),
        '}' => add_token(TokenType::RightBrace),
        ',' => add_token(TokenType::Comma),
        '.' => add_token(TokenType::Dot),
        ';' => add_token(TokenType::Semicolon),
        '/' => add_token(TokenType::FSlash),
        '*' => add_token(TokenType::Star),
        '+' => add_token(TokenType::Plus),
        '-' => {
            let to_return;

            if peek_next('-') {
                if peek_next('-') {
                    let char_content = get_comment_data(chars);

                    to_return = Some(add_token(TokenType::DocComment(char_content)));
                } else {
                    get_comment_data(chars); // remove but dont save

                    to_return = Some(add_token(TokenType::Comment));
                }

                *cur_line += 1;
            } else if peek_next('!') && peek_next('-') {
                let char_content = get_comment_data(chars);

                to_return = Some(add_token(TokenType::HeaderComment(char_content)));

                *cur_line += 1;
            } else {
                to_return = Some(add_token(TokenType::Minus))
            }

            to_return.unwrap()
        } // `-` for minus, `--` for comment, `---` for docstring or `-!-` for header comment
        '=' => {
            if peek_next('=') {
                add_token(TokenType::EqualEqual)
            } else {
                add_token(TokenType::Equal)
            }
        } // `=` for equals, `==` for equal to
        '<' => {
            if peek_next('=') {
                add_token(TokenType::LessEqual)
            } else {
                add_token(TokenType::Less)
            }
        } // `<` for less than, `<=` for less than or equal to
        '>' => {
            if peek_next('=') {
                add_token(TokenType::GreaterEqual)
            } else {
                add_token(TokenType::Greater)
            }
        } // `>` for greater than, `>=` for greater than or equal to
        '!' => {
            if peek_next('=') {
                add_token(TokenType::NotEqual)
            } else {
                add_token(TokenType::Not)
            }
        } // `!` for not, `!=` for not equal
        '"' => Some(get_strlit_data(chars, cur_line)?), // string literal/constant
        '\n' => {
            *cur_line += 1;
            None
        } // add line
        '\r' | '\t' | ' ' => None,                      // ignore whitespace
        c => {
            if is_digit(c) {
                // number/float literal
                Some(get_numlit_data(c, chars, cur_line)?)
            } else if is_alpha(c) {
                Some(get_identifier_kw_data(c, chars, cur_line)?)
            } else {
                // unknown
                return Err(JingoError::ScanningError(ScanningError::UnknownToken(
                    *cur_line, c, // acceptible to `*`, final error
                )));
            }
        }
    };

    Ok(token)
}

/// Lexes code into [Vec]<[Token]> or provides an error in the form of [JingoError].
///
/// # Examples
///
/// ```rust
/// use jingo_lib::frontend::lexer::scan_code;
///
/// let input = ".../---/...";
///
/// // please note that jingo != morse code, just a lexer torture test,
/// // should output something like `Ok([dot, dot, dot, fslash, doccomment])`.
///
/// println!("{:?}", scan_code(input));
/// ```
pub fn scan_code(code: &str) -> Result<Vec<Token>, JingoError> {
    let mut tokens = vec![]; // resulting tokens
    let mut cur_line = 1; // current line, appended as `\n` is found

    let mut chars = code.chars().peekable();

    loop {
        let scan = scan_next_token(&mut chars, &mut cur_line)?;

        match scan {
            Some(token) => match token.token_type {
                TokenType::Eof => break, // eof indicator
                _ => tokens.push(token), // normal token
            },
            _ => (), // whitespace
        }
    }

    Ok(tokens)
}
