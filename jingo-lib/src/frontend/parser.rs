use super::{ast, lexer::Token};
use logos::Lexer;
use std::fmt;

/// Errors which may occur during parsing, specifically from the [Parse] trait
///
/// See the [fmt::Display] trait implementation for user-specific documentation
/// on each error variation.
///
/// This is also typically associated with a [Token] to give further context,
/// with this enumeration just being the kind of error which occured.
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

fn parse_id<'a>(
    lex: &'a mut Lexer<'a, Token>,
) -> Result<ast::Id, (ParseError, &'a mut Lexer<'a, Token>)> {
    match lex.next() {
        Some(Token::Id(inner)) => Ok(ast::Id {
            inner,
            range: lex.span(),
        }),
        unknown => Err((unknown.into(), lex)),
    }
}

fn parse_doc<'a>(
    lex: &'a mut Lexer<'a, Token>,
) -> Result<ast::Doc, (ParseError, &'a mut Lexer<'a, Token>)> {
    match lex.next() {
        Some(Token::Doc(inner)) => Ok(ast::Doc {
            inner,
            range: lex.span(),
        }),
        unknown => Err((unknown.into(), lex)),
    }
}
