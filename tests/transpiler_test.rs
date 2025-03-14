#![allow(unexpected_cfgs)]
use braincrap_rs::parser::BraincrapCommand;
use braincrap_rs::transpiler::Transpiler;

#[test]
fn test_transpile_basic_commands() {
    let commands = vec![
        BraincrapCommand::Addition,
        BraincrapCommand::Substraction,
        BraincrapCommand::MoveLeft,
        BraincrapCommand::MoveRight,
        BraincrapCommand::OpenLoop,
        BraincrapCommand::CloseLoop,
        BraincrapCommand::Output,
        BraincrapCommand::Input,
    ];

    let mut transpiler = Transpiler::new();
    let result = transpiler.transpile(commands);

    assert_eq!(result, "+-<>[].,");
}

#[test]
fn test_transpile_macro_definition_and_run() {
    let commands = vec![
        BraincrapCommand::DefineMacro {
            name: 'a',
            tokens: vec![],
            code: vec![BraincrapCommand::Addition, BraincrapCommand::MoveRight],
        },
        BraincrapCommand::RunMacro { name: 'a' },
        BraincrapCommand::RunMacro { name: 'a' },
    ];

    let mut transpiler = Transpiler::new();
    let result = transpiler.transpile(commands);

    assert_eq!(result, "+>+>");
}

#[test]
fn test_transpile_macro_without_definition() {
    let commands = vec![
        BraincrapCommand::RunMacro { name: 'b' }, // Undefined macro 'b'
    ];

    let mut transpiler = Transpiler::new();
    let result = transpiler.transpile(commands);

    assert_eq!(result, ""); // No output since macro 'b' is not defined
}

#[test]
fn test_transpile_macro_with_redefinition() {
    let commands = vec![
        BraincrapCommand::DefineMacro {
            name: 'a',
            tokens: vec![],
            code: vec![BraincrapCommand::Addition],
        },
        BraincrapCommand::RunMacro { name: 'a' },
        BraincrapCommand::DefineMacro {
            name: 'a',
            tokens: vec![],
            code: vec![BraincrapCommand::Substraction],
        },
        BraincrapCommand::RunMacro { name: 'a' },
    ];

    let mut transpiler = Transpiler::new();
    let result = transpiler.transpile(commands);

    assert_eq!(result, "+-");
}

#[test]
fn test_transpile_import() {
    let commands = vec![BraincrapCommand::Import {
        file: "other.bcf".to_string(),
        tokens: vec![], // Assuming empty tokens for simplicity
        code: vec![BraincrapCommand::Addition, BraincrapCommand::MoveLeft],
    }];

    let mut transpiler = Transpiler::new();
    let result = transpiler.transpile(commands);

    assert_eq!(result, "+<"); // Import from another file results in "+<"
}

#[test]
fn test_transpile_empty_input() {
    let commands: Vec<BraincrapCommand> = vec![];

    let mut transpiler = Transpiler::new();
    let result = transpiler.transpile(commands);

    assert_eq!(result, ""); // No commands to transpile
}
