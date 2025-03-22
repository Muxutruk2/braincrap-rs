/// Defines characters that cannot be used as macro names.
const ILLEGAL_MACROS: &[char] = &[
    '+', '-', '>', '<', '.', ',', '[', ']', ' ', '\t', '\n', '\r', '\x0C', '\x1B',
];

use log::error;

/// Represents different token types recognized by the lexer.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum BraincrapToken {
    Plus(usize),
    Minus(usize),
    Left(usize),
    Right(usize),
    LeftBracket,
    RightBracket,
    Dot(usize),
    Comma(usize),
    Hash,
    Dollar,
    /// Represents a string, used for filenames or macro code.
    String(String),
    /// Represents a character, used for macro names.
    Char(char),
}

/// A lexer that converts Braincrap source code into tokens.
pub struct Lexer {
    /// The current position within the input string.
    index: u64,
    /// The Braincrap source code as a string.
    input: String,
}

impl Lexer {
    /// Creates a new `Lexer` instance.
    ///
    /// # Arguments
    /// * `input` - The Braincrap source code to be tokenized.
    pub fn new(input: String) -> Lexer {
        Self { index: 0, input }
    }

    /// Converts the input source code into a vector of `BraincrapToken`s.
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
                    let macro_name = if ILLEGAL_MACROS.contains(&self.current_char()) {
                        error!(
                            "Illegal macro name at index {}: {}",
                            self.index,
                            self.current_char()
                        );
                        'e' // Default to an arbitrary invalid macro name
                    } else {
                        self.current_char()
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
                    let mut count: usize = 0;
                    while self.current_char() == '+' {
                        self.index += 1;
                        count += 1;
                    }
                    tokens.push(BraincrapToken::Plus(count));
                }
                '-' => {
                    let mut count: usize = 0;
                    while self.current_char() == '-' {
                        self.index += 1;
                        count += 1;
                    }
                    tokens.push(BraincrapToken::Minus(count));
                }
                '<' => {
                    let mut count: usize = 0;
                    while self.current_char() == '<' {
                        self.index += 1;
                        count += 1;
                    }
                    tokens.push(BraincrapToken::Left(count));
                }
                '>' => {
                    let mut count: usize = 0;
                    while self.current_char() == '>' {
                        self.index += 1;
                        count += 1;
                    }
                    tokens.push(BraincrapToken::Right(count));
                }
                '.' => {
                    let mut count: usize = 0;
                    while self.current_char() == '.' {
                        self.index += 1;
                        count += 1;
                    }
                    tokens.push(BraincrapToken::Dot(count));
                }
                ',' => {
                    let mut count: usize = 0;
                    while self.current_char() == ',' {
                        self.index += 1;
                        count += 1;
                    }
                    tokens.push(BraincrapToken::Comma(count));
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
                    // Skip comments until a newline is found.
                    while self.index < self.input.len() as u64 {
                        let c = self.current_char();
                        if c == '\n' {
                            break;
                        }
                        self.index += 1;
                    }
                }
                _ => {
                    // If the character is not illegal, treat it as a potential macro call.
                    if !ILLEGAL_MACROS.contains(&self.current_char()) {
                        tokens.push(BraincrapToken::Char(self.current_char()));
                    }
                    self.index += 1;
                }
            }
        }
        tokens
    }

    /// Returns the current character without advancing the position.
    fn current_char(&self) -> char {
        self.input
            .chars()
            .nth(usize::try_from(self.index).unwrap_or(0))
            .unwrap_or('\0')
    }
}
