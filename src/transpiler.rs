use crate::parser::BraincrapCommand;
use std::collections::HashMap;

/// A transpiler that converts Braincrap commands into Brainfuck code.
pub struct Transpiler {
    /// Stores defined macros, mapping their names to their transpiled code.
    macros: HashMap<char, String>,
}

impl Transpiler {
    /// Creates a new `Transpiler` instance.
    pub fn new() -> Self {
        Self {
            macros: HashMap::new(),
        }
    }

    /// Transpiles a vector of `BraincrapCommand`s into a Brainfuck-compatible string.
    ///
    /// # Arguments
    /// * `commands` - A vector of `BraincrapCommand`s to be transpiled.
    ///
    /// # Returns
    /// A `String` containing the equivalent Brainfuck code.
    pub fn transpile(&mut self, commands: Vec<BraincrapCommand>) -> String {
        let mut output = String::new();
        for command in commands {
            let bf_command = self.transpile_command(&command);
            output.push_str(&bf_command);
        }
        output
    }

    /// Transpiles a single `BraincrapCommand` into its Brainfuck equivalent.
    ///
    /// # Arguments
    /// * `command` - A reference to a `BraincrapCommand`.
    ///
    /// # Returns
    /// A `String` containing the Brainfuck representation of the command.
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

            /// Defines a macro and stores its transpiled representation.
            BraincrapCommand::DefineMacro {
                name,
                tokens: _tokens,
                code,
            } => {
                let commands: Vec<BraincrapCommand> = code.to_vec();
                if !self.macros.contains_key(name) {
                    let expanded_code = self.transpile(commands);
                    self.macros.insert(*name, expanded_code);
                }
                String::new()
            }

            /// Expands a previously defined macro.
            BraincrapCommand::RunMacro { name } => {
                match self.macros.get(name) {
                    Some(expanded_code) => expanded_code.clone(),
                    None => String::new(), // Undefined macros produce no output
                }
            }

            /// Transpiles imported Braincrap code by recursively processing its commands.
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
