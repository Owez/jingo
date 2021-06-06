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

    /// Multiple expressions where given where a single expression should be
    MultipleExpressions,

    /// Class names need to be a single identifier, not a path
    ClassNameIsPath,

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
            ParseStop::MultipleExpressions => write!(
                f,
                "Multiple expressions given where a single expression should be"
            ),
            ParseStop::ClassNameIsPath => {
                write!(f, "Class name is a path and not a single identifier")
            }
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
            doc,
            start,
        )),
        Some(Token::FwdSlash) => Ok(Expr::from_parse(
            op_flow(lex, buf, OpKind::Div)?,
            doc,
            start,
        )),
        Some(Token::Exclaim) => Ok(Expr::from_parse(Not(box_next(lex)?), doc, start)),
        Some(Token::True) => Ok(Expr::from_parse(BoolLit(true), doc, start)),
        Some(Token::False) => Ok(Expr::from_parse(BoolLit(false), doc, start)),
        Some(Token::None) => Ok(Expr::from_parse(ExprKind::None, doc, start)),
        Some(Token::Class) => Ok(Expr::from_parse(class_flow(lex)?, doc, start)),
        Some(Token::While) => Ok(Expr::from_parse(while_flow(lex)?, doc, start)),
        Some(Token::Return) => Ok(Expr::from_parse(Return(box_next(lex)?), doc, start)),
        Some(Token::Let) => Ok(Expr::from_parse(let_flow(lex)?, doc, start)),
        Some(Token::Str(d)) => Ok(Expr::from_parse(StrLit(d), doc, start)),
        Some(Token::Char(d)) => Ok(Expr::from_parse(CharLit(d), doc, start)),
        Some(Token::Float(d)) => Ok(Expr::from_parse(FloatLit(d), doc, start)),
        Some(Token::Int(d)) => Ok(Expr::from_parse(IntLit(d), doc, start)),
        Some(Token::Doc(d)) => next(lex, buf, Some(d), is_topmost),
        Some(Token::Fun) => Ok(Expr::from_parse(subprogram_flow(lex)?, doc, start)),
        Some(Token::Path(_d)) => todo!("pathing"),
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
    let (path, mutable) = match lex.next() {
        Some(Token::Path(path) )=> Ok((path, false)),
        Some(Token::Mut) if let Token::Path(path) = lex.next().ok_or(ParseStop::UnexpectedEof)? =>  Ok((path, true)),
        Some(_) => Err(ParseStop::UnexpectedToken(lex.slice().to_string())),
        None => Err(ParseStop::UnexpectedEof)
    }?;

    ensure(lex, Token::Equals)?;

    Ok(Let {
        path,
        mutable,
        expr: box_next(lex)?,
    })
}

/// Flow for `class` objects
fn class_flow(lex: &mut Lexer<Token>) -> Result<Class, ParseStop> {
    match lex.next() {
        Some(Token::Path(path)) => Ok(Class {
            id: path.to_id().ok_or(ParseStop::ClassNameIsPath)?,
            body: {
                ensure(lex, Token::BraceLeft)?;
                get_body(lex)?
            },
        }),
        Some(_) => Err(ParseStop::UnexpectedToken(lex.slice().to_string())),
        None => Err(ParseStop::UnexpectedEof),
    }
}

/// Flow for `while` loops
fn while_flow(lex: &mut Lexer<Token>) -> Result<While, ParseStop> {
    Ok(While {
        condition: Box::new(get_condition(lex)?),
        body: get_body(lex)?,
    })
}

/// Flow for subprograms, i.e. functions and methods
fn subprogram_flow(lex: &mut Lexer<Token>) -> Result<Function, ParseStop> {
    let path = match lex.next() {
        Some(Token::Path(path)) => Ok(path),
        Some(_) => Err(ParseStop::UnexpectedToken(lex.slice().to_string())),
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

    Ok(Function {
        path,
        args,
        body: get_body(lex)?,
    })
}

/// Gets condition which are multiple expression ending with a stray [Token::BraceRight] this consumes, based upon the [launch] function
fn get_body(lex: &mut Lexer<Token>) -> Result<Vec<Expr>, ParseStop> {
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
            Err(ParseStop::UnexpectedToken(d)) if &d == "}" => break,
            Err(unknown) => return Err(unknown),
        }
    }

    match buf {
        Some(expr) => output.push(expr),
        None => (),
    }

    Ok(output)
}

