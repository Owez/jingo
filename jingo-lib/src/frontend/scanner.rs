//! Scanner/lexer stage of parsing, the first main step to parse raw characters
//! into further parsable tokens

use crate::{Meta, MetaPos};

use std::{fmt, iter::Peekable};

/// Error enumeration representing errors whilst scanning; see the [fmt::Display]
/// trait impl for documentation on each case
#[derive(Debug, Clone, PartialEq)]
pub enum ScanError {
    TokenInnerNotFound(String),
    UnexpectedEof,
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScanError::TokenInnerNotFound(input) => {
                write!(f, "Input '{}' is not a known keyword or identifier", input)
            }
            ScanError::UnexpectedEof => {
                write!(f, "File ended abruptly whilst scanning, unexpected EOF")
            }
        }
    }
}

/// Type enumeration of a token, defining the possible types for a token, along
/// with any data (such as in string literals) the token may use
#[derive(Debug, Clone, PartialEq)]
pub enum TokenInner {
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

impl TokenInner {
    /// Creates a new [TokenInner] from input string, returning a [ScanError::TokenInnerNotFound]
    /// if the token could not be found
    pub fn new(input: &mut Peekable<impl Iterator<Item = char>>) -> Result<Self, ScanError> {
        match input.next().ok_or(ScanError::UnexpectedEof)? {
            '(' => Ok(TokenInner::ParenLeft),
            ')' => Ok(TokenInner::ParenRight),
            '{' => Ok(TokenInner::BraceLeft),
            '}' => Ok(TokenInner::BraceRight),
            ',' => Ok(TokenInner::Comma),
            '.' => Ok(TokenInner::Dot),
            ';' => Ok(TokenInner::Semicolon),
            '/' => Ok(TokenInner::FwdSlash),
            '*' => Ok(TokenInner::Star),
            '\n' => Ok(TokenInner::Newline),
            ' ' | '\t' => Ok(TokenInner::Whitespace),
            '+' => Ok(TokenInner::Plus),
            '-' => Ok(TokenInner::Minus),
            '=' => match input.peek() {
                Some(&'=') => {
                    input.next();
                    Ok(TokenInner::EqualsEquals)
                }
                _ => Ok(TokenInner::Equals),
            },
            '!' => match input.peek() {
                Some(&'=') => {
                    input.next();
                    Ok(TokenInner::ExclaimEquals)
                }
                _ => Ok(TokenInner::Exclaim),
            },
            '<' => match input.peek() {
                Some(&'=') => {
                    input.next();
                    Ok(TokenInner::LessEquals)
                }
                _ => Ok(TokenInner::Less),
            },
            '>' => match input.peek() {
                Some(&'=') => {
                    input.next();
                    Ok(TokenInner::GreaterEquals)
                }
                _ => Ok(TokenInner::Greater),
            },
            '"' => todo!("string"),
            '\'' => todo!("char"),
            _ => todo!("id"),
        }
    }
}

/// Represents a token with a token type + data (i.e. [TokenInner]) along with
/// positional data (i.e. [MetaPos]) where the token starts
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// Type + data of this token
    pub inner: TokenInner,

    /// Positional data for where this token occurs
    pub pos: MetaPos,
}

impl From<Token> for TokenInner {
    fn from(token: Token) -> Self {
        token.inner
    }
}

impl From<Token> for MetaPos {
    fn from(token: Token) -> Self {
        token.pos
    }
}

/// Scan given input into a vector of [Token] for further compilation
pub fn launch(mut meta: Meta, input: impl AsRef<str>) -> Result<Vec<Token>, (ScanError, Meta)> {
    let mut input = input.as_ref().chars().into_iter().peekable();
    let mut output = vec![];

    loop {
        meta.pos.col += 1;

        match TokenInner::new(&mut input) {
            Ok(inner) => {
                if inner == TokenInner::Newline {
                    meta.newline(1)
                }

                output.push(Token {
                    inner,
                    pos: meta.pos.clone(),
                })
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
            TokenInner::new(&mut "==".chars().peekable()).unwrap(),
            TokenInner::EqualsEquals
        )
    }

    #[test]
    fn neeq() {
        assert_eq!(
            TokenInner::new(&mut "!=".chars().peekable()).unwrap(),
            TokenInner::ExclaimEquals
        )
    }

    #[test]
    fn lesseq() {
        assert_eq!(
            TokenInner::new(&mut "<=".chars().peekable()).unwrap(),
            TokenInner::LessEquals
        )
    }

    #[test]
    fn greatereq() {
        assert_eq!(
            TokenInner::new(&mut ">=".chars().peekable()).unwrap(),
            TokenInner::GreaterEquals
        )
    }

    #[test]
    fn launch_scan() {
        let tokens = launch(Meta::new(None), "=!==!=!!=").unwrap();
        let exp = vec![
            TokenInner::Equals,
            TokenInner::ExclaimEquals,
            TokenInner::Equals,
            TokenInner::ExclaimEquals,
            TokenInner::Exclaim,
            TokenInner::ExclaimEquals,
        ];

        for (ind, token) in tokens.iter().enumerate() {
            assert_eq!(token.inner, exp[ind]);
        }
    }
}
