#![allow(unexpected_cfgs)]
use braincrap_rs::parser::{BraincrapCommand, Parser};
use braincrap_rs::tokenizer::BraincrapToken;
use std::path::PathBuf;

#[test]
fn test_parse_basic_commands() {
    let tokens = vec![
        BraincrapToken::Plus,
        BraincrapToken::Minus,
        BraincrapToken::Right,
        BraincrapToken::Left,
        BraincrapToken::Dot,
        BraincrapToken::Comma,
        BraincrapToken::LeftBracket,
        BraincrapToken::RightBracket,
    ];

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
fn test_parse_macro_definition() {
    let tokens = vec![
        BraincrapToken::Hash,
        BraincrapToken::Char('a'),
        BraincrapToken::String("+".to_string()),
    ];

    let mut parser = Parser::new(&tokens, PathBuf::from("."));
    let commands = parser.parse();

    assert_eq!(
        commands,
        vec![BraincrapCommand::DefineMacro {
            name: 'a',
            tokens: vec![BraincrapToken::Plus],
            code: vec![BraincrapCommand::Addition],
        }]
    );
}

#[test]
fn test_parse_macro_execution() {
    let tokens = vec![BraincrapToken::Char('a'), BraincrapToken::Char('b')];

    let mut parser = Parser::new(&tokens, PathBuf::from("."));
    let commands = parser.parse();

    assert_eq!(
        commands,
        vec![
            BraincrapCommand::RunMacro { name: 'a' },
            BraincrapCommand::RunMacro { name: 'b' },
        ]
    );
}

#[test]
fn test_parse_import() {
    let tokens = vec![
        BraincrapToken::Dollar,
        BraincrapToken::String("file.bcf".to_string()),
    ];

    let mut parser = Parser::new(&tokens, PathBuf::from("."));
    let commands = parser.parse();

    assert_eq!(
        commands,
        vec![BraincrapCommand::Import {
            file: "file.bcf".to_string(),
            tokens: vec![],
            code: vec![],
        }]
    );
}

#[test]
fn test_parse_mixed_commands_and_macro() {
    let tokens = vec![
        BraincrapToken::Plus,
        BraincrapToken::Hash,
        BraincrapToken::Char('a'),
        BraincrapToken::String("+".to_string()),
        BraincrapToken::Char('b'),
        BraincrapToken::Dollar,
        BraincrapToken::String("file.bcf".to_string()),
    ];

    let mut parser = Parser::new(&tokens, PathBuf::from("."));
    let commands = parser.parse();

    assert_eq!(
        commands,
        vec![
            BraincrapCommand::Addition,
            BraincrapCommand::DefineMacro {
                name: 'a',
                tokens: vec![BraincrapToken::Plus],
                code: vec![BraincrapCommand::Addition],
            },
            BraincrapCommand::RunMacro { name: 'b' },
            BraincrapCommand::Import {
                file: "file.bcf".to_string(),
                tokens: vec![],
                code: vec![],
            }
        ]
    );
}

#[test]
fn test_parse_empty_input() {
    let tokens: Vec<BraincrapToken> = vec![];

    let mut parser = Parser::new(&tokens, PathBuf::from("."));
    let commands = parser.parse();

    assert_eq!(commands, Vec::<BraincrapCommand>::new());
}
