#![allow(unexpected_cfgs)]
use braincrap_rs::tokenizer::{BraincrapToken, Lexer};

#[test]
fn test_tokenize_basic_symbols() {
    let input = "+-><.,[]";
    let mut lexer = Lexer::new(input.to_string());
    let tokens = lexer.tokenize();

    assert_eq!(
        tokens,
        vec![
            BraincrapToken::Plus,
            BraincrapToken::Minus,
            BraincrapToken::Right,
            BraincrapToken::Left,
            BraincrapToken::Dot,
            BraincrapToken::Comma,
            BraincrapToken::LeftBracket,
            BraincrapToken::RightBracket,
        ]
    );
}

#[test]
fn test_tokenize_macro_and_string() {
    let input = "#a ....\n$filename.txt";
    let mut lexer = Lexer::new(input.to_string());
    let tokens = lexer.tokenize();

    assert_eq!(
        tokens,
        vec![
            BraincrapToken::Hash,
            BraincrapToken::Char('a'),
            BraincrapToken::String("....".to_string()),
            BraincrapToken::Dollar,
            BraincrapToken::String("filename.txt".to_string()),
        ]
    );
}

#[test]
fn test_tokenize_illegal_macro() {
    let input = "#> illegal_macro";
    let mut lexer = Lexer::new(input.to_string());
    let tokens = lexer.tokenize();

    assert_eq!(
        tokens,
        vec![
            BraincrapToken::Hash,
            BraincrapToken::Char('e'), // Default char for illegal macro names
            BraincrapToken::String("illegal_macro".to_string())
        ]
    );
}

#[test]
fn test_tokenize_empty_input() {
    let input = "";
    let mut lexer = Lexer::new(input.to_string());
    let tokens = lexer.tokenize();

    assert_eq!(tokens, Vec::<BraincrapToken>::new());
}

#[test]
fn test_tokenize_whitespace_and_comments() {
    let input = "   ; This is a comment\n + -";
    let mut lexer = Lexer::new(input.to_string());
    let tokens = lexer.tokenize();

    assert_eq!(tokens, vec![BraincrapToken::Plus, BraincrapToken::Minus,]);
}
