//! Lexer/scanner stage of parsing, the first main step to parse raw characters
//! into further parsable tokens

use super::ast::{Id, OpKind, Path};
use logos::{Lexer, Logos};

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
    #[token("!")]
    Exclaim,
    #[token("_")]
    Interpret,
    #[token("*")]
    Star, // TODO: figure out pointers
    #[token("-")]
    Minus, // TODO: figure out negatives

    // multi-char
    #[token("=")]
    Equals,
    #[token("=>")]
    FatArrow,

    // operation symbols
    #[regex(r"\+|/|==|!=|<|<=|>|>=|and|or", get_op)]
    Op(OpKind),

    // keywords
    #[token("match")]
    Match,
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
    #[token("break")]
    Break,
    #[token("let")]
    Let,
    #[token("mut")]
    Mut,
    #[token("fun")]
    Fun,

    // literals
    #[regex(r#""(\\"|[^"])*""#, get_str)]
    Str(String),
    #[regex(r"'([^'\n]|\\(\\|n|r|t|b|f|v|0|x[0-9a-fA-F]+))'", get_char)]
    Char(u32),
    #[regex(r"[0-9]*\.[0-9]+", get_float)]
    Float(f64),
    #[regex(r"[0-9]+", get_int)]
    Int(i64),
    #[regex(r"\.?[a-zA-Z_][a-zA-Z0-9_]*(\.[a-zA-Z_][a-zA-Z0-9_]*)*", get_path)]
    Path(Path),

    // misc
    #[regex(r"---.*(\n---.*)*", get_doc)] // would be ---.*(\n+---.*)* but logos bug
    Doc(String),

    // special
    #[error]
    #[regex(r"[ \t\n\f]+|(--.*)", logos::skip)]
    Error,
}

fn get_op(lex: &mut Lexer<Token>) -> OpKind {
    match lex.slice() {
        "+" => OpKind::Plus,
        "-" => OpKind::Sub,
        "/" => OpKind::Div,
        "==" => OpKind::EqEq,
        "!=" => OpKind::NotEq,
        "<" => OpKind::Less,
        "<=" => OpKind::LessEq,
        ">" => OpKind::Greater,
        ">=" => OpKind::GreaterEq,
        "and" => OpKind::And,
        "or" => OpKind::Or,
        _ => panic!(), // regex prevents
    }
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
                hex_to_u32(chars.as_str(), 8)
            } // hex
            _ => panic!(), // regex prevents
        },
        c => Some(c as u32), // normal
    }
}

/// Converts character iterator of hex digits into a [u32] value
fn hex_to_u32(hex: &str, limit: usize) -> Option<u32> {
    (hex.len() <= limit).then_some(u32::from_str_radix(hex, 16).unwrap())
}

fn get_float(lex: &mut Lexer<Token>) -> Option<f64> {
    lex.slice().parse().ok()
}

fn get_path(lex: &mut Lexer<Token>) -> Path {
    let mut sliced = lex.slice();
    let affixed = if sliced.starts_with('.') {
        sliced = &sliced[1..];
        true
    } else {
        false
    };

    let mut fields: Vec<Id> = sliced.split('.').map(|id| id.to_string().into()).collect();
    let id = fields.pop().unwrap();

    Path {
        fields,
        id,
        affixed,
    }
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
            "match true { false 0 1 0.01 }\nmy_id.one.five2 -- comment!\n--- docstring\ntrue",
        );

        assert_eq!(lex.next().unwrap(), Token::Match);
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
                id: "five2".into(),
                affixed: false
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
        assert_eq!(hex_to_u32("F", 1).unwrap(), 15);
        assert_eq!(hex_to_u32("A", 1).unwrap(), 10);
        assert_eq!(hex_to_u32("0", 1).unwrap(), 0);
        assert_eq!(hex_to_u32("FF", 8).unwrap(), 255);
        assert_eq!(hex_to_u32("A039FBCF", 8).unwrap(), 2688154575);
        assert_eq!(hex_to_u32("fe10ebca", 8).unwrap(), 4262521802);
        assert_eq!(hex_to_u32("FFF", 3), Some(4095));
        assert_eq!(hex_to_u32("FFFF", 3), None);
        assert_eq!(hex_to_u32("0000", 3), None);
        assert_eq!(hex_to_u32("FFFFF", 3), None);
        assert_eq!(hex_to_u32("00000", 3), None);

        let mut lex = Token::lexer(r#"'\xF' '\xA' '\0' '\xFF' '\xA0CF' '\xfe10'"#);

        assert_eq!(lex.next().unwrap(), Token::Char(15));
        assert_eq!(lex.next().unwrap(), Token::Char(10));
        assert_eq!(lex.next().unwrap(), Token::Char(0));
        assert_eq!(lex.next().unwrap(), Token::Char(255));
        assert_eq!(lex.next().unwrap(), Token::Char(41167));
        assert_eq!(lex.next().unwrap(), Token::Char(65040));
    }

    #[test]
    fn pathing() {
        // eq
        assert_eq!(
            Token::lexer("hello_world").next().unwrap(),
            Token::Path(Path::new("hello_world"))
        );
        assert_eq!(
            Token::lexer("c.c").next().unwrap(),
            Token::Path(Path {
                id: "c".into(),
                fields: vec!["c".into()],
                affixed: false
            })
        );
        assert_eq!(
            Token::lexer("c").next().unwrap(),
            Token::Path(Path {
                id: "c".into(),
                fields: vec![],
                affixed: false
            })
        );
        assert_eq!(
            Token::lexer("a.b.c").next().unwrap(),
            Token::Path(Path {
                id: "c".into(),
                fields: vec!["a".into(), "b".into()],
                affixed: false
            })
        );
        assert_eq!(
            Token::lexer(".a.b.c").next().unwrap(),
            Token::Path(Path {
                id: "c".into(),
                fields: vec!["a".into(), "b".into()],
                affixed: true
            })
        );

        // ne
        assert_ne!(
            Token::lexer("c..c").next().unwrap(),
            Token::Path(Path {
                id: "c".into(),
                fields: vec!["c".into()],
                affixed: false
            })
        );
        assert_ne!(
            Token::lexer("..c.c").next().unwrap(),
            Token::Path(Path {
                id: "c".into(),
                fields: vec!["c".into()],
                affixed: true
            })
        );
        assert_ne!(
            Token::lexer("..c..c").next().unwrap(),
            Token::Path(Path {
                id: "c".into(),
                fields: vec!["c".into()],
                affixed: true
            })
        );
    }
}
