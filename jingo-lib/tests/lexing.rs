//! Tests lexing capabilities of [jingo_lib::frontend::lexer]-related parts.

use jingo_lib::frontend::lexer::{scan_code, Token, TokenType};

/// Default starting line for lexer
const DEFAULT_LINE: usize = 1;

/// Tests basic lexing capabilities.
#[test]
fn basic() {
    // simple plus
    assert_eq!(
        scan_code("+"),
        Ok(vec![Token::new(TokenType::Plus, DEFAULT_LINE)])
    );

    // plus and minus, check 2-length
    assert_eq!(
        scan_code("+-"),
        Ok(vec![
            Token::new(TokenType::Plus, DEFAULT_LINE),
            Token::new(TokenType::Minus, DEFAULT_LINE)
        ])
    );

    // check 1 length combined with 2-length
    assert_eq!(
        scan_code("-<=+"),
        Ok(vec![
            Token::new(TokenType::Minus, DEFAULT_LINE),
            Token::new(TokenType::LessEqual, DEFAULT_LINE),
            Token::new(TokenType::Plus, DEFAULT_LINE)
        ])
    );

    // 3 of the same chars
    assert_eq!(
        scan_code("..."),
        Ok(vec![
            Token::new(TokenType::Dot, DEFAULT_LINE),
            Token::new(TokenType::Dot, DEFAULT_LINE),
            Token::new(TokenType::Dot, DEFAULT_LINE)
        ])
    );
}

/// Ensures comments properly work and do not get mangled.
#[test]
fn comments() {
    let inputs = vec![
        " -------- --- -", // multiple minus/comment/doccomment
        "simple text",     // some simple text
        "+-<>--!.,",       // random 1-char symbols
        "!-",              // header comment test
    ];

    for input in inputs {
        // test simple `--` comment
        assert_eq!(
            scan_code(&format!("--{}", input)),
            Ok(vec![Token::new(TokenType::Comment, DEFAULT_LINE)])
        );

        // test `---` documentation comment
        assert_eq!(
            scan_code(&format!("---{}", input)),
            Ok(vec![Token::new(
                TokenType::DocComment(input.to_string()),
                DEFAULT_LINE
            )])
        );

        // test `-!-` header comment
        assert_eq!(
            scan_code(&format!("-!-{}", input)),
            Ok(vec![Token::new(
                TokenType::HeaderComment(input.to_string()),
                DEFAULT_LINE
            )])
        );
    }
}
