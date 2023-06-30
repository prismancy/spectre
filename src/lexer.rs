use std::{iter::Peekable, str::Chars};

pub use crate::token::Token;

pub struct Lexer<'a> {
    /// Source code string
    input: Peekable<Chars<'a>>,
    /// Current position in `input` (points to current char)
    // position: usize,
    /// Current reading position in `input` (after current char)
    // read_position: usize,

    /// Current char under examination
    ch: char,
}

impl Default for Lexer<'_> {
    fn default() -> Self {
        Self {
            input: "".chars().peekable(),
            // position: Default::default(),
            // read_position: Default::default(),
            ch: Default::default(),
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            input: input.chars().peekable(),
            ..Default::default()
        };
        lexer.read_char();
        lexer
    }
}

impl Lexer<'_> {
    fn read_char(&mut self) {
        self.ch = match self.input.peek() {
            Some(ch) => *ch,
            None => '\0',
        };

        self.input.next();
    }

    fn peek_char(&mut self) -> char {
        match self.input.peek() {
            Some(ch) => *ch,
            None => '\0',
        }
    }

    fn skip_whitespace(&mut self) {
        while let ' ' | '\t' | '\n' | '\r' = self.ch {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            '0'..='9' => {
                return Token::from(self.number());
            }
            _ => Token::from(self.ch),
        };
        self.read_char();
        token
    }

    fn number(&mut self) -> String {
        let mut number = String::new();
        while let '0'..='9' = self.ch {
            number.push(self.ch);
            self.read_char();
        }
        number
    }
}
