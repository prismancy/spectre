use std::fmt;

use common::Position;

#[derive(Debug)]
pub struct LexError {
    pub msg: String,
    pub start: Position,
    pub end: Position,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
