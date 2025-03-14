#![allow(unexpected_cfgs)]
use braincrap_rs::parser::BraincrapCommand;
use braincrap_rs::parser::Parser;
use braincrap_rs::tokenizer::{BraincrapToken, Lexer};
use std::path::PathBuf;

#[test]
fn test_basic_braincrap_parsing() {
    let input = "+-><.,[]"; // Braincrap code
    let mut lexer = Lexer::new(input.to_string());
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(&tokens, PathBuf::from("."));
    let commands = parser.parse();

    assert_eq!(
        commands,
        vec![
            BraincrapCommand::Addition,
            BraincrapCommand::Substraction,
            BraincrapCommand::MoveRight,
            BraincrapCommand::MoveLeft,
            BraincrapCommand::Output,
            BraincrapCommand::Input,
            BraincrapCommand::OpenLoop,
            BraincrapCommand::CloseLoop,
        ]
    );
}

#[test]
fn test_macro_parsing() {
    let input = "#m ++--\n";
    let mut lexer = Lexer::new(input.to_string());
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(&tokens, PathBuf::from("."));
    let commands = parser.parse();

    assert_eq!(
        commands,
        vec![BraincrapCommand::DefineMacro {
            name: 'm',
            tokens: vec![
                BraincrapToken::Plus,
                BraincrapToken::Plus,
                BraincrapToken::Minus,
                BraincrapToken::Minus
            ],
            code: vec![
                BraincrapCommand::Addition,
                BraincrapCommand::Addition,
                BraincrapCommand::Substraction,
                BraincrapCommand::Substraction
            ]
        }]
    );
}

#[test]
fn test_run_macro() {
    let input = "m";
    let mut lexer = Lexer::new(input.to_string());
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(&tokens, PathBuf::from("."));
    let commands = parser.parse();

    assert_eq!(commands, vec![BraincrapCommand::RunMacro { name: 'm' }]);
}
