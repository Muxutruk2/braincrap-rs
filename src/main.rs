use clap::{Arg, Command};
use std::path::Path;
mod parser;
mod tokenizer;
mod transpiler;
use crate::parser::BraincrapCommand;
use crate::parser::Parser;
use crate::transpiler::Transpiler;
use env_logger::Builder;
use log::debug;
use std::fs;

fn main() {
    let matches = Command::new("Transpile Braincrap")
        .arg(
            Arg::new("input")
                .required(true)
                .help("Path to the input Braincrap file"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .num_args(1)
                .help("Path to the output file"),
        )
        .get_matches();

    let env = env_logger::Env::default().filter_or("RUST_LOG", "debug");
    Builder::from_env(env).init();

    let input_path = Path::new(matches.get_one::<String>("input").unwrap());
    let pwd = input_path.parent().unwrap_or(Path::new(".")).to_path_buf();

    let input = fs::read_to_string(input_path)
        .unwrap_or_else(|_| panic!("Failed to read file: {}", input_path.display()));

    let mut tokenizer = tokenizer::Lexer::new(input);
    let tokens = tokenizer.tokenize();
    debug!("Tokenized: {tokens:?}");

    let mut parser = Parser::new(&tokens, pwd);
    let commands: Vec<BraincrapCommand> = parser.parse();
    debug!("Parsed: {commands:?}");

    let mut transpiler = Transpiler::new();
    let transpiled_code = transpiler.transpile(commands);
    debug!("Transpiled: {transpiled_code}");

    if let Some(output_path) = matches.get_one::<String>("output") {
        // <- Fix: replaces `.value_of(...)`
        fs::write(output_path, transpiled_code)
            .unwrap_or_else(|_| panic!("Failed to write to output file: {}", output_path));
    } else {
        println!("{}", transpiled_code);
    }
}
