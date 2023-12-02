use std::io::{self, Write};

mod interpreter;
mod lexer;
mod node;
mod parser;
mod token;

pub use interpreter::*;
pub use lexer::*;
pub use node::*;
pub use parser::*;
pub use token::*;

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        write!(&stdout, "> ").expect("PROMPT string should be written successfully!");

        stdout
            .flush()
            .expect("Should have flushed stdout successfully!");

        let mut input = String::new();

        if let Err(e) = stdin.read_line(&mut input) {
            writeln!(&stdout, "Error: {e}").expect("Error message should be written successfully!");
            return;
        }

        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex();

        let mut parser = Parser::new(tokens);
        let ast = parser.parse();

        let mut interpreter = Interpreter::default();
        let value = interpreter.run(ast);
        println!("{}", value)
    }
}
