use crate::{position::Position, LexError, Token, TokenType};
use TokenType::*;

const SUPERSCRIPT: &str = "ᵃᵇᶜᵈᵉᶠᵍʰⁱʲᵏˡᵐⁿᵒᵖʳˢᵗᵘᵛʷˣʸᶻᴬᴮᶜᴰᴱᶠᴳᴴᴵᴶᴷᴸᴹᴺᴼᴾᴿˢᵀᵁⱽᵂˣʸᶻ⁰¹²³⁴⁵⁶⁷⁸⁹⁺⁻⁼⁽⁾";
const NORMALSCRIPT: &str = "abcdefghijklmnoprstuvwxyzABCDEFGHIJKLMNOPRSTUVWXYZ0123456789+-=()";

type LexResult = Result<Token, LexError>;

pub struct Lexer {
    source: String,
    index: usize,
    current_char: char,
    position: Position,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            index: 0,
            current_char: source.chars().nth(0).unwrap_or('\0'),
            position: Position::default(),
            source,
        }
    }

    fn advance(&mut self) -> Token {
        self.index += 1;
        let ch = self.source.chars().nth(self.index).unwrap_or('\0');
        self.current_char = ch;
        self.position.advance(ch);
        Token {
            ty: EOF,
            start: self.position,
            end: self.position,
        }
    }

    fn error(&self, msg: String, start: Position) -> LexError {
        LexError {
            msg,
            start,
            end: self.position,
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens: Vec<Token> = vec![];
        let mut token = self.next_token()?;
        while token.ty != EOF {
            tokens.push(token);
            token = self.next_token()?;
        }
        tokens.push(token);
        Ok(tokens)
    }

    pub fn next_token(&mut self) -> LexResult {
        while matches!(self.current_char, ' ' | '\t' | '\r') {
            self.advance();
        }

        let start = self.position;
        match self.current_char {
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' | 'Α'..='ω' | '∞' => self.word(),
            ch if SUPERSCRIPT.contains(ch) => self.superscript(),
            '=' => {
                self.advance();
                Ok(Token {
                    ty: Eq,
                    start,
                    end: self.position,
                })
            }
            '+' => {
                self.advance();
                Ok(Token {
                    ty: Plus,
                    start,
                    end: self.position,
                })
            }
            '-' => {
                self.advance();
                Ok(Token {
                    ty: Minus,
                    start,
                    end: self.position,
                })
            }
            '*' => {
                self.advance();
                Ok(Token {
                    ty: Star,
                    start,
                    end: self.position,
                })
            }
            '∙' => {
                self.advance();
                Ok(Token {
                    ty: Dot,
                    start,
                    end: self.position,
                })
            }
            '×' => {
                self.advance();
                Ok(Token {
                    ty: Cross,
                    start,
                    end: self.position,
                })
            }
            '/' => {
                self.advance();
                Ok(Token {
                    ty: Slash,
                    start,
                    end: self.position,
                })
            }
            '÷' => {
                self.advance();
                Ok(Token {
                    ty: Divide,
                    start,
                    end: self.position,
                })
            }
            '%' => {
                self.advance();
                Ok(Token {
                    ty: Percent,
                    start,
                    end: self.position,
                })
            }
            '^' => {
                self.advance();
                Ok(Token {
                    ty: Carrot,
                    start,
                    end: self.position,
                })
            }
            '!' => {
                self.advance();
                Ok(Token {
                    ty: Exclamation,
                    start,
                    end: self.position,
                })
            }
            '°' => {
                self.advance();
                Ok(Token {
                    ty: Degree,
                    start,
                    end: self.position,
                })
            }
            '√' => {
                self.advance();
                Ok(Token {
                    ty: Sqrt,
                    start,
                    end: self.position,
                })
            }
            '∛' => {
                self.advance();
                Ok(Token {
                    ty: Cbrt,
                    start,
                    end: self.position,
                })
            }
            '∜' => {
                self.advance();
                Ok(Token {
                    ty: Fort,
                    start,
                    end: self.position,
                })
            }
            '(' => {
                self.advance();
                Ok(Token {
                    ty: LeftParen,
                    start,
                    end: self.position,
                })
            }
            ')' => {
                self.advance();
                Ok(Token {
                    ty: RightParen,
                    start,
                    end: self.position,
                })
            }
            '|' => {
                self.advance();
                Ok(Token {
                    ty: Pipe,
                    start,
                    end: self.position,
                })
            }
            '⌊' => {
                self.advance();
                Ok(Token {
                    ty: LeftFloor,
                    start,
                    end: self.position,
                })
            }
            '⌋' => {
                self.advance();
                Ok(Token {
                    ty: RightFloor,
                    start,
                    end: self.position,
                })
            }
            '⌈' => {
                self.advance();
                Ok(Token {
                    ty: LeftCeil,
                    start,
                    end: self.position,
                })
            }
            '⌉' => {
                self.advance();
                Ok(Token {
                    ty: RightCeil,
                    start,
                    end: self.position,
                })
            }
            ',' => {
                self.advance();
                Ok(Token {
                    ty: Comma,
                    start,
                    end: self.position,
                })
            }
            '\n' | ';' => {
                self.advance();
                Ok(Token {
                    ty: Newline,
                    start,
                    end: self.position,
                })
            }
            '\0' => Ok(Token {
                ty: EOF,
                start,
                end: self.position,
            }),
            _ => Err(self.error(
                format!("unknown character '{}'", self.current_char),
                self.position,
            )),
        }
    }

    fn number(&mut self) -> LexResult {
        let start = self.position;
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

        Ok(Token {
            ty: if decimals > 0 {
                Float(num_str.parse::<f64>().unwrap())
            } else {
                Int(num_str.parse::<i32>().unwrap())
            },
            start,
            end: self.position,
        })
    }

    fn word(&mut self) -> LexResult {
        let start = self.position;
        let mut word = self.current_char.to_string();
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

        Ok(Token {
            ty: Identifier(word.into()),
            start,
            end: self.position,
        })
    }

    fn superscript(&mut self) -> LexResult {
        let start = self.position;
        let mut source = String::new();

        while self.current_char != '\0' {
            match SUPERSCRIPT.chars().position(|ch| ch == self.current_char) {
                Some(index) => {
                    let normal_char = NORMALSCRIPT.chars().nth(index).unwrap();
                    source.push(normal_char);
                    self.advance();
                }
                None => break,
            };
        }

        let mut lexer = Lexer::new(source);
        let mut tokens = lexer.lex()?;
        tokens.pop();
        Ok(Token {
            ty: Superscript(tokens),
            start,
            end: self.position,
        })
    }
}
