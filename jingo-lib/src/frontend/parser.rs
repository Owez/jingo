use super::{ast::*, lexer::Token};
use logos::Lexer;
use std::fmt;

/// Parsing-specific error/stop enumeration, encompassing the possible errors or
/// stops in parsing flow which may have occurred during parsing
#[derive(Debug, Clone, PartialEq)]
pub enum ParseStop {
    //
    // errors
    //
    /// Unexpected token
    UnexpectedToken,

    /// Unknown token whilst lexing
    UnknownToken,

    /// Operation was found with no lefthand expression
    NoLeftExpr,

    /// File ended unexpectedly
    UnexpectedEof,

    //
    // special
    //
    /// File ended expectedly
    FileEnded,
}

impl<T> From<Option<T>> for ParseStop {
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
            ParseStop::UnknownToken => write!(f, "Unknown token"),
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
        Some(Token::Id(id)) => Ok(Expr::from_parse(path_flow(lex, vec![id])?, doc, start)),
        Some(Token::Doc(string)) => next(lex, buf, Some(string), is_topmost),
        Some(Token::Error) => Err(ParseStop::UnknownToken),
        Some(_) => Err(ParseStop::UnexpectedToken),
        None => Err(match is_topmost {
            true => ParseStop::FileEnded,
            false => ParseStop::UnexpectedEof,
        }),
    }
}

/// Path flow for all [Token::Path] or [Token::Id]
fn path_flow(_lex: &mut Lexer<Token>, _path: Vec<String>) -> Result<ExprKind, ParseStop> {
    todo!("path/id flow")
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
    match lex.next() {
        Some(Token::Mut) => Ok(Let {
            mutable: true,
            id: get_id(lex)?,
            expr: {
                ensure(lex, Token::Equals)?;
                box_next(lex)?
            },
        }),
        Some(Token::Id(id)) => Ok(Let {
            mutable: false,
            id: id.into(),
            expr: {
                ensure(lex, Token::Equals)?;
                box_next(lex)?
            },
        }),
        unknown => Err(unknown.into()),
    }
}

/// Gets id from next [Lexer] token or errors
fn get_id(lex: &mut Lexer<Token>) -> Result<Id, ParseStop> {
    match lex.next() {
        Some(Token::Id(id)) => Ok(id.into()),
        unknown => Err(unknown.into()),
    }
}

/// Gets next expression without passing a previous `buf` of `doc` and returns a
/// [Box], used as a shortcut for sequential parsing
fn box_next(lex: &mut Lexer<Token>) -> Result<Box<Expr>, ParseStop> {
    Ok(Box::new(next(lex, &mut None, None, false)?))
}

/// Ensures next lex token equals inputted `token` value
fn ensure(lex: &mut Lexer<Token>, token: Token) -> Result<(), ParseStop> {
    let next = lex.next();

    if next == Some(token) {
        Ok(())
    } else {
        Err(next.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use logos::Logos;

    #[test]
    fn lets() {
        assert_eq!(
            next(&mut Token::lexer("let x = 5"), &mut None, None, true).unwrap(),
            Expr {
                kind: ExprKind::Let(Let {
                    mutable: false,
                    id: Id("x".to_string()),
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
            Err(ParseStop::UnexpectedToken)
        );
        assert_eq!(
            next(&mut Token::lexer("#"), &mut None, None, true),
            Err(ParseStop::UnknownToken)
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
}
