use clap::{self, Parser};
use std::{
    fs,
    io::{self, Write},
};

mod ast;
mod interpreter;
mod lexer;
mod position;

pub use interpreter::*;
pub use lexer::*;

#[derive(clap::Parser)]
struct Arguments {
    /// The file to run
    file: Option<String>,
    /// Verbose mode
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Arguments::parse();

    match args.file {
        Some(file) => {
            let input = fs::read_to_string(file).expect("File should be read successfully!");
            run(input, args.verbose, &mut Interpreter::default());
        }
        None => {
            let stdin = io::stdin();
            let mut stdout = io::stdout();

            let mut interpreter = Interpreter::default();

            loop {
                write!(&stdout, "> ").expect("PROMPT string should be written successfully!");

                stdout
                    .flush()
                    .expect("Should have flushed stdout successfully!");

                let mut input = String::new();

                if let Err(e) = stdin.read_line(&mut input) {
                    writeln!(&stdout, "Error: {e}")
                        .expect("Error message should be written successfully!");
                    return;
                }

                run(input, args.verbose, &mut interpreter);
            }
        }
    }
}

fn run(input: String, verbose: bool, interpreter: &mut Interpreter) {
    let mut lexer = Lexer::new(input.clone());
    match lexer.lex() {
        Ok(tokens) => {
            if verbose {
                println!(
                    "tokens: {}",
                    tokens
                        .iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                );
            }

            let mut parser = ast::Parser::new(tokens);
            match parser.parse() {
                Ok(ast) => {
                    if verbose {
                        println!("AST: {}", ast);
                    }

                    let value = interpreter.run(ast);
                    println!("{}", value);
                }
                Err(e) => {
                    eprintln!(
                        "Error: {}\n{}",
                        e,
                        e.start.get_lines_between_as_display(&e.end, &input)
                    );
                }
            }
        }
        Err(e) => {
            eprintln!(
                "Error: {}\n{}",
                e,
                e.start.get_lines_between_as_display(&e.end, &input)
            );
        }
    }
}
