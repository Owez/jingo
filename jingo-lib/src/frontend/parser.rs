use super::{ast, lexer::Token};
use logos::Lexer;
use std::fmt;

/// Parsing-specific error enumeration, encompassing the possible errors which
/// may have occurred during parsing using the [Parse] trait
///
/// See the [fmt::Display] trait implementation for user-specific documentation
/// on each error variation.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnknownToken,
    UnexpectedEof,
}

impl<T> From<Option<T>> for ParseError {
    /// Matches basic parsing errors from an [Option], used for matching basic/common
    /// parsing errors
    ///
    /// - [Some]: [ParseError::UnknownToken]
    /// - [None]: [ParseError::UnexpectedEof]
    fn from(option: Option<T>) -> ParseError {
        match option {
            Some(_) => ParseError::UnknownToken,
            None => ParseError::UnexpectedEof,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnknownToken => write!(f, "Unknown token"),
            ParseError::UnexpectedEof => write!(f, "File ended unexpectedly"),
        }
    }
}

/// Parsing trait, defining parsing flow when provided with a lexed lexer stream
pub trait Parse<'a>: Sized {
    /// Parses inputted lexing stream
    fn parse(lex: &'a mut Lexer<'a, Token>) -> Result<Self, ParseError>;
}

impl<'a> Parse<'a> for ast::Id {
    fn parse(lex: &'a mut Lexer<'a, Token>) -> Result<Self, ParseError> {
        match lex.next() {
            Some(Token::Id(inner)) => Ok(Self(inner)),
            unknown => Err(unknown.into()),
        }
    }
}

impl<'a> Parse<'a> for ast::Class {
    fn parse(lex: &'a mut Lexer<'a, Token>) -> Result<Self, ParseError> {
        match lex.next() {
            Some(Token::Class) => Ok(Self(ast::Id::parse(lex)?)),
            unknown => Err(unknown.into()),
        }
    }
}

impl<'a> Parse<'a> for ast::IntLit {
    fn parse(lex: &'a mut Lexer<'a, Token>) -> Result<Self, ParseError> {
        match lex.next() {
            Some(Token::Int(inner)) => Ok(Self(inner)),
            unknown => Err(unknown.into()),
        }
    }
}

impl<'a> Parse<'a> for ast::FloatLit {
    fn parse(lex: &'a mut Lexer<'a, Token>) -> Result<Self, ParseError> {
        match lex.next() {
            Some(Token::Float(inner)) => Ok(Self(inner)),
            unknown => Err(unknown.into()),
        }
    }
}

impl<'a> Parse<'a> for ast::CharLit {
    fn parse(lex: &'a mut Lexer<'a, Token>) -> Result<Self, ParseError> {
        match lex.next() {
            Some(Token::Char(inner)) => Ok(Self(inner)),
            unknown => Err(unknown.into()),
        }
    }
}

// /// Runs through given tokens and checks [PartialEq] or errors
// fn exp_tokens<'a>(lex: &'a mut Lexer<'a, Token>, tokens: Vec<Token>) -> Result<(), ParseError> {
//     for token in tokens {
//         match lex.next() {
//             Some(token) => (),
//             unknown => return Err(unknown.into()),
//         }
//     }

//     Ok(())
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::lexer::Token;
    use logos::Logos;

    #[test]
    fn id() {
        assert_eq!(
            ast::Id::parse(&mut Token::lexer("cool_id")).unwrap(),
            ast::Id("cool_id".to_string())
        );
    }

    #[test]
    fn class() {
        assert_eq!(
            ast::Class::parse(&mut Token::lexer("class my_class")).unwrap(),
            ast::Class(ast::Id("my_class".to_string()))
        )
    }
}
