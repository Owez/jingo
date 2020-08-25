//! Parser for Jingo, makes [crate::frontend::lexer] into an AST
//! (abstract-syntax-tree), the second main stage of a compiler.
//!
//! See [crate::frontend::ast] for the AST classes.

use crate::error::{JingoError, ParsingError};
use crate::frontend::ast::*;
use crate::frontend::lexer::{Token, TokenType};

/// Primary parsing core, contains shared data for a parser instance that is
/// used when calling [Parser::parse]
pub struct Parser {
    /// Inputted tokens that may be found from [crate::lexer::scan_code]
    pub tokens: Vec<Token>,

    /// Internal mesurement on currently accessed index of [Parser::tokens]
    cur_token: usize,
}

impl Parser {
    /// Shortcut to making a new parser instance
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            cur_token: 0,
        }
    }

    /// Parses inputted tokens from [Parser::new] into a structured AST
    pub fn parse(&self) -> Result<(), JingoError> {
        // TODO: hook up & figure out return type
        Err(JingoError::Unimplemented(Some(String::from(
            "The parser is not complete",
        ))))
    }

    /// Gets token before [Parser::cur_token] or returns error if couldn't retreive
    fn get_previous(&self) -> &Token {
        &self.tokens[self.cur_token - 1]
    }

    /// Checks if next token matches given [TokenType]s. If it does, march one
    /// [Parser::cur_token] onwards
    fn matches(&self, types: Vec<TokenType>) -> bool {
        unimplemented!()
    }

    /// Underlying top-level parser function for checking expressions and forming
    /// them into rigid [ExpressionNode]s
    fn expression(&self) -> ExpressionNode {
        self.binop()
    }

    /// Cycles through sub-functions related to [crate::frontend::ast::BinOp]
    fn binop(&self) -> ExpressionNode {
        self.equality()
    }

    /// Checks equality operators for [binop] (e.g. `==` and `!=`)
    fn equality(&self) -> ExpressionNode {
        let mut expr = self.expression();

        while self.matches(vec![TokenType::EqualEqual, TokenType::NotEqual]) {
            let operator = match self.get_previous().token_type {
                TokenType::NotEqual => BinOp::NotEqual,
                TokenType::EqualEqual => BinOp::EqualEqual,
                _ => panic!("Token isn't the same as just found, this shouldn't happen!"), // shouldn't happen
            };
            let right = self.expression();

            expr = ExpressionNode::BinOp(Box::new(expr), operator, Box::new(right))
        }

        expr
    }
}
