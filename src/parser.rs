use crate::tokenizer::BraincrapToken;
use crate::tokenizer::Lexer;
use std::fs;

#[derive(Debug, Clone)]
pub enum BraincrapCommand {
    Addition,
    Substraction,
    MoveLeft,
    MoveRight,
    OpenLoop,
    CloseLoop,
    Output,
    Input,
    DefineMacro {
        name: char,
        tokens: Vec<BraincrapToken>,
        code: Vec<BraincrapCommand>,
    },
    RunMacro {
        name: char,
    },
    Import {
        file: String,
        tokens: Vec<BraincrapToken>,
        code: Vec<BraincrapCommand>,
    },
}

pub struct Parser<'a> {
    tokens: &'a [BraincrapToken],
    current_position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [BraincrapToken]) -> Self {
        Parser {
            tokens,
            current_position: 0,
        }
    }

    fn parse_macro(&mut self, name: char, tokens: Vec<BraincrapToken>) -> BraincrapCommand {
        let mut nested_parser = Parser::new(&tokens);
        let code = nested_parser.parse();

        BraincrapCommand::DefineMacro { name, tokens, code }
    }

    fn parse_import(&mut self, filename: String) -> BraincrapCommand {
        let file_content = fs::read_to_string(&filename).unwrap_or_else(|_| {
            eprintln!("Failed to read file: {}", filename);
            String::new()
        });
        let mut lexer = Lexer::new(file_content);
        let tokens = lexer.tokenize();
        let mut nested_parser = Parser::new(&tokens);
        let code = nested_parser.parse();

        BraincrapCommand::Import {
            file: filename,
            tokens,
            code,
        }
    }

    fn next_token(&mut self) -> Option<BraincrapToken> {
        if self.current_position < self.tokens.len() {
            self.current_position += 1;
            Some(self.tokens[self.current_position - 1].clone())
        } else {
            None
        }
    }
    fn previous_token(&mut self) -> Option<BraincrapToken> {
        if self.current_position < self.tokens.len() {
            Some(self.tokens[self.current_position].clone())
        } else {
            None
        }
    }

    pub fn parse(&mut self) -> Vec<BraincrapCommand> {
        let mut commands = Vec::new();

        while let Some(token) = self.next_token() {
            match token {
                BraincrapToken::Plus => commands.push(BraincrapCommand::Addition),
                BraincrapToken::Minus => commands.push(BraincrapCommand::Substraction),
                BraincrapToken::Left => commands.push(BraincrapCommand::MoveLeft),
                BraincrapToken::Right => commands.push(BraincrapCommand::MoveRight),
                BraincrapToken::LeftBracket => commands.push(BraincrapCommand::OpenLoop),
                BraincrapToken::RightBracket => commands.push(BraincrapCommand::CloseLoop),
                BraincrapToken::Dot => commands.push(BraincrapCommand::Output),
                BraincrapToken::Comma => commands.push(BraincrapCommand::Input),

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
                    if let Some(BraincrapToken::Hash) = self.previous_token() {
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
