//! Scanner/lexer stage of parsing, the first main step to parse raw characters
//! into further parsable tokens

use crate::Meta;

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
    Newline,
    Whitespace,

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
    Str(String),
    Char(char),
    Number(i64),
}

impl Token {
    /// Creates a new [Token] from input string, returning a [ScanError::TokenNotFound]
    /// if the token could not be found
    pub fn new(input: &mut Peekable<impl Iterator<Item = char>>) -> Result<Self, ScanError> {
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
            '\n' => Ok(Token::Newline),
            ' ' | '\t' => Ok(Token::Whitespace),
            '+' => Ok(Token::Plus),
            '-' => Ok(Token::Minus),
            '=' => match input.peek() {
                Some(&'=') => {
                    input.next();
                    Ok(Token::EqualsEquals)
                }
                _ => Ok(Token::Equals),
            },
            '!' => match input.peek() {
                Some(&'=') => {
                    input.next();
                    Ok(Token::ExclaimEquals)
                }
                _ => Ok(Token::Exclaim),
            },
            '<' => match input.peek() {
                Some(&'=') => {
                    input.next();
                    Ok(Token::LessEquals)
                }
                _ => Ok(Token::Less),
            },
            '>' => match input.peek() {
                Some(&'=') => {
                    input.next();
                    Ok(Token::GreaterEquals)
                }
                _ => Ok(Token::Greater),
            },
            '"' => todo!("string"),
            '\'' => todo!("char"),
            _ => todo!("id"),
        }
    }
}

/// Scan given input into a vector of [Token] for further compilation
pub fn launch(mut meta: Meta, input: impl AsRef<str>) -> Result<Vec<Token>, (ScanError, Meta)> {
    let mut input = input.as_ref().chars().into_iter().peekable();
    let mut output = vec![];

    loop {
        meta.col += 1;

        match Token::new(&mut input) {
            Ok(token) => {
                if token == Token::Newline {
                    meta.newline(1)
                }

                output.push(token)
            }
            Err(ScanError::UnexpectedEof) => break,
            Err(err) => return Err((err, meta)),
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eqeq() {
        assert_eq!(
            Token::new(&mut "==".chars().peekable()).unwrap(),
            Token::EqualsEquals
        )
    }

    #[test]
    fn neeq() {
        assert_eq!(
            Token::new(&mut "!=".chars().peekable()).unwrap(),
            Token::ExclaimEquals
        )
    }

    #[test]
    fn lesseq() {
        assert_eq!(
            Token::new(&mut "<=".chars().peekable()).unwrap(),
            Token::LessEquals
        )
    }

    #[test]
    fn greatereq() {
        assert_eq!(
            Token::new(&mut ">=".chars().peekable()).unwrap(),
            Token::GreaterEquals
        )
    }

    #[test]
    fn launch_scan() {
        assert_eq!(
            launch(Meta::new(None), "=!==!=!!=").unwrap(),
            vec![
                Token::Equals,
                Token::ExclaimEquals,
                Token::Equals,
                Token::ExclaimEquals,
                Token::Exclaim,
                Token::ExclaimEquals
            ]
        )
    }
}