/// Gets condition which is a single expression ending with a stray [Token::BraceLeft] this consumes
fn get_condition(lex: &mut Lexer<Token>) -> Result<Expr, ParseStop> {
    let mut buf = None;

    loop {
        match next(lex, &mut buf, None, false) {
            Ok(expr) if buf.is_none() => buf = Some(expr),
            Ok(_) => break Err(ParseStop::MultipleExpressions),
            Err(ParseStop::UnexpectedToken(d)) if buf.is_some() && &d == "{" => {
                break Ok(buf.unwrap())
            }
            Err(unknown) => break Err(unknown),
        }
    }
}

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

    /// Shortcut for parsing next
    fn nparse(input: impl AsRef<str>) -> Expr {
        next(&mut Token::lexer(input.as_ref()), &mut None, None, true).unwrap()
    }

    #[test]
    fn while_loops() {
        assert_eq!(
            nparse("while true {}"),
            Expr {
                kind: While {
                    condition: Box::new(Expr {
                        kind: BoolLit(true).into(),
                        doc: None,
                        start: 6
                    }),
                    body: vec![]
                }
                .into(),
                doc: None,
                start: 0
            }
        );
        assert_eq!(
            nparse("while true { none }"),
            Expr {
                kind: While {
                    condition: Box::new(Expr {
                        kind: BoolLit(true).into(),
                        doc: None,
                        start: 6
                    }),
                    body: vec![Expr {
                        kind: ExprKind::None,
                        doc: None,
                        start: 13
                    }]
                }
                .into(),
                doc: None,
                start: 0
            }
        );
        assert_eq!(
            nparse("while 1+2 { none }"),
            Expr {
                kind: While {
                    condition: Box::new(Expr {
                        kind: Op {
                            left: Box::new(Expr {
                                kind: IntLit(1).into(),
                                doc: None,
                                start: 6
                            }),
                            right: Box::new(Expr {
                                kind: IntLit(2).into(),
                                doc: None,
                                start: 8
                            }),
                            kind: OpKind::Add
                        }
                        .into(),
                        doc: None,
                        start: 7
                    }),
                    body: vec![Expr {
                        kind: ExprKind::None,
                        doc: None,
                        start: 12
                    }]
                }
                .into(),
                doc: None,
                start: 0
            }
        );
        assert_eq!(
            nparse("while true { while true { none none } none }"),
            Expr {
                kind: While {
                    condition: Box::new(Expr {
                        kind: BoolLit(true).into(),
                        doc: None,
                        start: 6
                    }),
                    body: vec![
                        Expr {
                            kind: While {
                                condition: Box::new(Expr {
                                    kind: BoolLit(true).into(),
                                    doc: None,
                                    start: 19
                                }),
                                body: vec![
                                    Expr {
                                        kind: ExprKind::None,
                                        doc: None,
                                        start: 26
                                    },
                                    Expr {
                                        kind: ExprKind::None,
                                        doc: None,
                                        start: 31
                                    }
                                ]
                            }
                            .into(),
                            doc: None,
                            start: 13
                        },
                        Expr {
                            kind: ExprKind::None,
                            doc: None,
                            start: 38
                        }
                    ]
                }
                .into(),
                doc: None,
                start: 0
            }
        );
    }

    #[test]
    fn none() {
        assert_eq!(
            nparse("none"),
            Expr {
                kind: ExprKind::None,
                doc: None,
                start: 0
            }
        );
        assert_eq!(
            nparse("let mynone = none"),
            Expr {
                kind: ExprKind::Let(Let {
                    mutable: false,
                    path: Path::new("mynone"),
                    expr: Box::new(Expr {
                        kind: ExprKind::None,
                        doc: None,
                        start: 13
                    })
                }),
                doc: None,
                start: 0
            }
        );
    }

    #[test]
    fn lets() {
        assert_eq!(
            nparse("let x = 5"),
            Expr {
                kind: ExprKind::Let(Let {
                    mutable: false,
                    path: Path::new("x"),
                    expr: Box::new(Expr {
                        kind: IntLit(5).into(),
                        doc: None,
                        start: 8
                    })
                }),
                doc: None,
                start: 0
            }
        );
        assert_eq!(
            nparse("let mut x = 5"),
            Expr {
                kind: ExprKind::Let(Let {
                    mutable: true,
                    path: Path::new("x"),
                    expr: Box::new(Expr {
                        kind: IntLit(5).into(),
                        doc: None,
                        start: 12
                    })
                }),
                doc: None,
                start: 0
            }
        );
        assert_eq!(
            nparse(r#"let mut blah = "mut""#),
            Expr {
                kind: ExprKind::Let(Let {
                    mutable: true,
                    path: Path::new("blah"),
                    expr: Box::new(Expr {
                        kind: StrLit("mut".into()).into(),
                        doc: None,
                        start: 15
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
    fn bodies() {
        assert_eq!(
            get_body(&mut Token::lexer("\"hello\"}")),
            Ok(vec![Expr {
                kind: StrLit("hello".to_string()).into(),
                doc: None,
                start: 0
            }])
        );
        assert_eq!(
            get_body(&mut Token::lexer("56    + 3298}")),
            Ok(vec![Expr {
                kind: Op {
                    left: Box::new(Expr {
                        kind: IntLit(56).into(),
                        doc: None,
                        start: 0
                    }),
                    right: Box::new(Expr {
                        kind: IntLit(3298).into(),
                        doc: None,
                        start: 8
                    }),
                    kind: OpKind::Add
                }
                .into(),
                doc: None,
                start: 6
            }])
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
                            kind: ExprKind::CharLit(CharLit('c' as u32)),
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
                    start: 20,
                }),
                right: Box::new(Expr {
                    kind: IntLit(2).into(),
                    doc: None,
                    start: 25,
                }),
                kind: OpKind::Add,
            }
            .into(),
            doc: None,
            start: 23,
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

    #[test]
    fn classes() {
        let y = Expr {
            kind: ExprKind::Let(Let {
                path: Path::new("y"),
                mutable: true,
                expr: Box::new(Expr {
                    kind: IntLit(4).into(),
                    doc: None,
                    start: 62,
                }),
            }),
            doc: None,
            start: 50,
        };

        let other_thing = Expr {
            kind: ExprKind::Function(Function {
                path: Path::new("other_thing"),
                args: vec![Id("x".to_string())],
                body: vec![y],
            }),
            doc: None,
            start: 29,
        };

        let x = Expr {
            kind: ExprKind::Let(Let {
                path: Path::new("x"),
                mutable: false,
                expr: Box::new(Expr {
                    kind: IntLit(2).into(),
                    doc: None,
                    start: 27,
                }),
            }),
            doc: None,
            start: 19,
        };

        let hello_there = Expr {
            kind: ExprKind::Class(Class {
                id: Id("HelloThere".to_string()),
                body: vec![x, other_thing],
            }),
            doc: None,
            start: 0,
        };

        assert_eq!(
            nparse("class HelloThere { let x = 2 fun other_thing(x) { let mut y = 4 } }"),
            hello_there
        );
    }
}
