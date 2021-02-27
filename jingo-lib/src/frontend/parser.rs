use super::{ast::*, lexer::Token};
use logos::Lexer;
use std::fmt;

const REPORT_BUG: &str = ", please report this as a bug";

/// Parsing-specific error/stop enumeration, encompassing the possible errors or
/// stops in parsing flow which may have occurred during parsing
#[derive(Debug, Clone, PartialEq)]
pub enum ParseStop {
    /// Unexpected token
    UnexpectedToken,

    /// File ended unexpectedly
    UnexpectedEof,

    /// Indicates that the file ended expectedly for the parser
    FileEnded,

    /// Parser returning documentation
    FoundDoc(String),
}

impl<T> From<Option<T>> for ParseStop {
    /// Matches basic parsing errors from an [Option], used for matching basic/common
    /// parsing errors
    ///
    /// - [Some]: [ParseStop::UnexpectedToken]
    /// - [None]: [ParseStop::UnexpectedEof]
    fn from(option: Option<T>) -> ParseStop {
        match option {
            Some(_) => ParseStop::UnexpectedToken,
            None => ParseStop::UnexpectedEof,
        }
    }
}

impl fmt::Display for ParseStop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseStop::UnexpectedToken => write!(f, "Unexpected token"),
            ParseStop::UnexpectedEof => write!(f, "File ended unexpectedly"),
            ParseStop::FileEnded => write!(f, "File ended expectedly{}", REPORT_BUG),
            ParseStop::FoundDoc(_) => write!(f, "Writing doc to ast{}", REPORT_BUG),
        }
    }
}

pub fn launch<'a>(lex: &mut Lexer<'a, Token>) -> Result<Vec<Expr>, ParseStop> {
    let mut output = Vec::new();
    let mut doc = Vec::new();

    loop {
        match match_next(lex, &mut doc) {
            Ok(expr) => output.push(expr),
            Err(ParseStop::FileEnded) => break,
            Err(err) => return Err(err),
        }
    }

    Ok(output)
}

fn match_next<'a>(lex: &mut Lexer<'a, Token>, doc: &mut Vec<String>) -> Result<Expr, ParseStop> {
    let cur = lex.next();
    let start = lex.span().start;

    match cur {
        Some(Token::Class) => Ok(Expr::from_parse(
            Class::parse(lex)?,
            doc,
            start,
            lex.span().end,
        )),
        Some(Token::Fun) => Ok(Expr::from_parse(fun_flow(lex)?, doc, start, lex.span().end)),
        Some(Token::Doc(line)) => Err(ParseStop::FoundDoc(line)),
        Some(_) => Err(ParseStop::UnexpectedToken),
        None => Err(ParseStop::FileEnded),
    }
}

/// Allows direct parsing from a raw [Lexer] stream if the next token is known
trait DirectParse: Sized {
    /// Parses next token in [Lexer] stream
    fn parse<'a>(lex: &mut Lexer<'a, Token>) -> Result<Self, ParseStop>;
}

impl DirectParse for Id {
    fn parse<'a>(lex: &mut Lexer<'a, Token>) -> Result<Self, ParseStop> {
        match lex.next() {
            Some(Token::Id(id)) => Ok(Self(id)),
            unknown => Err(unknown.into()),
        }
    }
}

impl DirectParse for Class {
    fn parse<'a>(lex: &mut Lexer<'a, Token>) -> Result<Self, ParseStop> {
        Ok(Self(Id::parse(lex)?))
    }
}

/// Start of function flow
fn fun_flow<'a>(lex: &mut Lexer<'a, Token>) -> Result<ExprKind, ParseStop> {
    let first_id = Id::parse(lex)?;

    match lex.next() {
        Some(Token::Static) => Ok(ExprKind::Method(get_method(lex, first_id, true)?)),
        Some(Token::Dot) => Ok(ExprKind::Method(get_method(lex, first_id, false)?)),
        Some(Token::ParenLeft) => Ok(ExprKind::Function(Function {
            id: first_id,
            args: subprogram_args(lex)?,
            body: brace_body(lex)?,
        })),
        unknown => Err(unknown.into()),
    }
}

fn get_method(
    lex: &mut Lexer<Token>,
    class_id: Id,
    creation_method: bool,
) -> Result<Method, ParseStop> {
    Ok(Method {
        id: Id::parse(lex)?,
        creation_method,
        class_id,
        args: subprogram_args(lex)?,
        body: brace_body(lex)?,
    })
}

// /// Skips token cleanly as long as it matches inputted `token` enum
// fn next<'a>(lex: &mut Lexer<'a, Token>, token: Token) -> Result<(), ParseStop> {
//     let got = lex.next();

//     if got == Some(token) {
//         Ok(())
//     } else {
//         Err(got.into())
//     }
// }

/// Gets arguments for a subprogram within `x,y,z)` formatting, assuming `(` has
/// been taken by greedy parsing
fn subprogram_args<'a>(lex: &mut Lexer<'a, Token>) -> Result<Vec<Id>, ParseStop> {
    let mut args = vec![];

    loop {
        args.push(Id::parse(lex)?);

        match lex.next() {
            Some(Token::Comma) => (),
            Some(Token::ParenRight) => break,
            unknown => return Err(unknown.into()),
        }
    }

    Ok(args)
}

/// Matches expressions until a symmetrical `{}` pair is found, making a body
fn brace_body<'a>(_lex: &mut Lexer<'a, Token>) -> Result<Vec<Expr>, ParseStop> {
    todo!("body parsing")
}

// impl<'a> Parse<'a> for IntLit {
//     fn parse(lex: &mut Lexer<'a, Token>) -> Result<Self, ParseStop> {
//         match lex.next() {
//             Some(Token::Int(inner)) => Ok(Self(inner)),
//             unknown => Err(unknown.into()),
//         }
//     }
// }

// impl<'a> Parse<'a> for FloatLit {
//     fn parse(lex: &mut Lexer<'a, Token>) -> Result<Self, ParseStop> {
//         match lex.next() {
//             Some(Token::Float(inner)) => Ok(Self(inner)),
//             unknown => Err(unknown.into()),
//         }
//     }
// }

// impl<'a> Parse<'a> for StringLit {
//     fn parse(lex: &mut Lexer<'a, Token>) -> Result<Self, ParseStop> {
//         match lex.next() {
//             Some(Token::Str(inner)) => Ok(Self(inner)),
//             unknown => Err(unknown.into()),
//         }
//     }
// }

// impl<'a> Parse<'a> for CharLit {
//     fn parse(lex: &mut Lexer<'a, Token>) -> Result<Self, ParseStop> {
//         match lex.next() {
//             Some(Token::Char(inner)) => Ok(Self(inner)),
//             unknown => Err(unknown.into()),
//         }
//     }
// }

// // /// Runs through given tokens and checks [PartialEq] or errors
// // fn exp_tokens<'a>(lex: &mut Lexer<'a, Token>, tokens: Vec<Token>) -> Result<(), ParseStop> {
// //     for token in tokens {
// //         match lex.next() {
// //             Some(token) => (),
// //             unknown => return Err(unknown.into()),
// //         }
// //     }

// //     Ok(())
// // }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::frontend::lexer::Token;
//     use logos::Logos;

//     #[test]
//     fn id() {
//         assert_eq!(
//             Id::parse(&mut Token::lexer("cool_id")).unwrap(),
//             Id("cool_id".to_string())
//         );
//     }

//     #[test]
//     fn class() {
//         assert_eq!(
//             Class::parse(&mut Token::lexer("class my_class")).unwrap(),
//             Class(Id("my_class".to_string()))
//         )
//     }
// }
