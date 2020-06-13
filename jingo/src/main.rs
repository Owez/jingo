//! The CLI for Jingo (code for the `./jingo [file]` program), if you are looking
//! for the main documentation, [go here](https://github.com/scOwez/jingo).

mod log;
mod cli;

use cli::{CLIResult, parse_args};

fn main() {
    match parse_args() {
        CLIResult::Error(e) => log::error(e),
        CLIResult::Direct(_, _) => log::error("Direct todo".to_string()),
        CLIResult::File(_, _) => log::error("File todo".to_string()),
        _ => ()
    }
}
