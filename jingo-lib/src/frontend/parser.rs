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
    fn parse(lex: &mut Lexer<'a, Token>) -> Result<Self, ParseError>;
}

impl<'a> Parse<'a> for ast::Id {
    fn parse(lex: &mut Lexer<'a, Token>) -> Result<Self, ParseError> {
        match lex.next() {
            Some(Token::Id(inner)) => Ok(Self(inner)),
            unknown => Err(unknown.into()),
        }
    }
}

impl<'a> Parse<'a> for ast::Class {
    fn parse(lex: &mut Lexer<'a, Token>) -> Result<Self, ParseError> {
        match lex.next() {
            Some(Token::Class) => Ok(Self(ast::Id::parse(lex)?)),
            unknown => Err(unknown.into()),
        }
    }
}

impl<'a> Parse<'a> for ast::Function {
    fn parse(lex: &mut Lexer<'a, Token>) -> Result<Self, ParseError> {
        next(lex, Token::Fun)?;
        let id = ast::Id::parse(lex)?;

        let args = subprogram_args(lex)?;
        let body = brace_body(lex)?;

        Ok(Self { id, args, body })
    }
}

impl<'a> Parse<'a> for ast::Method {
    fn parse(lex: &mut Lexer<'a, Token>) -> Result<Self, ParseError> {
        next(lex, Token::Fun)?;
        let class_id = ast::Id::parse(lex)?;

        let creation_method = match lex.next() {
            Some(Token::Static) => true,
            Some(Token::Dot) => false,
            unknown => return Err(unknown.into()),
        };

        let id = ast::Id::parse(lex)?;

        let args = subprogram_args(lex)?;
        let body = brace_body(lex)?;

        Ok(Self {
            class_id,
            creation_method,
            id,
            args,
            body,
        })
    }
}

impl<'a> Parse<'a> for ast::IntLit {
    fn parse(lex: &mut Lexer<'a, Token>) -> Result<Self, ParseError> {
        match lex.next() {
            Some(Token::Int(inner)) => Ok(Self(inner)),
            unknown => Err(unknown.into()),
        }
    }
}

impl<'a> Parse<'a> for ast::FloatLit {
    fn parse(lex: &mut Lexer<'a, Token>) -> Result<Self, ParseError> {
        match lex.next() {
            Some(Token::Float(inner)) => Ok(Self(inner)),
            unknown => Err(unknown.into()),
        }
    }
}

impl<'a> Parse<'a> for ast::StringLit {
    fn parse(lex: &mut Lexer<'a, Token>) -> Result<Self, ParseError> {
        match lex.next() {
            Some(Token::Str(inner)) => Ok(Self(inner)),
            unknown => Err(unknown.into()),
        }
    }
}

impl<'a> Parse<'a> for ast::CharLit {
    fn parse(lex: &mut Lexer<'a, Token>) -> Result<Self, ParseError> {
        match lex.next() {
            Some(Token::Char(inner)) => Ok(Self(inner)),
            unknown => Err(unknown.into()),
        }
    }
}

fn next<'a>(lex: &mut Lexer<'a, Token>, token: Token) -> Result<(), ParseError> {
    let got = lex.next();

    if got == Some(token) {
        Ok(())
    } else {
        Err(got.into())
    }
}

/// Gets arguments for a subprogram within `(x,y,z)` formatting
fn subprogram_args<'a>(lex: &mut Lexer<'a, Token>) -> Result<Vec<ast::Id>, ParseError> {
    next(lex, Token::ParenLeft)?;
    let mut args = vec![];

    loop {
        args.push(ast::Id::parse(lex)?);

        match lex.next() {
            Some(Token::Comma) => (),
            Some(Token::ParenRight) => break,
            unknown => return Err(unknown.into()),
        }
    }

    Ok(args)
}

/// Matches expressions until a symmetrical `{}` pair is found, making a body
fn brace_body<'a>(_lex: &mut Lexer<'a, Token>) -> Result<Vec<ast::Expr>, ParseError> {
    todo!("body parsing")
}

// /// Runs through given tokens and checks [PartialEq] or errors
// fn exp_tokens<'a>(lex: &mut Lexer<'a, Token>, tokens: Vec<Token>) -> Result<(), ParseError> {
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
