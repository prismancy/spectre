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
    Dot,
    Cross,
    Slash,
    Divide,
    Percent,
    Carrot,
    Exclamation,
    Degree,
    Sqrt,
    Cbrt,
    Fort,
    LeftParen,
    RightParen,
    Pipe,
    LeftFloor,
    RightFloor,
    LeftCeil,
    RightCeil,
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
            Dot => write!(f, "'∙'"),
            Cross => write!(f, "'×'"),
            Slash => write!(f, "'/'"),
            Divide => write!(f, "'÷'"),
            Percent => write!(f, "'%'"),
            Carrot => write!(f, "'^'"),
            Exclamation => write!(f, "'!'"),
            Degree => write!(f, "'°'"),
            Sqrt => write!(f, "'√'"),
            Cbrt => write!(f, "'∛'"),
            Fort => write!(f, "'∜'"),
            LeftParen => write!(f, "'('"),
            RightParen => write!(f, "')'"),
            Pipe => write!(f, "'|'"),
            LeftFloor => write!(f, "'⌊'"),
            RightFloor => write!(f, "'⌋'"),
            LeftCeil => write!(f, "'⌈'"),
            RightCeil => write!(f, "'⌉'"),
            Comma => write!(f, "','"),
            Newline => write!(f, "'\\n'"),
            EOF => write!(f, "<eof>"),
        }
    }
}
