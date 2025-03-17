use crate::parser::BraincrapCommand;
use std::collections::HashMap;

#[derive(Debug)]
pub enum TranspilerArguments {
    C,
    Brainfuck,
}

/// A transpiler that converts Braincrap commands into Brainfuck code.
pub struct Transpiler {
    /// Stores defined macros, mapping their names to their transpiled code.
    macros: HashMap<char, String>,
}

impl Default for Transpiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Transpiler {
    /// Creates a new `Transpiler` instance.
    pub fn new() -> Self {
        Self {
            macros: HashMap::new(),
        }
    }

    /// Transpiles a vector of `BraincrapCommand`s into either Brainfuck or C.
    pub fn transpile(
        &mut self,
        commands: Vec<BraincrapCommand>,
        args: &TranspilerArguments,
    ) -> String {
        match args {
            TranspilerArguments::Brainfuck => self.transpile_brainfuck(commands),
            TranspilerArguments::C => self.transpile_c(commands),
        }
    }

    /// Transpiles to Brainfuck
    fn transpile_brainfuck(&mut self, commands: Vec<BraincrapCommand>) -> String {
        let mut output = String::new();

        for command in commands {
            let bf_command = self.transpile_command_bf(&command);
            output.push_str(&bf_command);
        }

        output
    }

    /// Transpiles to C
    fn transpile_c(&mut self, commands: Vec<BraincrapCommand>) -> String {
        let mut output = String::new();

        for command in commands {
            let c_line = self.transpile_command_c(&command);
            output.push_str(&c_line);
        }

        output
    }

    /// Transpiles a single Braincrap command into Brainfuck
    fn transpile_command_bf(&mut self, command: &BraincrapCommand) -> String {
        match command {
            BraincrapCommand::Addition => "+".to_string(),
            BraincrapCommand::Substraction => "-".to_string(),
            BraincrapCommand::MoveLeft => "<".to_string(),
            BraincrapCommand::MoveRight => ">".to_string(),
            BraincrapCommand::OpenLoop => "[".to_string(),
            BraincrapCommand::CloseLoop => "]".to_string(),
            BraincrapCommand::Output => ".".to_string(),
            BraincrapCommand::Input => ",".to_string(),
            BraincrapCommand::DefineMacro { name, code, .. } => {
                let expanded_code = self.transpile_brainfuck(code.clone());
                self.macros.insert(*name, expanded_code);
                String::new()
            }
            BraincrapCommand::RunMacro { name } => {
                self.macros.get(name).cloned().unwrap_or_else(String::new)
            }
            BraincrapCommand::Import { code, .. } => self.transpile_brainfuck(code.clone()),
        }
    }

    /// Transpiles a single Braincrap command into C
    fn transpile_command_c(&mut self, command: &BraincrapCommand) -> String {
        match command {
            BraincrapCommand::Addition => "*ptr += 1;".to_string(),
            BraincrapCommand::Substraction => "*ptr -= 1;".to_string(),
            BraincrapCommand::MoveLeft => "ptr--;".to_string(),
            BraincrapCommand::MoveRight => "ptr++;".to_string(),
            BraincrapCommand::OpenLoop => "while (*ptr) {".to_string(),
            BraincrapCommand::CloseLoop => "}".to_string(),
            BraincrapCommand::Output => "putchar(*ptr);".to_string(),
            BraincrapCommand::Input => "*ptr = getchar();".to_string(),
            BraincrapCommand::DefineMacro { name, code, .. } => {
                let expanded_code = self.transpile_c(code.clone());
                self.macros.insert(*name, expanded_code);
                String::new()
            }
            BraincrapCommand::RunMacro { name } => {
                self.macros.get(name).cloned().unwrap_or_else(String::new)
            }
            BraincrapCommand::Import { code, .. } => self.transpile_c(code.clone()),
        }
    }
}
