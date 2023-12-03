use std::io::{self, Write};

mod ast;
mod interpreter;
mod lexer;

pub use ast::*;
pub use interpreter::*;
pub use lexer::*;

fn main() {
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
            writeln!(&stdout, "Error: {e}").expect("Error message should be written successfully!");
            return;
        }

        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex();
        println!("{:?}", tokens);
        if tokens[0] == Token::EOF {
            continue;
        }

        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        println!("{}", ast);

        let value = interpreter.run(ast);
        println!("{}", value)
    }
}
