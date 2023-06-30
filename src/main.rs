use lexer::{Lexer, Token};
use std::io::{self, Write};

mod lexer;
mod token;

fn main() {
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands...");

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

        let mut lexer = Lexer::new(&input);

        loop {
            let token = lexer.next_token();
            if token == Token::Eof {
                break;
            }
            writeln!(&stdout, "{token:?}").expect("Token should be written successfully!");
        }
    }
}
