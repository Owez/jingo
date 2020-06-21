//! Frontend activities regarding Jingo, such as lexing, parsing and the ast.
//!
//! This is the first major stage in this compiler before it's handed over to the
//! [crate::backend] module for codegen and the final stages.
//! 
//! ## Process
//! 
//! General flow of frontend components before moving to backend for code
//! generation:
//! 
//! [lexer] -> [parser] -> [ast]

pub mod ast;
pub mod lexer;
pub mod parser;
