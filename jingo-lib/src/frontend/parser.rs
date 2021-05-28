//! Parser for converting lexed tokens into the finalized abstract syntax tree

use super::{ast::*, lexer::Token};
use logos::Lexer;
use std::fmt;

/// Parsing-specific error/stop enumeration, encompassing the possible errors or
/// stops in parsing flow which may have occurred during parsing
#[derive(Debug, Clone, PartialEq)]
pub enum ParseStop {
    //--------//
    // errors //
    //--------//
    /// Unexpected token
    UnexpectedToken(String),

    /// Unknown token whilst lexing
    UnknownToken(String),

    /// Operation was found with no lefthand expression
    NoLeftExpr,

    /// File ended unexpectedly
    UnexpectedEof,

    //---------//
    // special //
    //---------//
    /// File ended expectedly
    FileEnded,
}

impl fmt::Display for ParseStop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseStop::UnexpectedToken(slice) => write!(f, "Unexpected token '{}' found", slice),
            ParseStop::UnknownToken(slice) => write!(f, "Unknown token '{}' found", slice),
            ParseStop::NoLeftExpr => {
                write!(f, "Operation was found with no lefthand expression")
            }
            ParseStop::UnexpectedEof => write!(f, "File ended unexpectedly"),
            ParseStop::FileEnded => {

                write!(f, "File ended expectedly, please report this as a bug!")
            }
        }
    }
}

/// Parses a given lexer input into the resulting parsed values
pub fn launch(lex: &mut Lexer<Token>) -> Result<Vec<Expr>, ParseStop> {
    let mut buf = None;
    let mut output = vec![];

    loop {
        let buf_was_some = buf.is_some();

        match next(lex, &mut buf, None, true) {
            Ok(expr) => {
                if buf_was_some && buf.is_some() {
                    output.push(buf.take().unwrap());
                }

                buf = Some(expr);
            }
            Err(ParseStop::FileEnded) => break,
            Err(unknown) => return Err(unknown.into()),
        }
    }

    match buf {
        Some(expr) => output.push(expr),
        None => (),
    }

    Ok(output)
}

/// Gets the next full expression, used internally as the main parsing hook
fn next(
    lex: &mut Lexer<Token>,
    buf: &mut Option<Expr>,
    doc: Option<String>,
    is_topmost: bool,
) -> Result<Expr, ParseStop> {
    let cur = lex.next();
    let start = lex.span().start;

    match cur {
        Some(Token::Plus) => Ok(Expr::from_parse(
            op_flow(lex, buf, OpKind::Add)?,
            None,
            start,
        )),
        Some(Token::FwdSlash) => Ok(Expr::from_parse(
            op_flow(lex, buf, OpKind::Div)?,
            None,
            start,
        )),
        Some(Token::Exclaim) => Ok(Expr::from_parse(Not(box_next(lex)?), doc, start)),
        Some(Token::True) => Ok(Expr::from_parse(BoolLit(true), doc, start)),
        Some(Token::False) => Ok(Expr::from_parse(BoolLit(false), doc, start)),
        Some(Token::Let) => Ok(Expr::from_parse(let_flow(lex)?, doc, start)),
        Some(Token::Return) => Ok(Expr::from_parse(Return(box_next(lex)?), doc, start)),
        Some(Token::Str(d)) => Ok(Expr::from_parse(StrLit(d), doc, start)),
        Some(Token::Char(d)) => Ok(Expr::from_parse(CharLit(d), doc, start)),
        Some(Token::Float(d)) => Ok(Expr::from_parse(FloatLit(d), doc, start)),
        Some(Token::Int(d)) => Ok(Expr::from_parse(IntLit(d), doc, start)),
        Some(Token::Doc(string)) => next(lex, buf, Some(string), is_topmost),
        Some(Token::Fun) => Ok(Expr::from_parse(subprogram_flow(lex)?, doc, start)),
        Some(Token::Path(_path)) => todo!("pathing"),
        Some(Token::Error) => Err(ParseStop::UnknownToken(lex.slice().to_string())),
        Some(_) => Err(ParseStop::UnexpectedToken(lex.slice().to_string())),
        None => Err(match is_topmost {
            true => ParseStop::FileEnded,
            false => ParseStop::UnexpectedEof,
        }),
    }
}

