use crate::Token;
use Token::*;

pub struct Lexer {
    text: String,
    index: usize,
    current_char: char,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        Self {
            index: 0,
            current_char: text.chars().nth(0).unwrap(),
            text,
        }
    }

    fn advance(&mut self) -> Token {
        self.index += 1;
        let next = self.text.chars().nth(self.index);
        self.current_char = match next {
            Some(c) => c,
            _ => '\0',
        };
        EOF
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut token = self.next_token();
        while token != EOF {
            tokens.push(token);
            token = self.next_token();
        }
        tokens.push(token);
        tokens
    }

    pub fn next_token(&mut self) -> Token {
        while self.current_char != '\0' {
            let token = match self.current_char {
                ' ' | '\t' | '\r' => self.advance(),
                '0'..='9' => self.number(),
                'a'..='z' | 'A'..='Z' | '_' | 'Α'..='ω' | '∞' => self.word(),
                '=' => {
                    self.advance();
                    Eq
                }
                '+' => {
                    self.advance();
                    Plus
                }
                '-' => {
                    self.advance();
                    Minus
                }
                '*' => {
                    self.advance();
                    Star
                }
                '∙' => {
                    self.advance();
                    Dot
                }
                '×' => {
                    self.advance();
                    Cross
                }
                '/' => {
                    self.advance();
                    Slash
                }
                '÷' => {
                    self.advance();
                    Divide
                }
                '%' => {
                    self.advance();
                    Percent
                }
                '^' => {
                    self.advance();
                    Carrot
                }
                '!' => {
                    self.advance();
                    Exclamation
                }
                '°' => {
                    self.advance();
                    Degree
                }
                '√' => {
                    self.advance();
                    Sqrt
                }
                '∛' => {
                    self.advance();
                    Cbrt
                }
                '∜' => {
                    self.advance();
                    Fort
                }
                '(' => {
                    self.advance();
                    LeftParen
                }
                ')' => {
                    self.advance();
                    RightParen
                }
                '|' => {
                    self.advance();
                    Pipe
                }
                '⌊' => {
                    self.advance();
                    LeftFloor
                }
                '⌋' => {
                    self.advance();
                    RightFloor
                }
                '⌈' => {
                    self.advance();
                    LeftCeil
                }
                '⌉' => {
                    self.advance();
                    RightCeil
                }
                ',' => {
                    self.advance();
                    Comma
                }
                '\n' | ';' => {
                    self.advance();
                    Newline
                }
                '\0' => EOF,
                _ => panic!("Illegal character: '{}'", self.current_char),
            };
            if token != EOF {
                return token;
            }
        }
        EOF
    }

    fn number(&mut self) -> Token {
        let mut num_str: String = self.current_char.to_string();
        let mut decimals = 0;
        self.advance();

        while "0123456789.".contains(self.current_char) {
            if self.current_char == '.' {
                decimals += 1;
            }
            num_str.push(self.current_char);
            self.advance();
        }

        if decimals > 0 {
            Float(num_str.parse::<f64>().unwrap())
        } else {
            Int(num_str.parse::<i32>().unwrap())
        }
    }

    fn word(&mut self) -> Token {
        let mut word: String = self.current_char.to_string();
        self.advance();

        while self.current_char != '\0' {
            match self.current_char {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | 'Α'..='ω' | '∞' => {
                    word.push(self.current_char);
                    self.advance();
                }
                _ => break,
            };
        }

        Identifier(word.into())
    }
}
