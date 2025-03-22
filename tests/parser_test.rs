#![allow(unexpected_cfgs)]
use braincrap_rs::parser::{BraincrapCommand, Parser};
use braincrap_rs::tokenizer::BraincrapToken;
use std::path::PathBuf;

#[test]
fn test_parse_basic_commands() {
    let tokens = vec![
        BraincrapToken::Plus(1),
        BraincrapToken::Minus(1),
        BraincrapToken::Right(1),
        BraincrapToken::Left(1),
        BraincrapToken::Dot(1),
        BraincrapToken::Comma(1),
        BraincrapToken::LeftBracket,
        BraincrapToken::RightBracket,
    ];

    let mut parser = Parser::new(&tokens, PathBuf::from("."));
    let commands = parser.parse();

    assert_eq!(
        commands,
        vec![
            BraincrapCommand::Addition(1),
            BraincrapCommand::Substraction(1),
            BraincrapCommand::MoveRight(1),
            BraincrapCommand::MoveLeft(1),
            BraincrapCommand::Output(1),
            BraincrapCommand::Input(1),
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
            tokens: vec![BraincrapToken::Plus(1)],
            code: vec![BraincrapCommand::Addition(1)],
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
        BraincrapToken::Plus(1),
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
            BraincrapCommand::Addition(1),
            BraincrapCommand::DefineMacro {
                name: 'a',
                tokens: vec![BraincrapToken::Plus(1)],
                code: vec![BraincrapCommand::Addition(1)],
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
