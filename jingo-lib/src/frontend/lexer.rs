//! Lexer/scanner stage of parsing, the first main step to parse raw characters
//! into further parsable tokens

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
    #[token(".")]
    Dot,
    #[token(";")]
    Semicolon,
    #[token("/")]
    FwdSlash,
    #[token("*")]
    Star,

    // multi-char
    #[token("::")]
    Static,

    // math-only symbols
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
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
    #[token("loop")]
    Loop,
    #[token("while")]
    While,
    #[token("return")]
    Return,
    #[token("this")]
    This,
    #[token("var")]
    Var,
    #[token("fun")]
    Fun,

    // literals
    #[regex(r#"".*""#, get_str)]
    Str(String),
    #[regex(r"'(\\t|\\r|\\n|\\'|[^'])'", get_char)]
    Char(char),
    #[regex(r"[0-9]*\.[0-9]+", get_float)]
    Float(f64),
    #[regex(r"[0-9]+", get_int)]
    Int(i64),
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", get_id)]
    Id(String),

    // misc
    #[regex(r"---.*(\n---.*)*", get_doc)]
    Doc(String),

    // special
    #[error]
    #[regex(r"[ \t\n\f]+|--.*", logos::skip)]
    Error,
}

fn get_str(lex: &mut Lexer<Token>) -> String {
    let slice = lex.slice();
    slice[1..slice.len() - 1].to_string()
}

fn get_char(lex: &mut Lexer<Token>) -> Option<char> {
    lex.slice().parse().ok()
}

fn get_float(lex: &mut Lexer<Token>) -> Option<f64> {
    lex.slice().parse().ok()
}

fn get_int(lex: &mut Lexer<Token>) -> Option<i64> {
    lex.slice().parse().ok()
}

fn get_id(lex: &mut Lexer<Token>) -> String {
    lex.slice().to_string()
}

fn get_doc(lex: &mut Lexer<Token>) -> String {
    lex.slice()
        .split('\n')
        .map(|s| s[3..].trim())
        .collect::<Vec<&str>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