/// Flow for operation grammar, i.e. adding or subtracting
fn op_flow(lex: &mut Lexer<Token>, buf: &mut Option<Expr>, kind: OpKind) -> Result<Op, ParseStop> {
    Ok(Op {
        left: Box::new(buf.take().ok_or(ParseStop::NoLeftExpr)?),
        right: box_next(lex)?,
        kind,
    })
}

/// Flow for `let` grammar
fn let_flow(lex: &mut Lexer<Token>) -> Result<Let, ParseStop> {
    let (path, mutable) = match lex.next().ok_or(ParseStop::UnexpectedEof)? {
        Token::Path(path) => Ok((path, false)),
        Token::Mut => {
            if let Token::Path(path) = lex.next().ok_or(ParseStop::UnexpectedEof)? {
                Ok((path, true))
            } else {
                Err(ParseStop::UnexpectedToken(lex.slice().to_string()))
            }
        }
        _ => Err(ParseStop::UnexpectedToken(lex.slice().to_string())),
    }?;

    ensure(lex, Token::Equals)?;

    Ok(Let {
        path,
        mutable,
        expr: box_next(lex)?,
    })
}

fn subprogram_flow(lex: &mut Lexer<Token>) -> Result<Function, ParseStop> {
    let path = match lex.next() {
        Some(Token::Path(path)) => Ok(path),
        Some(_) => Err(ParseStop::UnknownToken(lex.slice().to_string())),
        None => Err(ParseStop::UnexpectedEof),
    }?;

    ensure(lex, Token::ParenLeft)?;

    let mut args = vec![];

    loop {
        match lex.next().ok_or(ParseStop::UnexpectedEof)? {
            Token::Path(path) => args.push(
                path.to_id()
                    .ok_or(ParseStop::UnexpectedToken(lex.slice().to_string()))?,
            ),
            Token::ParenRight => break,
            _ => return Err(ParseStop::UnexpectedToken(lex.slice().to_string())),
        }
    }

    ensure(lex, Token::BraceLeft)?;

    let mut body = vec![];

    loop {
        match next(lex, &mut None, None, false) {
            Ok(expr) => body.push(expr),
            Err(ParseStop::UnexpectedToken(slice)) if slice == String::from("}") => break,
            Err(err) => return Err(err),
        }
    }

    Ok(Function { path, args, body })
}

// /// Gets path from next [Lexer] token or errors
// fn get_path(lex: &mut Lexer<Token>) -> Result<Path, ParseStop> {
//     match lex.next() {
//         Some(Token::Path(path)) => Ok(path),
//         unknown => Err(unknown.into()),
//     }
// }

/// Gets next expression without passing a previous `buf` of `doc` and returns a
/// [Box], used as a shortcut for sequential parsing
fn box_next(lex: &mut Lexer<Token>) -> Result<Box<Expr>, ParseStop> {
    Ok(Box::new(next(lex, &mut None, None, false)?))
}

