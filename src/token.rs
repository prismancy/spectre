use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Token {
    Int(String),
    Float(String),

    Plus,
    Minus,
    Asterisk,
    Slash,
    Carrot,

    LParen,
    RParen,

    Eof,
    Illegal,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Token::*;
        match self {
            Int(value) => write!(f, "{}", value),
            Float(value) => write!(f, "{}f", value),

            Plus => write!(f, "'+'"),
            Minus => write!(f, "'-'"),
            Asterisk => write!(f, "'*'"),
            Slash => write!(f, "'/'"),
            Carrot => write!(f, "'^'"),

            LParen => write!(f, "'('"),
            RParen => write!(f, "')'"),

            Eof => write!(f, "<eof>"),
            Illegal => write!(f, "<illegal>"),
        }
    }
}

impl From<char> for Token {
    fn from(ch: char) -> Self {
        match ch {
            '+' => Self::Plus,
            '-' => Self::Minus,
            '(' => Self::LParen,
            ')' => Self::RParen,
            '/' => Self::Slash,
            '*' => Self::Asterisk,
            '^' => Self::Carrot,
            '(' => Self::LParen,
            ')' => Self::RParen,
            '\0' => Self::Eof,
            _ => Self::Illegal,
        }
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        if value.chars().all(|b| b.is_ascii_digit()) {
            if value.contains('.') {
                Self::Float(value)
            } else {
                Self::Int(value)
            }
        } else {
            Self::Int(value)
        }
    }
}
