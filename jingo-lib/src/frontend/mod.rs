//! Frontend activities regarding Jingo, such as lexing, parsing and the ast.
//!
//! This is the first major stage in this compiler before it's handed over to the
//! [crate::backend] module for codegen and the final stages.

pub mod lexer;