/// Ensures next lex token equals inputted `token` value
fn ensure(lex: &mut Lexer<Token>, token: Token) -> Result<(), ParseStop> {
    match lex.next() {
        Some(found) if found == token => Ok(()),
        Some(_) => Err(ParseStop::UnexpectedToken(lex.slice().to_string())),
        None => Err(ParseStop::UnexpectedEof),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use logos::Logos;

    // TODO: basic math
    // TODO: boxed expressions
    // TODO: order of operations

    #[test]
    fn lets() {
        assert_eq!(
            next(&mut Token::lexer("let x = 5"), &mut None, None, true).unwrap(),
            Expr {
                kind: ExprKind::Let(Let {
                    mutable: false,
                    path: Path::new("x".to_string()),
                    expr: Box::new(Expr {
                        kind: ExprKind::IntLit(IntLit(5)),
                        doc: None,
                        start: 8
                    })
                }),
                doc: None,
                start: 0
            }
        );
    }

    #[test]
    fn basic_errs() {
        assert_eq!(
            next(&mut Token::lexer("let x + 5"), &mut None, None, true),
            Err(ParseStop::UnexpectedToken("+".to_string()))
        );
        assert_eq!(
            next(&mut Token::lexer("#"), &mut None, None, true),
            Err(ParseStop::UnknownToken("#".to_string()))
        );
        assert_eq!(
            next(&mut Token::lexer("let x = -- 5"), &mut None, None, true),
            Err(ParseStop::UnexpectedEof)
        );
    }

    #[test]
    fn parse_launch() {
        assert_eq!(
            launch(&mut Token::lexer("5 + 3")).unwrap(),
            vec![Expr {
                kind: ExprKind::Op(Op {
                    left: Box::new(Expr {
                        kind: ExprKind::IntLit(IntLit(5)),
                        doc: None,
                        start: 0
                    }),
                    right: Box::new(Expr {
                        kind: ExprKind::IntLit(IntLit(3)),
                        doc: None,
                        start: 4
                    }),
                    kind: OpKind::Add
                }),
                doc: None,
                start: 2
            }]
        );
        assert_eq!(
            launch(&mut Token::lexer("!5")).unwrap(),
            vec![Expr {
                kind: ExprKind::Not(Not(Box::new(Expr {
                    kind: ExprKind::IntLit(IntLit(5)),
                    doc: None,
                    start: 1
                }))),
                doc: None,
                start: 0
            }]
        );
        assert_eq!(launch(&mut Token::lexer("+ 5")), Err(ParseStop::NoLeftExpr));
        assert_eq!(
            launch(&mut Token::lexer("5 +")),
            Err(ParseStop::UnexpectedEof)
        );
        assert_eq!(
            launch(&mut Token::lexer("5 + 5 + 5 +")),
            Err(ParseStop::UnexpectedEof)
        );
    }

    #[test]
    fn pathing() {
        // basic tests are in lexer
        assert_eq!(
            launch(&mut Token::lexer("hello_there.five.ten.fifteen")).unwrap(),
            vec![Expr {
                kind: LetCall::from(Path {
                    fields: vec!["hello_there".into(), "five".into(), "ten".into()],
                    id: "fifteen".into()
                })
                .into(),
                doc: None,
                start: 0
            }]
        );
        assert_ne!(
            launch(&mut Token::lexer("hello1_there")).unwrap(),
            vec![Expr {
                kind: LetCall::from(Path::new("hello1_there")).into(),
                doc: None,
                start: 0
            }]
        );
    }

    #[test]
    fn function_basics() {
        assert_eq!(
            launch(&mut Token::lexer("fun main() {}")).unwrap(),
            vec![Expr {
                kind: Function {
                    path: Path::new("main"),
                    args: vec![],
                    body: vec![]
                }
                .into(),
                doc: None,
                start: 0
            }]
        );

        assert_eq!(
            launch(&mut Token::lexer("fun main() { 1 'c' }")).unwrap(),
            vec![Expr {
                kind: Function {
                    path: Path::new("main"),
                    args: vec![],
                    body: vec![
                        Expr {
                            kind: ExprKind::IntLit(IntLit(1)),
                            doc: None,
                            start: 13
                        },
                        Expr {
                            kind: ExprKind::CharLit(CharLit('c')),
                            doc: None,
                            start: 15
                        }
                    ]
                }
                .into(),
                doc: None,
                start: 0
            }]
        );

        let sixnine_plus_two = Expr {
            kind: Op {
                left: Box::new(Expr {
                    kind: IntLit(69).into(),
                    doc: None,
                    start: 19,
                }),
                right: Box::new(Expr {
                    kind: IntLit(2).into(),
                    doc: None,
                    start: 24,
                }),
                kind: OpKind::Add,
            }
            .into(),
            doc: None,
            start: 19,
        };

        assert_eq!(
            launch(&mut Token::lexer("fun hello_there() { 69 + 2 }")).unwrap(),
            vec![Expr {
                kind: Function {
                    path: Path::new("hello_there").into(),
                    args: vec![],
                    body: vec![sixnine_plus_two]
                }
                .into(),
                doc: None,
                start: 0
            }]
        );
    }
}
