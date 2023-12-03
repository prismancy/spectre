use clap::{self, Parser};
use std::{
    fs,
    io::{self, Write},
};

mod ast;
mod interpreter;
mod lexer;

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
            let mut lexer = Lexer::new(input);
            let tokens = lexer.lex();
            if args.verbose {
                println!("tokens: {:?}", tokens);
            }

            let mut parser = ast::Parser::new(tokens);
            let ast = parser.parse();
            if args.verbose {
                println!("AST: {}", ast);
            }

            let mut interpreter = Interpreter::default();
            let value = interpreter.run(ast);
            println!("{}", value);
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

                let mut lexer = Lexer::new(input);
                let tokens = lexer.lex();
                if args.verbose {
                    println!("tokens: {:?}", tokens);
                }

                let mut parser = ast::Parser::new(tokens);
                let ast = parser.parse();
                if args.verbose {
                    println!("AST: {}", ast);
                }

                let value = interpreter.run(ast);
                println!("{}", value);
            }
        }
    }
}
