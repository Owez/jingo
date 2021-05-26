//! Parser runner

use crate::utils::{help_exit, msg_exit, open_file};
use crate::{FilePos, Parsed};
use jingo_lib::frontend::{lexer::Token, parser};
use logos::Logos;
use std::path::PathBuf;

/// Runs parsing steps
pub fn launch(parsed: Parsed) {
    if parsed.data.len() == 0 {
        help_exit("No files passed for parsing")
    } else if parsed.data.len() > 1 {
        help_exit("More then one file passed for parsing")
    }

    let path = PathBuf::from(parsed.data[0].clone());
    let input = &open_file(path.clone());

    let mut lex = Token::lexer(input);

    match parser::launch(&mut lex) {
        Ok(parsed) => println!("Parsed result ↴\n{:#?}", parsed),
        Err(err) => msg_exit(format!(
            "Error in {}\n  Whilst parsing → {}",
            FilePos::new(path, input, lex.span().start).unwrap(),
            err
        )),
    }
}
