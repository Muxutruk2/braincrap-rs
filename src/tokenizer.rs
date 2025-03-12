const ILLEGAL_MACROS: &[char] = &[
    '+', '-', '>', '<', '.', ',', '[', ']', ' ', '\t', '\n', '\r', '\x0C', '\x1B',
];
use log::error;

#[derive(Debug, Clone)]
pub enum BraincrapToken {
    Plus,
    Minus,
    Left,
    Right,
    LeftBracket,
    RightBracket,
    Dot,
    Comma,
    Hash,
    Dollar,
    String(String), // Filename or macro code
    Char(char),     // Macro name
}

pub struct Lexer {
    index: u64,
    input: String,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Self { index: 0, input }
    }
    pub fn tokenize(&mut self) -> Vec<BraincrapToken> {
        let mut tokens = Vec::new();

        while self.index < self.input.len() as u64 {
            let current_char = self.current_char();
            match current_char {
                '$' => {
                    self.index += 1;
                    let mut filename = String::new();
                    while self.index < self.input.len() as u64 {
                        let c = self.current_char();
                        if c.is_whitespace() {
                            break;
                        }
                        filename.push(c);
                        self.index += 1;
                    }
                    tokens.push(BraincrapToken::Dollar);
                    tokens.push(BraincrapToken::String(filename));
                }
                '#' => {
                    self.index += 1;
                    let macro_name = match ILLEGAL_MACROS.contains(&self.current_char()) {
                        false => self.current_char(),
                        true => {
                            error!(
                                "Illegal macro name in index {}: {}",
                                self.index,
                                self.current_char()
                            );
                            'e'
                        }
                    };
                    self.index += 1;
                    self.index += 1;
                    let mut macro_code = String::new();
                    while self.index < self.input.len() as u64 {
                        let c = self.current_char();
                        if c == '\n' {
                            break;
                        }
                        macro_code.push(c);
                        self.index += 1;
                    }
                    tokens.push(BraincrapToken::Hash);
                    tokens.push(BraincrapToken::Char(macro_name));
                    tokens.push(BraincrapToken::String(macro_code));
                    self.index += 1;
                }
                '+' => {
                    tokens.push(BraincrapToken::Plus);
                    self.index += 1;
                }
                '-' => {
                    tokens.push(BraincrapToken::Minus);
                    self.index += 1;
                }
                '<' => {
                    tokens.push(BraincrapToken::Left);
                    self.index += 1;
                }
                '>' => {
                    tokens.push(BraincrapToken::Right);
                    self.index += 1;
                }
                '.' => {
                    tokens.push(BraincrapToken::Dot);
                    self.index += 1;
                }
                ',' => {
                    tokens.push(BraincrapToken::Comma);
                    self.index += 1;
                }
                '[' => {
                    tokens.push(BraincrapToken::LeftBracket);
                    self.index += 1;
                }
                ']' => {
                    tokens.push(BraincrapToken::RightBracket);
                    self.index += 1;
                }
                ';' => {
                    while self.index < self.input.len() as u64 {
                        let c = self.current_char();
                        if c == '\n' {
                            break;
                        }
                        self.index += 1;
                    }
                }
                _ => {
                    if !ILLEGAL_MACROS.contains(&self.current_char()) {
                        tokens.push(BraincrapToken::Char(self.current_char()));
                    }
                    self.index += 1;
                }
            }
        }
        tokens
    }
    fn current_char(&self) -> char {
        self.input.chars().nth(self.index as usize).unwrap_or('\0')
    }
}
