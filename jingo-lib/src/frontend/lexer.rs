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
    #[token("loop")]
    Loop,
    #[token("while")]
    While,
    #[token("return")]
    Return,
    #[token("this")]
    This,
    #[token("let")]
    Let,
    #[token("mut")]
    Mut,
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
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*(::[a-zA-Z_][a-zA-Z0-9_]*)+", get_path)]
    Path(Vec<String>),

    // misc
    #[regex(r"---.*(\n---.*)*", get_doc)] // would be ---.*(\n+---.*)* but logos bug
    Doc(String),

    // special
    #[error]
    #[regex(r"[ \t\n\f]+|(--.*)", logos::skip)]
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


fn get_id(lex: &mut Lexer<Token>) -> String {
    lex.slice().to_string()
}
fn get_path(lex: &mut Lexer<Token>) -> Vec<String> {
    lex.slice().split("::").map(|id| id.to_string()).collect()
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
    fn basic() {
        let mut lex =
            Token::lexer("if true { false 0 1 0.01 }\nmy_id -- comment!\n--- docstring\ntrue");

        assert_eq!(lex.next().unwrap(), Token::If);
        assert_eq!(lex.next().unwrap(), Token::True);
        assert_eq!(lex.next().unwrap(), Token::BraceLeft);
        assert_eq!(lex.next().unwrap(), Token::False);
        assert_eq!(lex.next().unwrap(), Token::Int(0));
        assert_eq!(lex.next().unwrap(), Token::Int(1));
        assert_eq!(lex.next().unwrap(), Token::Float(0.01));
        assert_eq!(lex.next().unwrap(), Token::BraceRight);
        assert_eq!(lex.next().unwrap(), Token::Id("my_id".to_string()));
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
}
