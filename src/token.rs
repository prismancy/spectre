use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Int(i32),
    Float(f64),
    Identifier(String),
    Eq,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Carrot,
    Exclamation,
    Degree,
    Sqrt,
    Cbrt,
    Fort,
    LParen,
    RParen,
    Pipe,
    Comma,
    Newline,
    EOF,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Token::*;
        match self {
            Int(value) => write!(f, "{}", value),
            Float(value) => write!(f, "{}", value),
            Identifier(name) => write!(f, "{}", name),
            Eq => write!(f, "'='"),
            Plus => write!(f, "'+'"),
            Minus => write!(f, "'-'"),
            Star => write!(f, "'*'"),
            Slash => write!(f, "'/'"),
            Percent => write!(f, "'%'"),
            Carrot => write!(f, "'^'"),
            Exclamation => write!(f, "'!'"),
            Degree => write!(f, "'°'"),
            Sqrt => write!(f, "'√'"),
            Cbrt => write!(f, "'∛'"),
            Fort => write!(f, "'∜'"),
            LParen => write!(f, "'('"),
            RParen => write!(f, "')'"),
            Pipe => write!(f, "'|'"),
            Comma => write!(f, "','"),
            Newline => write!(f, "'\\n'"),
            EOF => write!(f, "<eof>"),
        }
    }
}
