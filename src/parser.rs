use crate::tokenizer::BraincrapToken;
use crate::tokenizer::Lexer;
use std::fs;
use std::path::PathBuf;

/// Represents a Braincrap command that the parser recognizes.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum BraincrapCommand {
    Addition(usize),
    Substraction(usize),
    MoveLeft(usize),
    MoveRight(usize),
    OpenLoop,
    CloseLoop,
    Output(usize),
    Input(usize),
    /// Defines a macro
    DefineMacro {
        name: char,
        tokens: Vec<BraincrapToken>,
        code: Vec<BraincrapCommand>,
    },
    /// Runs a macro
    RunMacro {
        name: char,
    },
    /// Imports another Braincrap script or library from a file
    Import {
        file: String,
        tokens: Vec<BraincrapToken>,
        code: Vec<BraincrapCommand>,
    },
}

/// A parser for Braincrap language tokens.
pub struct Parser<'a> {
    /// Stores the current working directory for resolving relative imports.
    pwd: PathBuf,
    /// A slice of Braincrap tokens to be parsed.
    tokens: &'a [BraincrapToken],
    /// Tracks the current position within the token stream.
    current_position: usize,
}

impl<'a> Parser<'a> {
    /// Creates a new `Parser` instance.
    ///
    /// # Arguments
    /// * `tokens` - A slice of `BraincrapToken` representing the input program.
    /// * `pwd` - The current working directory for handling file imports.
    pub fn new(tokens: &'a [BraincrapToken], pwd: PathBuf) -> Self {
        Parser {
            pwd,
            tokens,
            current_position: 0,
        }
    }

    /// Parses a macro definition.
    ///
    /// # Arguments
    /// * `name` - The name of the macro.
    /// * `tokens` - The list of tokens forming the macro body.
    fn parse_macro(&mut self, name: char, tokens: Vec<BraincrapToken>) -> BraincrapCommand {
        let mut nested_parser = Parser::new(tokens.as_slice(), self.pwd.clone());
        let code = nested_parser.parse();

        BraincrapCommand::DefineMacro { name, tokens, code }
    }

    /// Parses an import statement and loads another Braincrap script.
    ///
    /// # Arguments
    /// * `filename` - The path to the file to be imported.
    fn parse_import(&mut self, filename: String) -> BraincrapCommand {
        let filepath = self.pwd.join(&filename);
        let file_content = fs::read_to_string(&filepath).unwrap_or_else(|_| {
            eprintln!("Failed to read file: {}", filepath.display());
            String::new()
        });

        let mut lexer = Lexer::new(file_content);
        let tokens = lexer.tokenize();
        let mut nested_parser = Parser::new(
            &tokens,
            filepath.parent().unwrap_or(&self.pwd).to_path_buf(),
        );
        let code = nested_parser.parse();

        BraincrapCommand::Import {
            file: filename,
            tokens,
            code,
        }
    }

    /// Retrieves the next token from the stream, advancing the position.
    fn next_token(&mut self) -> Option<BraincrapToken> {
        if self.current_position < self.tokens.len() {
            self.current_position += 1;
            Some(self.tokens[self.current_position - 1].clone())
        } else {
            None
        }
    }

    /// Retrieves the previous token without advancing the position.
    fn peek_previous(&mut self) -> Option<BraincrapToken> {
        if self.current_position < self.tokens.len() {
            Some(self.tokens[self.current_position].clone())
        } else {
            None
        }
    }

    /// Parses the token stream and produces a list of `BraincrapCommand`s.
    pub fn parse(&mut self) -> Vec<BraincrapCommand> {
        let mut commands = Vec::new();

        while let Some(token) = self.next_token() {
            match token {
                BraincrapToken::Plus(count) => commands.push(BraincrapCommand::Addition(count)),
                BraincrapToken::Minus(count) => {
                    commands.push(BraincrapCommand::Substraction(count))
                }
                BraincrapToken::Left(count) => commands.push(BraincrapCommand::MoveLeft(count)),
                BraincrapToken::Right(count) => commands.push(BraincrapCommand::MoveRight(count)),
                BraincrapToken::LeftBracket => commands.push(BraincrapCommand::OpenLoop),
                BraincrapToken::RightBracket => commands.push(BraincrapCommand::CloseLoop),
                BraincrapToken::Dot(count) => commands.push(BraincrapCommand::Output(count)),
                BraincrapToken::Comma(count) => commands.push(BraincrapCommand::Input(count)),

                BraincrapToken::Hash => {
                    if let Some(BraincrapToken::Char(name)) = self.next_token() {
                        if let Some(BraincrapToken::String(code_string)) = self.next_token() {
                            let mut lexer = Lexer::new(code_string);
                            let tokens = lexer.tokenize();
                            commands.push(self.parse_macro(name, tokens));
                        }
                    }
                }

                BraincrapToken::Dollar => {
                    if let Some(BraincrapToken::String(filename)) = self.next_token() {
                        commands.push(self.parse_import(filename));
                    }
                }

                BraincrapToken::Char(m) => {
                    if let Some(BraincrapToken::Hash) = self.peek_previous() {
                        // Skip macro definition name token handling here
                    } else {
                        commands.push(BraincrapCommand::RunMacro { name: m });
                    }
                }
                BraincrapToken::String(_) => {}
            }
        }

        commands
    }
}
