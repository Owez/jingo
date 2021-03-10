use super::{ast::*, lexer::Token};
use logos::Lexer;
use std::fmt;

/// Parsing-specific error/stop enumeration, encompassing the possible errors or
/// stops in parsing flow which may have occurred during parsing
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// Unexpected token
    UnexpectedToken,

    /// Unknown token whilst lexing
    UnknownToken,

    /// Mathematical operation was found with no left clause
    OpNoLeft,

    /// File ended unexpectedly
    UnexpectedEof,

    /// File ended expectedly
    FileEnded
}

impl<T> From<Option<T>> for ParseError {
    fn from(option: Option<T>) -> ParseError {
        match option {
            Some(_) => ParseError::UnexpectedToken,
            None => ParseError::UnexpectedEof,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken => write!(f, "Unexpected token"),
            ParseError::UnknownToken => write!(f, "Unknown token"),
            ParseError::OpNoLeft => {
                write!(f, "Mathematical operation was found with no left clause")
            },
            ParseError::UnexpectedEof=> write!(f, "File ended unexpectedly"),
            ParseError::FileEnded => write!(f, "File ended expectedly, please report this as a bug!"),
        }
    }
}

/// Parses a given lexer input into the resulting parsed values
pub fn launch(lex: &mut Lexer<Token>) -> Result<Vec<Expr>, ParseError> {
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
            },
            Err(ParseError::FileEnded) => break,
            Err(unknown) => return Err(unknown.into())
        }
    }

    match buf {
        Some(expr) => output.push(expr),
        None => ()
    }

    Ok(output)
}

/// Gets the next full expression, used internally as the main parsing hook
fn next(
    lex: &mut Lexer<Token>,
    buf: &mut Option<Expr>,
    doc: Option<String>,
    is_topmost: bool
) -> Result<Expr, ParseError> {
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
        Some(Token::Id(id)) => Ok(Expr::from_parse(id_flow(lex, Id(id))?, doc, start)),
        Some(Token::Doc(string)) => next(lex, buf, Some(string), is_topmost),
        Some(Token::Error) => Err(ParseError::UnknownToken),
        Some(_) => Err(ParseError::UnexpectedToken),
        None => Err(match is_topmost {
            true => ParseError::FileEnded,
            false => ParseError::UnexpectedEof
        }),
    }
}

/// Flow for all which begin with an identifier, such as [Id] or [FunctionCall]
fn id_flow(_lex: &mut Lexer<Token>, _id: Id) -> Result<ExprKind, ParseError> {
    todo!("id-based expressions")
}

/// Flow for operation grammar, i.e. adding or subtracting
fn op_flow(lex: &mut Lexer<Token>, buf: &mut Option<Expr>, kind: OpKind) -> Result<Op, ParseError> {
    Ok(Op {
        left: Box::new(buf.take().ok_or(ParseError::OpNoLeft)?),
        right: box_next(lex)?,
        kind,
    })
}

/// Flow for `let` grammar
fn let_flow(lex: &mut Lexer<Token>) -> Result<Let, ParseError> {
    match lex.next() {
        Some(Token::Mut) => Ok(Let {
            mutable: true,
            id: get_id(lex)?,
            expr: {ensure(lex, Token::Equals)?; box_next(lex)?},
        }),
        Some(Token::Id(id)) => Ok(Let {
            mutable: false,
            id: id.into(),
            expr: {ensure(lex, Token::Equals)?; box_next(lex)?},
        }),
        unknown => Err(unknown.into()),
    }
}

/// Gets id from next [Lexer] token or errors
fn get_id(lex: &mut Lexer<Token>) -> Result<Id, ParseError> {
    match lex.next() {
        Some(Token::Id(id)) => Ok(id.into()),
        unknown => Err(unknown.into()),
    }
}

/// Gets next expression without passing a previous `buf` of `doc` and returns a
/// [Box], used as a shortcut for sequential parsing
fn box_next(lex: &mut Lexer<Token>) -> Result<Box<Expr>, ParseError> {
    Ok(Box::new(next(lex, &mut None, None, false)?))
}

/// Ensures next lex token equals inputted `token` value
fn ensure(lex: &mut Lexer<Token>, token: Token) -> Result<(), ParseError> {
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
        assert_eq!(next(&mut Token::lexer("let x + 5"), &mut None, None, true), Err(ParseError::UnexpectedToken));
        assert_eq!(next(&mut Token::lexer("#"), &mut None, None, true), Err(ParseError::UnknownToken));
        assert_eq!(next(&mut Token::lexer("let x = -- 5"), &mut None, None, true), Err(ParseError::UnexpectedEof));
    }

    #[test]
    fn parse_launch() {
        assert_eq!(
            launch(&mut Token::lexer("5 + 3")).unwrap(), vec![Expr {
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
        assert_eq!(launch(&mut Token::lexer("!5")).unwrap(), vec![Expr {
            kind: ExprKind::Not(Not(Box::new(Expr {
                kind: ExprKind::IntLit(IntLit(5)),
                doc: None,
                start: 1
            }))),
            doc: None,start: 0
        }]);
        assert_eq!(launch(&mut Token::lexer("+ 5")), Err(ParseError::OpNoLeft));
        assert_eq!(launch(&mut Token::lexer("5 +")), Err(ParseError::UnexpectedEof));
        assert_eq!(launch(&mut Token::lexer("5 + 5 + 5 +")), Err(ParseError::UnexpectedEof));
    }
}
