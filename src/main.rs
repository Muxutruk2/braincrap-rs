mod parser;
mod tokenizer;
mod transpiler;
use crate::parser::BraincrapCommand;
use crate::parser::Parser;
use crate::transpiler::Transpiler;
use env_logger::{Builder, Env};
use log::debug;
use std::fs;

// TODO: CLI AND FILES
fn main() {
    let env = Env::default().filter_or("RUST_LOG", "debug");
    Builder::from_env(env).init();

    let input = fs::read_to_string("main.bf").unwrap();

    let mut tokenizer = tokenizer::Lexer::new(input);
    let tokens = tokenizer.tokenize();
    debug!("Tokenized: {tokens:?}");

    let mut parser = Parser::new(&tokens);
    let commands: Vec<BraincrapCommand> = parser.parse();
    debug!("Parsed: {commands:?}");

    let mut transpiler = Transpiler::new();
    let transpiled_code = transpiler.transpile(commands);
    debug!("Transpiled: {transpiled_code}");
}
