use crate::parser::BraincrapCommand;
use std::collections::HashMap;

pub struct Transpiler {
    macros: HashMap<char, String>,
}

impl Transpiler {
    pub fn new() -> Self {
        Self {
            macros: HashMap::new(),
        }
    }

    pub fn transpile(&mut self, commands: Vec<BraincrapCommand>) -> String {
        let mut output = String::new();
        for command in commands {
            let bf_command = self.transpile_command(&command);
            output.push_str(&bf_command);
        }
        output
    }

    fn transpile_command(&mut self, command: &BraincrapCommand) -> String {
        match command {
            BraincrapCommand::Addition => "+".to_string(),
            BraincrapCommand::Substraction => "-".to_string(),
            BraincrapCommand::MoveLeft => "<".to_string(),
            BraincrapCommand::MoveRight => ">".to_string(),
            BraincrapCommand::OpenLoop => "[".to_string(),
            BraincrapCommand::CloseLoop => "]".to_string(),
            BraincrapCommand::Output => ".".to_string(),
            BraincrapCommand::Input => ",".to_string(),

            BraincrapCommand::DefineMacro {
                name,
                tokens: _tokens,
                code,
            } => {
                let commands: Vec<BraincrapCommand> = code.to_vec();
                if !self.macros.contains_key(name) {
                    let expanded_code = self.transpile(commands);
                    self.macros.insert(*name, expanded_code);
                    // println!("Macro defined: {}", name); // Debugging output
                } else {
                    // println!("Macro already defined: {}", name); // Debugging output
                }
                String::new()
            }

            BraincrapCommand::RunMacro { name } => {
                match self.macros.get(name) {
                    Some(expanded_code) => {
                        // println!("Running macro: {}", name); // Debugging output
                        expanded_code.clone()
                    }
                    None => {
                        // eprintln!("Undefined macro: {}", name);
                        String::new()
                    }
                }
            }

            BraincrapCommand::Import {
                file: _file,
                tokens: _tokens,
                code,
            } => {
                let commands: Vec<BraincrapCommand> = code.to_vec();
                self.transpile(commands)
            }
        }
    }
}
