use crate::parser::BraincrapCommand;
use log::error;
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

fn are_braces_balanced(code: &str) -> bool {
    let mut stack = Vec::new();

    for c in code.chars() {
        match c {
            '[' => stack.push(c),
            ']' => {
                if stack.pop().is_none() {
                    return false;
                }
            }
            _ => {}
        }
    }

    stack.is_empty()
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

        if !are_braces_balanced(&output) {
            error!("Braces not balanced!")
        }

        output
    }

    /// Transpiles to C
    fn transpile_c(&mut self, commands: Vec<BraincrapCommand>) -> String {
        let mut output = String::new();

        for command in commands {
            let c_line = self.transpile_command_c(&command);
            output.push_str(&c_line);
            // output.push('\n');
        }

        output
    }

    /// Transpiles a single Braincrap command into Brainfuck
    fn transpile_command_bf(&mut self, command: &BraincrapCommand) -> String {
        match command {
            BraincrapCommand::Addition(count) => "+".repeat(*count).to_string(),
            BraincrapCommand::Substraction(count) => "-".repeat(*count).to_string(),
            BraincrapCommand::MoveLeft(count) => "<".repeat(*count).to_string(),
            BraincrapCommand::MoveRight(count) => ">".repeat(*count).to_string(),
            BraincrapCommand::OpenLoop => "[".to_string(),
            BraincrapCommand::CloseLoop => "]".to_string(),
            BraincrapCommand::Output(count) => ".".repeat(*count).to_string(),
            BraincrapCommand::Input(count) => ",".repeat(*count).to_string(),
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
            BraincrapCommand::Addition(count) => format!("(*ptr += {count});"),
            BraincrapCommand::Substraction(count) => format!("(*ptr -= {count});"),
            BraincrapCommand::MoveLeft(count) => format!("(ptr -= {count});"),
            BraincrapCommand::MoveRight(count) => format!("(ptr += {count});"),
            BraincrapCommand::OpenLoop => "while(*ptr != 0){".to_string(),
            BraincrapCommand::CloseLoop => "}".to_string(),
            BraincrapCommand::Output(count) => {
                if *count >= 3 {
                    format!("for(int i=0;i<{count};i++){{putchar(*ptr);}}")
                } else {
                    "putchar(*ptr);".repeat(*count).to_string()
                }
            }
            BraincrapCommand::Input(count) => {
                if *count >= 2 {
                    format!("for(i=0;i<{count};i++){{*ptr = getchar());}}")
                } else {
                    "(*ptr = getchar());".repeat(*count).to_string()
                }
            }
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
