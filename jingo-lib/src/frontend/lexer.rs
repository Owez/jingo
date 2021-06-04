//! Lexer/scanner stage of parsing, the first main step to parse raw characters
//! into further parsable tokens

use super::ast::{Id, Path};
use logos::{Lexer, Logos};
use std::str::Chars;

/// Lexed token from [logos], encompassing all possible tokens
#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    // single-char
    #[token("(")]
    ParenLeft,
    #[token(")")]
    ParenRight,
    #[token("{")]
    BraceLeft,
    #[token("}")]
    BraceRight,
    #[token(",")]
    Comma,
    #[token("*")]
    Star,

    // math-only symbols
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("/")]
    FwdSlash,
    #[token("=")]
    Equals,
    #[token("==")]
    EqualsEquals,
    #[token("!")]
    Exclaim,
    #[token("!=")]
    ExclaimEquals,
    #[token("<")]
    Less,
    #[token("<=")]
    LessEquals,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEquals,

    // keywords
    #[token("if")]
    If,
    #[token("and")]
    And,
    #[token("or")]
    Or,
    #[token("else")]
    Else,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("none")]
    None,
    #[token("class")]
    Class,
    #[token("while")]
    While,
    #[token("return")]
    Return,
    #[token("self")]
    SelfRef,
    #[token("let")]
    Let,
    #[token("mut")]
    Mut,
    #[token("fun")]
    Fun,

    // literals
    #[regex(r#""(\\"|[^\n"])*""#, get_str)]
    Str(String),
    #[regex(r"'([^'\n]|\\(\\|n|r|t|b|f|v|0|x[0-9a-fA-F]+))'", get_char)]
    Char(u32),
    #[regex(r"[0-9]*\.[0-9]+", get_float)]
    Float(f64),
    #[regex(r"[0-9]+", get_int)]
    Int(i64),
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*(\.[a-zA-Z_][a-zA-Z0-9_]*)*", get_path)]
    Path(Path),

    // misc
    #[regex(r"---.*(\n---.*)*", get_doc)] // would be ---.*(\n+---.*)* but logos bug
    Doc(String),

    // special
    #[error]
    #[regex(r"[ \t\n\f]+|(--.*)", logos::skip)]
    Error,
}

fn get_str(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    let found = &slice[1..slice.len() - 1];

    (found != "\\").then_some(found.to_string())
}

fn get_char(lex: &mut Lexer<Token>) -> Option<u32> {
    let mut chars = lex.slice().chars();
    chars.next();

    match chars.next().unwrap() {
        '\\' => match chars.next().unwrap() {
            'n' => Some('\n' as u32),   // newline
            'r' => Some('\r' as u32),   // carriage return
            't' => Some('\t' as u32),   // tab
            'b' => Some('\x7f' as u32), // backspace
            'f' => Some('\x0C' as u32), // form feed
            '0' => Some('\0' as u32),
            'x' => {
                chars.next_back();
                Some(hex_to_u32(chars))
            } // hex
            _ => panic!(), // regex prevents
        },
        c => Some(c as u32), // normal
    }
}

/// Converts character iterator of hex digits into a [u32] value
fn hex_to_u32(chars: Chars) -> u32 {
    let mut res = 0;

    for (ind, c) in chars.rev().enumerate() {
        res += c.to_digit(16).unwrap() * 16u32.pow(ind as u32)
    }

    res
}

fn get_float(lex: &mut Lexer<Token>) -> Option<f64> {
    lex.slice().parse().ok()
}

fn get_path(lex: &mut Lexer<Token>) -> Path {
    let mut fields: Vec<Id> = lex
        .slice()
        .split('.')
        .map(|id| id.to_string().into())
        .collect();
    let id = fields.pop().unwrap();

    Path { fields, id }
}

fn get_int(lex: &mut Lexer<Token>) -> Option<i64> {
    lex.slice().parse().ok()
}

