use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Int(i32),
    Float(f64),
    Identifier(String),
    Eq,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Pow,
    LParen,
    RParen,
    Comma,
    Newline,
    EOF,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Token::*;
        match self {
            Int(value) => write!(f, "{}", value),
            Float(value) => write!(f, "{}f", value),
            Identifier(name) => write!(f, "{}", name),
            Eq => write!(f, "'='"),
            Add => write!(f, "'+'"),
            Sub => write!(f, "'-'"),
            Mul => write!(f, "'*'"),
            Div => write!(f, "'/'"),
            Rem => write!(f, "'%'"),
            Pow => write!(f, "'^'"),
            LParen => write!(f, "'('"),
            RParen => write!(f, "')'"),
            Comma => write!(f, "','"),
            Newline => write!(f, "'\\n'"),
            EOF => write!(f, "<eof>"),
        }
    }
}
