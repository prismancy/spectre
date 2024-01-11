use std::{fmt, rc::Rc};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UnaryOp {
    Pos,
    Neg,
    Abs,
    Floor,
    Ceil,
    Round,
    Degree,
    Sqrt,
    Cbrt,
    Fort,
    Fact,
    Not,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Pow,
    EqEq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    And,
    Or,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use BinaryOp::*;
        match self {
            Add => write!(f, "+"),
            Sub => write!(f, "-"),
            Mul => write!(f, "*"),
            Div => write!(f, "/"),
            Rem => write!(f, "%"),
            Pow => write!(f, "^"),
            EqEq => write!(f, "=="),
            Neq => write!(f, "!="),
            Lt => write!(f, "<"),
            Lte => write!(f, "<="),
            Gt => write!(f, ">"),
            Gte => write!(f, ">="),
            And => write!(f, "and"),
            Or => write!(f, "or"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Int(i32),
    Float(f64),
    Identifier(Rc<str>),
    Assignment(Rc<str>, Box<Node>),
    Unary(UnaryOp, Box<Node>),
    Binary(Box<Node>, BinaryOp, Box<Node>),
    If(Box<Node>, Box<Node>, Option<Box<Node>>),
    FnDef(Rc<str>, Vec<Rc<str>>, Box<Node>),
    Call(Rc<str>, Vec<Node>),
    Statements(Vec<Node>),
    Eof,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Int(x) => write!(f, "{}", x),
            Node::Float(x) => write!(f, "{}", x),
            Node::Identifier(name) => write!(f, "{}", name),
            Node::Assignment(name, node) => write!(f, "({} = {})", name, node),
            Node::Unary(op, node) => {
                use UnaryOp::*;
                match op {
                    Pos => write!(f, "(+{})", node),
                    Neg => write!(f, "(-{})", node),
                    Abs => write!(f, "|{}|", node),
                    Floor => write!(f, "⌊{}⌋", node),
                    Ceil => write!(f, "⌈{}⌉", node),
                    Round => write!(f, "⌊{}⌉", node),
                    Degree => write!(f, "({}°)", node),
                    Fact => write!(f, "({}!)", node),
                    Sqrt => write!(f, "(√{})", node),
                    Cbrt => write!(f, "(∛{})", node),
                    Fort => write!(f, "(∜{})", node),
                    Not => write!(f, "(not {})", node),
                }
            }
            Node::Binary(left, op, right) => write!(f, "({} {} {})", left, op, right),
            Node::If(cond, then, else_case) => match else_case {
                Some(else_case) => write!(f, "if {} then {} else {}", cond, then, else_case),
                None => write!(f, "if {} then {}", cond, then),
            },
            Node::FnDef(name, args, body) => write!(
                f,
                "fn {}({}) {{\n  {}\n}}",
                name,
                args.iter()
                    .map(|arg| format!("{}", arg))
                    .collect::<Vec<String>>()
                    .join(", "),
                body
            ),
            Node::Call(name, args) => write!(
                f,
                "{}({})",
                name,
                args.iter()
                    .map(|arg| format!("{}", arg))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Node::Statements(nodes) => write!(
                f,
                "{{\n  {}\n}}",
                nodes
                    .iter()
                    .map(|node| format!("{}", node))
                    .collect::<Vec<String>>()
                    .join("\n  ")
            ),
            Node::Eof => write!(f, "<eof>"),
        }
    }
}
