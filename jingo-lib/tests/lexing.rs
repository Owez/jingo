//! Tests lexing capabilities of [jingo_lib::frontend::lexer]-related parts.

use jingo_lib::error::{JingoError, ScanningError};
use jingo_lib::frontend::lexer::{scan_code, Token, TokenType};

/// Default starting line for lexer
const DEFAULT_LINE: usize = 1;

/// Tests basic lexing capabilities.
#[test]
fn basic_operators() {
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

/// Ensures comments properly add increment the `line` for a token like other
/// `\n`s would (due to the nature of some docstrings failing).
#[test]
fn comments_newlines() {
    let input = "-!-Header comment\n---Docstring comment\n--Normal comment";

    assert_eq!(
        scan_code(input),
        Ok(vec![
            Token::new(
                TokenType::HeaderComment("Header comment".to_string()),
                DEFAULT_LINE
            ),
            Token::new(
                TokenType::DocComment("Docstring comment".to_string()),
                DEFAULT_LINE + 1
            ),
            Token::new(TokenType::Comment, DEFAULT_LINE + 2),
        ])
    )
}

/// Basic functionality checks on string literals
#[test]
fn string_literal() {
    // basic string test
    assert_eq!(
        scan_code("\"test\""),
        Ok(vec![Token::new(
            TokenType::StringLit("test".to_string()),
            DEFAULT_LINE
        )])
    );

    // unclosed string - should error
    assert_eq!(
        scan_code("\"unclosed string"),
        Err(JingoError::ScanningError(
            ScanningError::UnterminatedString(DEFAULT_LINE)
        ))
    );
}

/// Tests basic floats.
#[test]
fn floats() {
    // semi-long float
    assert_eq!(
        scan_code("2332.23213"),
        Ok(vec![Token::new(
            TokenType::FloatLit(2332.23213),
            DEFAULT_LINE
        )])
    );

    // normal float
    assert_eq!(
        scan_code("3424.2"),
        Ok(vec![Token::new(TokenType::FloatLit(3424.2), DEFAULT_LINE)])
    );

    // ensure normal float isnt int
    assert_eq!(
        scan_code("22.2"),
        Ok(vec![Token::new(TokenType::FloatLit(22.2), DEFAULT_LINE)])
    );

    // ensure .0 doesn't become int
    assert_eq!(
        scan_code("10.0"),
        Ok(vec![Token::new(TokenType::FloatLit(10.0), DEFAULT_LINE)])
    );
}

/// ensures numbers can handle newlines and others properly.
#[test]
fn floats_newlines() {
    // basic "clear" test
    assert_eq!(
        scan_code("23.3\n56.2223\n\n\n11.2"),
        Ok(vec![
            Token::new(TokenType::FloatLit(23.3), DEFAULT_LINE),
            Token::new(TokenType::FloatLit(56.2223), DEFAULT_LINE + 1),
            Token::new(TokenType::FloatLit(11.2), DEFAULT_LINE + 4),
        ])
    );

    // with comment between
    assert_eq!(
        scan_code("23.3\n\n\n-- testing newlines\n\n56.2223"),
        Ok(vec![
            Token::new(TokenType::FloatLit(23.3), DEFAULT_LINE),
            Token::new(TokenType::Comment, DEFAULT_LINE + 3),
            Token::new(TokenType::FloatLit(56.2223), DEFAULT_LINE + 5),
        ])
    );
}

/// tests whole numbers (integers).
#[test]
fn numbers() {
    // basic 1, 2, 3
    assert_eq!(
        scan_code("1 2 3"),
        Ok(vec![
            Token::new(TokenType::NumLit(1), DEFAULT_LINE),
            Token::new(TokenType::NumLit(2), DEFAULT_LINE),
            Token::new(TokenType::NumLit(3), DEFAULT_LINE),
        ])
    );

    // longer numbers
    assert_eq!(
        scan_code("342424 23423424"),
        Ok(vec![
            Token::new(TokenType::NumLit(342424), DEFAULT_LINE),
            Token::new(TokenType::NumLit(23423424), DEFAULT_LINE),
        ])
    );

    // max int size
    assert_eq!(
        scan_code("9223372036854775807"),
        Ok(vec![Token::new(
            TokenType::NumLit(9223372036854775807),
            DEFAULT_LINE
        )])
    );

    // min int size
    assert_eq!(
        scan_code("-9223372036854775807"),
        Ok(vec![
            Token::new(TokenType::Minus, DEFAULT_LINE),
            Token::new(TokenType::NumLit(9223372036854775807), DEFAULT_LINE)
        ])
    );
}

/// Tests newlines capabilities of whole numbers/ints.
#[test]
fn numbers_newlines() {
    assert_eq!(
        scan_code("32432\n9999999999\n-- Hi\n384348539851"),
        Ok(vec![
            Token::new(TokenType::NumLit(32432), DEFAULT_LINE),
            Token::new(TokenType::NumLit(9999999999), DEFAULT_LINE + 1),
            Token::new(TokenType::Comment, DEFAULT_LINE + 2),
            Token::new(TokenType::NumLit(384348539851), DEFAULT_LINE + 3),
        ])
    )
}

/// Basic string functionality test.
#[test]
fn strings() {
    // empty string
    assert_eq!(
        scan_code("\"\""),
        Ok(vec![Token::new(
            TokenType::StringLit(String::new()),
            DEFAULT_LINE
        )])
    );

    // basic "hi there"
    assert_eq!(
        scan_code("\"Hi there\""),
        Ok(vec![Token::new(
            TokenType::StringLit("Hi there".to_string()),
            DEFAULT_LINE
        )])
    );
}

/// Tests valid string escape sequences.
#[test]
fn strings_escapes() {
    // newline
    assert_eq!(
        scan_code("\"\n\""),
        Ok(vec![Token::new(
            TokenType::StringLit("\n".to_string()),
            DEFAULT_LINE
        )])
    );

    // tab
    assert_eq!(
        scan_code("\"\t\""),
        Ok(vec![Token::new(
            TokenType::StringLit("\t".to_string()),
            DEFAULT_LINE
        )])
    );

    // `\r`
    assert_eq!(
        scan_code("\"\r\""),
        Ok(vec![Token::new(
            TokenType::StringLit("\r".to_string()),
            DEFAULT_LINE
        )])
    );

    // multiple \n\n\t
    assert_eq!(
        scan_code("\"\n\n\t\""),
        Ok(vec![Token::new(
            TokenType::StringLit("\n\n\t".to_string()),
            DEFAULT_LINE
        )])
    );
}

/// Tests specifically [backslash][quote] (e.g. `\"`) inside a string for common
/// edge case. Gets very ugly with rust, for example `\"\\\"\"` in rust is really
/// `"\""` in normal text
#[test]
fn strings_escapes_string() {
    assert_eq!(
        scan_code("\"\\\"\""),
        Ok(vec![Token::new(
            TokenType::StringLit("\"".to_string()),
            DEFAULT_LINE
        )])
    );

    scan_code("\"\\\"\\\"\"").unwrap(); // normal: `"\"\""`
    scan_code("\"\\\\\\\"\"").unwrap(); // normal: `"\\\""` (i think)
}
