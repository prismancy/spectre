use std::{fmt, rc::Rc};

use parser::Node;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Float(f64),
    Complex(f64, f64),
    Function(Rc<str>, Vec<Rc<str>>, Box<Node>),
    NativeFunction(fn(&Vec<Value>) -> Value),
}

use Value::*;

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Int(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Float(value)
    }
}

impl From<(f64, f64)> for Value {
    fn from(value: (f64, f64)) -> Self {
        Complex(value.0, value.1)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Int(value) => write!(f, "{}", value),
            Float(value) => write!(f, "{}", value),
            Complex(r, i) => write!(f, "{} + {}i", r, i),
            Function(name, _, _) => write!(f, "<fn {}>", name),
            NativeFunction(_) => write!(f, "<native fn>"),
        }
    }
}
