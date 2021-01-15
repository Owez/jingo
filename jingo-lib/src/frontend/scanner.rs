//! Scanner/lexer stage of parsing, the first main step to parse raw characters
//! into further parsable tokens

use std::{fmt, iter::Peekable};

/// Error enumeration representing errors whilst scanning; see the [fmt::Display]
/// trait impl for documentation on each case
#[derive(Debug, Clone, PartialEq)]
pub enum ScanError {
    TokenNotFound(String),
    UnexpectedEof,
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScanError::TokenNotFound(input) => {
                write!(f, "Input '{}' is not a known keyword or identifier", input)
            }
            ScanError::UnexpectedEof => {
                write!(f, "File ended abruptly whilst scanning, unexpected EOF")
            }
        }
    }
}

/// Token type
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // single-char
    ParenLeft,
    ParenRight,
    BraceLeft,
    BraceRight,
    Comma,
    Dot,
    Semicolon,
    FwdSlash,
    Star,

    // math-only symbols
    Plus,
    Minus,
    Equals,
    EqualsEquals,
    Exclaim,
    ExclaimEquals,
    Less,
    LessEquals,
    Greater,
    GreaterEquals,

    // keywords
    If,
    And,
    Or,
    Else,
    True,
    False,
    None,
    Class,
    For,
    While,
    Return,
    This,
    Var,

    // literals
    Id(String),
    Str(String), // this is unfiltered (e.g. otherwise invalid `\`)
    Char(char),
    Number(i64),
}

impl Token {
    /// Creates a new [Token] from input string, returning a [ScanError::TokenNotFound]
    /// if the token could not be found
    pub fn new<'a>(mut input: Peekable<impl Iterator<Item = &'a char>>) -> Result<Self, ScanError> {
        match input.next().ok_or(ScanError::UnexpectedEof)? {
            '(' => Ok(Token::ParenLeft),
            ')' => Ok(Token::ParenRight),
            '{' => Ok(Token::BraceLeft),
            '}' => Ok(Token::BraceRight),
            ',' => Ok(Token::Comma),
            '.' => Ok(Token::Dot),
            ';' => Ok(Token::Semicolon),
            '/' => Ok(Token::FwdSlash),
            '*' => Ok(Token::Star),
            '+' => Ok(Token::Plus),
            '-' => Ok(Token::Minus),
            '=' => {
                if input.peek().ok_or(ScanError::UnexpectedEof)? == &&'=' {
                    Ok(Token::EqualsEquals)
                } else {
                    Ok(Token::Equals)
                }
            }
            '!' => {
                if input.peek().ok_or(ScanError::UnexpectedEof)? == &&'=' {
                    Ok(Token::ExclaimEquals)
                } else {
                    Ok(Token::Exclaim)
                }
            }
            '<' => {
                if input.peek().ok_or(ScanError::UnexpectedEof)? == &&'=' {
                    Ok(Token::LessEquals)
                } else {
                    Ok(Token::Less)
                }
            }
            '>' => {
                if input.peek().ok_or(ScanError::UnexpectedEof)? == &&'=' {
                    Ok(Token::GreaterEquals)
                } else {
                    Ok(Token::Greater)
                }
            }
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eqeq() {
        assert_eq!(
            Token::new(['=', '='].iter().peekable()).unwrap(),
            Token::EqualsEquals
        )
    }

    #[test]
    fn neeq() {
        assert_eq!(
            Token::new(['!', '='].iter().peekable()).unwrap(),
            Token::ExclaimEquals
        )
    }

    #[test]
    fn lesseq() {
        assert_eq!(
            Token::new(['<', '='].iter().peekable()).unwrap(),
            Token::LessEquals
        )
    }

    #[test]
    fn greatereq() {
        assert_eq!(
            Token::new(['>', '='].iter().peekable()).unwrap(),
            Token::GreaterEquals
        )
    }
}
