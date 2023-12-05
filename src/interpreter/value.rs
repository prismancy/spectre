use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Int(i32),
    Float(f64),
    Complex(f64, f64),
    Function(fn(&Vec<Value>) -> Value),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Value::*;
        match self {
            Int(value) => write!(f, "{}", value),
            Float(value) => write!(f, "{}", value),
            Complex(r, i) => write!(f, "{} + {}i", r, i),
            Function(_) => write!(f, "<fn>"),
        }
    }
}
