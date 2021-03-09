use super::{ast::*, lexer::Token};
use logos::Lexer;
use std::fmt;

/// Parsing-specific error/stop enumeration, encompassing the possible errors or
/// stops in parsing flow which may have occurred during parsing
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    /// Unexpected token
    UnexpectedToken,

    /// File ended unexpectedly
    UnexpectedEof,

    /// Mathematical operation was found with no left clause
    OpNoLeft,
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
            ParseError::UnexpectedEof => write!(f, "File ended unexpectedly"),
            ParseError::OpNoLeft => {
                write!(f, "Mathematical operation was found with no left clause")
            }
        }
    }
}

/// Gets next expression in series by recursing down the ways, a low-level getter
/// of the next full expression
fn next(
    lex: &mut Lexer<Token>,
    buf: &mut Option<Expr>,
    doc: Option<String>,
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
        Some(Token::True) => Ok(Expr::from_parse(BoolLit(true), doc, start)),
        Some(Token::False) => Ok(Expr::from_parse(BoolLit(false), doc, start)),
        Some(Token::Let) => Ok(Expr::from_parse(let_flow(lex)?, doc, start)),
        Some(Token::Return) => Ok(Expr::from_parse(Return(box_next(lex)?), doc, start)),
        Some(Token::Str(d)) => Ok(Expr::from_parse(StrLit(d), doc, start)),
        Some(Token::Char(d)) => Ok(Expr::from_parse(CharLit(d), doc, start)),
        Some(Token::Float(d)) => Ok(Expr::from_parse(FloatLit(d), doc, start)),
        Some(Token::Int(d)) => Ok(Expr::from_parse(IntLit(d), doc, start)),
        Some(Token::Id(_id)) => todo!("parse id or setlet"),
        Some(Token::Doc(string)) => next(lex, buf, Some(string)),
        Some(_) => Err(ParseError::UnexpectedToken),
        None => Err(ParseError::UnexpectedEof),
    }
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
            expr: box_next(lex)?,
        }),
        Some(Token::Id(id)) => Ok(Let {
            mutable: false,
            id: id.into(),
            expr: box_next(lex)?,
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
    Ok(Box::new(next(lex, &mut None, None)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use logos::Logos;

    #[test]
    fn lets() {
        assert_eq!(
            next(&mut Token::lexer("let x = 5"), &mut None, None).unwrap(),
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
}