fn get_doc(lex: &mut Lexer<Token>) -> String {
    lex.slice()
        .split('\n')
        .filter(|l| l.len() != 0)
        .map(|l| l[3..].trim())
        .collect::<Vec<&str>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chars() {
        let mut lex = Token::lexer("'a' 'b' '\\n'");

        assert_eq!(lex.next().unwrap(), Token::Char('a' as u32));
        assert_eq!(lex.next().unwrap(), Token::Char('b' as u32));
        assert_eq!(lex.next().unwrap(), Token::Char('\n' as u32));
    }

    #[test]
    fn basic() {
        let mut lex = Token::lexer(
            "if true { false 0 1 0.01 }\nmy_id.one.five2 -- comment!\n--- docstring\ntrue",
        );

        assert_eq!(lex.next().unwrap(), Token::If);
        assert_eq!(lex.next().unwrap(), Token::True);
        assert_eq!(lex.next().unwrap(), Token::BraceLeft);
        assert_eq!(lex.next().unwrap(), Token::False);
        assert_eq!(lex.next().unwrap(), Token::Int(0));
        assert_eq!(lex.next().unwrap(), Token::Int(1));
        assert_eq!(lex.next().unwrap(), Token::Float(0.01));
        assert_eq!(lex.next().unwrap(), Token::BraceRight);
        assert_eq!(
            lex.next().unwrap(),
            Token::Path(Path {
                fields: vec!["my_id".into(), "one".into()],
                id: "five2".into()
            })
        );
        assert_eq!(lex.next().unwrap(), Token::Doc("docstring".to_string()));
        assert_eq!(lex.next().unwrap(), Token::True);
    }

    #[test]
    fn check_get_doc() {
        let mut lex = Token::lexer("--- hello\n---there\n---\n---  woo \n--- singleliner ---\n");

        lex.next(); // this should return same as below but tested in [docs]
        assert_eq!(
            get_doc(&mut lex),
            "hello\nthere\n\nwoo\nsingleliner ---".to_string()
        );
    }

    #[test]
    fn docs() {
        assert_eq!(
            Token::lexer("---hi there").next().unwrap(),
            Token::Doc("hi there".to_string())
        );
        assert_eq!(
            Token::lexer("---     hi there     ").next().unwrap(),
            Token::Doc("hi there".to_string())
        );
        assert_eq!(
            Token::lexer("---    hi there ---\n---   pretty cool eh?\n")
                .next()
                .unwrap(),
            Token::Doc("hi there ---\npretty cool eh?".to_string())
        );
    }

    #[test]
    fn strings() {
        assert_eq!(
            Token::lexer("\"hello there\"").next().unwrap(),
            Token::Str("hello there".to_string())
        );
        assert_eq!(Token::lexer("\"\\\"").next().unwrap(), Token::Error);
    }

    #[test]
    fn char_hex() {
        assert_eq!(hex_to_u32("F".chars()), 15);
        assert_eq!(hex_to_u32("A".chars()), 10);
        assert_eq!(hex_to_u32("0".chars()), 0);
        assert_eq!(hex_to_u32("FF".chars()), 255);
        assert_eq!(hex_to_u32("A039FBCF".chars()), 2688154575);
        assert_eq!(hex_to_u32("fe10ebca".chars()), 4262521802);

        let mut lex = Token::lexer(r#"'\xF' '\xA' '\0' '\xFF' '\xA039FBCF' '\xfe10ebca'"#);

        assert_eq!(lex.next().unwrap(), Token::Char(15));
        assert_eq!(lex.next().unwrap(), Token::Char(10));
        assert_eq!(lex.next().unwrap(), Token::Char(0));
        assert_eq!(lex.next().unwrap(), Token::Char(255));
        assert_eq!(lex.next().unwrap(), Token::Char(2688154575));
        assert_eq!(lex.next().unwrap(), Token::Char(4262521802));
    }
}
