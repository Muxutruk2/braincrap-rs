#![allow(unexpected_cfgs)]
use braincrap_rs::parser::BraincrapCommand;
use braincrap_rs::transpiler::{Transpiler, TranspilerArguments};

#[test]
fn test_transpile_basic_commands() {
    let commands = vec![
        BraincrapCommand::Addition(1),
        BraincrapCommand::Substraction(1),
        BraincrapCommand::MoveLeft(1),
        BraincrapCommand::MoveRight(1),
        BraincrapCommand::OpenLoop,
        BraincrapCommand::CloseLoop,
        BraincrapCommand::Output(1),
        BraincrapCommand::Input(1),
    ];

    let mut transpiler = Transpiler::new();
    let bf_result = transpiler.transpile(commands.clone(), &TranspilerArguments::Brainfuck);
    let c_result = transpiler.transpile(commands, &TranspilerArguments::C);

    assert_eq!(c_result, "(*ptr += 1);(*ptr -= 1);(ptr -= 1);(ptr += 1);while(*ptr != 0){}putchar(*ptr);(*ptr = getchar());");
    assert_eq!(bf_result, "+-<>[].,");
}

#[test]
fn test_transpile_macro_definition_and_run() {
    let commands = vec![
        BraincrapCommand::DefineMacro {
            name: 'a',
            tokens: vec![],
            code: vec![
                BraincrapCommand::Addition(1),
                BraincrapCommand::MoveRight(1),
            ],
        },
        BraincrapCommand::RunMacro { name: 'a' },
        BraincrapCommand::RunMacro { name: 'a' },
    ];

    let mut transpiler = Transpiler::new();
    let bf_result = transpiler.transpile(commands.clone(), &TranspilerArguments::Brainfuck);
    let c_result = transpiler.transpile(commands, &TranspilerArguments::C);

    assert_eq!(bf_result, "+>+>");
    assert_eq!(c_result, "(*ptr += 1);(ptr += 1);(*ptr += 1);(ptr += 1);");
}

#[test]
fn test_transpile_macro_without_definition() {
    let commands = vec![BraincrapCommand::RunMacro { name: 'b' }];

    let mut transpiler = Transpiler::new();
    let bf_result = transpiler.transpile(commands.clone(), &TranspilerArguments::Brainfuck);
    let c_result = transpiler.transpile(commands, &TranspilerArguments::C);

    assert_eq!(bf_result, "");
    assert_eq!(c_result, "");
}

#[test]
fn test_transpile_macro_with_redefinition() {
    let commands = vec![
        BraincrapCommand::DefineMacro {
            name: 'a',
            tokens: vec![],
            code: vec![BraincrapCommand::Addition(1)],
        },
        BraincrapCommand::RunMacro { name: 'a' },
        BraincrapCommand::DefineMacro {
            name: 'a',
            tokens: vec![],
            code: vec![BraincrapCommand::Substraction(1)],
        },
        BraincrapCommand::RunMacro { name: 'a' },
    ];

    let mut transpiler = Transpiler::new();
    let bf_result = transpiler.transpile(commands.clone(), &TranspilerArguments::Brainfuck);
    let c_result = transpiler.transpile(commands, &TranspilerArguments::C);

    assert_eq!(bf_result, "+-");
    assert_eq!(c_result, "(*ptr += 1);(*ptr -= 1);");
}

#[test]
fn test_transpile_import() {
    let commands = vec![BraincrapCommand::Import {
        file: "other.bcf".to_string(),
        tokens: vec![],
        code: vec![BraincrapCommand::Addition(1), BraincrapCommand::MoveLeft(1)],
    }];

    let mut transpiler = Transpiler::new();
    let bf_result = transpiler.transpile(commands.clone(), &TranspilerArguments::Brainfuck);
    let c_result = transpiler.transpile(commands, &TranspilerArguments::C);

    assert_eq!(bf_result, "+<");
    assert_eq!(c_result, "(*ptr += 1);(ptr -= 1);");
}

#[test]
fn test_transpile_empty_input() {
    let commands: Vec<BraincrapCommand> = vec![];

    let mut transpiler = Transpiler::new();
    let bf_result = transpiler.transpile(commands.clone(), &TranspilerArguments::Brainfuck);
    let c_result = transpiler.transpile(commands, &TranspilerArguments::C);

    assert_eq!(bf_result, "");
    assert_eq!(c_result, "");
}
