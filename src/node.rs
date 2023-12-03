use std::fmt;

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
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Pow,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Int(i32),
    Float(f64),
    Identifier(String),
    Unary(UnaryOp, Box<Node>),
    Binary(Box<Node>, BinaryOp, Box<Node>),
    Call(String, Vec<Node>),
    Statements(Vec<Node>),
    EOF,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Int(x) => write!(f, "{}", x),
            Node::Float(x) => write!(f, "{}", x),
            Node::Identifier(name) => write!(f, "{}", name),
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
                }
            }
            Node::Binary(left, op, right) => {
                use BinaryOp::*;
                match op {
                    Add => write!(f, "({} + {})", left, right),
                    Sub => write!(f, "({} - {})", left, right),
                    Mul => write!(f, "({} * {})", left, right),
                    Div => write!(f, "({} / {})", left, right),
                    Rem => write!(f, "({} % {})", left, right),
                    Pow => write!(f, "({} ^ {})", left, right),
                }
            }
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
            Node::EOF => write!(f, "<eof>"),
        }
    }
}
