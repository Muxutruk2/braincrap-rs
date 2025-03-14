#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::complexity)]
#![deny(clippy::style)]
#![deny(clippy::correctness)]
#![warn(clippy::unused_io_amount)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::unnecessary_unwrap)]
#![warn(clippy::expect_used)]
#![allow(unexpected_cfgs)]
#![cfg(not(test))]
use braincrap_rs::parser::{BraincrapCommand, Parser};
use braincrap_rs::tokenizer;
use braincrap_rs::transpiler::Transpiler;
use clap::{Arg, Command};
use env_logger::Builder;
use log::debug;
use std::fs;
use std::path::Path;

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

    // Logging initalization. Set RUST_LOG in the shell
    let env = env_logger::Env::default().filter_or("RUST_LOG", "debug");
    Builder::from_env(env).init();

    // Get the input and pwd
    let input_path = Path::new(matches.get_one::<String>("input").map_or("main.bc", |v| v));
    let pwd = input_path.parent().unwrap_or(Path::new(".")).to_path_buf();

    // Read the file
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

    // Handle output
    if let Some(output_path) = matches.get_one::<String>("output") {
        fs::write(output_path, transpiled_code)
            .unwrap_or_else(|_| panic!("Failed to write to output file: {output_path}"));
    } else {
        println!("{transpiled_code}",);
    }
}
