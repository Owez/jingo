//! Lexer runner

use crate::utils::{help_exit, msg_exit, open_file};
use crate::{FilePos, Parsed};
use jingo_lib::frontend::lexer::Token;
use logos::Logos;
use std::path::PathBuf;

/// Runs lexing steps
pub fn launch(parsed: Parsed) {
    if parsed.data.len() == 0 {
        help_exit("No files passed for lexing")
    } else if parsed.data.len() > 1 {
        help_exit("More then one file passed for lexing")
    }

    let path = PathBuf::from(parsed.data[0].clone());
    let input = &open_file(path.clone());

    let mut lex = Token::lexer(input);
    let mut output = vec![];

    loop {
        // seperate loop in order to print all at once for error consistancy
        match lex.next() {
            Some(Token::Error) => msg_exit(format!(
                "Error in {} ↴\n  Unknown token whilst lexing → {}",
                FilePos::new(path, input, lex.span().start).unwrap(),
                lex.slice()
            )),
            Some(token) => output.push((token, lex.span())),
            None => break,
        }
    }

    println!("Lexed result ↴");

    for (token, span) in output {
        println!("- {:?} @ {:?}", token, span);
    }
}
