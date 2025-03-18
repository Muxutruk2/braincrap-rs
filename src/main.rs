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
use braincrap_rs::parser::{BraincrapCommand, Parser as BraincrapParser};
use braincrap_rs::tokenizer;
use braincrap_rs::transpiler::{Transpiler, TranspilerArguments};
use clap::Parser;
use env_logger::Builder;
use log::debug;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the input Braincrap file
    #[clap(required = true)]
    input: String,

    /// Path to the output file
    #[clap(short, long)]
    output: Option<String>,

    /// Transpile into Brainfuck
    #[clap(short = 'b', long = "brainfuck", conflicts_with = "c", action)]
    brainfuck: bool,

    /// Transpile into C
    #[clap(short = 'c', long = "C", conflicts_with = "brainfuck", action)]
    c: bool,
}

fn main() {
    let args = Args::parse();

    let transpiler_arg = if args.brainfuck {
        TranspilerArguments::Brainfuck
    } else if args.c {
        TranspilerArguments::C
    } else {
        eprintln!("One of -b or -c must be specified!");
        return;
    };

    // Logging initalization. Set RUST_LOG in the shell
    let env = env_logger::Env::default().filter_or("RUST_LOG", "debug");
    Builder::from_env(env).init();

    // Get the input and pwd
    let input_path = Path::new(&args.input);
    let pwd: PathBuf = input_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf();

    // Read the file
    let input = fs::read_to_string(input_path)
        .unwrap_or_else(|_| panic!("Failed to read file: {}", input_path.display()));

    let mut tokenizer = tokenizer::Lexer::new(input);
    let tokens = tokenizer.tokenize();
    debug!("Tokenized: {tokens:?}");

    let mut parser = BraincrapParser::new(&tokens, pwd);
    let commands: Vec<BraincrapCommand> = parser.parse();
    debug!("Parsed: {commands:?}");

    let mut transpiler = Transpiler::new();
    let mut transpiled_code = transpiler.transpile(commands, &transpiler_arg);
    debug!("Transpiled: {transpiled_code}");

    if let TranspilerArguments::C = transpiler_arg {
        let c_start_setup = "#include <stdio.h>\n#define a (*ptr += 1)\n#define s (*ptr -= 1)\n#define l (ptr--)\n#define r (ptr++)\n#define o while (*ptr) {\n#define c }\n#define p putchar(*ptr)\n#define i (*ptr = getchar())\n\nint main() {\n\tunsigned char tape[30000] = {0};\n\tunsigned char *ptr = tape;\n\n";

        let c_end_setup = "\n\treturn 0;\n}\n";

        transpiled_code = format!("{c_start_setup}\t{transpiled_code}{c_end_setup}");
    }

    // Value not in scope now, old one is used

    if let Some(output_path) = &args.output {
        fs::write(output_path, &transpiled_code)
            .unwrap_or_else(|_| panic!("Failed to write to output file: {output_path}",));
    } else {
        println!("{transpiled_code}",);
    }
}
