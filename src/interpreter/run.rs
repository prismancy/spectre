use crate::{interpreter::Value, BinaryOp, Node, Scope, UnaryOp};

pub struct Interpreter {
    pub scope: Scope,
}

impl Default for Interpreter {
    fn default() -> Self {
        let mut interpreter = Self {
            scope: Scope::default(),
        };
        interpreter.math();
        interpreter
    }
}

impl Interpreter {
    pub fn run(&mut self, ast: Node) -> Value {
        self.visit(ast)
    }

    pub fn add_var(&mut self, name: &str, value: Value) {
        self.scope.set(name.to_string(), value);
    }

    fn visit(&mut self, node: Node) -> Value {
        match node {
            Node::Int(x) => Value::Int(x),
            Node::Float(x) => Value::Float(x),
            Node::Identifier(name) => self.scope.get(&name),
            Node::Unary(op, node) => {
                let value = self.visit(*node);

                use UnaryOp::*;
                match op {
                    Pos => value,
                    Neg => match value {
                        Value::Int(x) => Value::Int(-x),
                        Value::Float(x) => Value::Float(-x),
                        _ => unimplemented!(),
                    },
                    Fact => match value {
                        Value::Int(x) => Value::Int(factorial(x)),
                        Value::Float(x) => Value::Int(factorial(x as i32)),
                        _ => unimplemented!(),
                    },
                    Sqrt => match value {
                        Value::Int(x) => Value::Int((x as f64).sqrt() as i32),
                        Value::Float(x) => Value::Float(x.sqrt()),
                        _ => unimplemented!(),
                    },
                    Cbrt => match value {
                        Value::Int(x) => Value::Int((x as f64).cbrt() as i32),
                        Value::Float(x) => Value::Float(x.cbrt()),
                        _ => unimplemented!(),
                    },
                    Fort => match value {
                        Value::Int(x) => Value::Int((x as f64).sqrt().sqrt() as i32),
                        Value::Float(x) => Value::Float(x.sqrt().sqrt()),
                        _ => unimplemented!(),
                    },
                }
            }
            Node::Binary(left, op, right) => {
                let l_value = self.visit(*left);
                let r_value = self.visit(*right);

                use BinaryOp::*;
                match op {
                    Add => match l_value {
                        Value::Int(l) => match r_value {
                            Value::Int(r) => Value::Int(l + r),
                            Value::Float(r) => Value::Float((l as f64) + r),
                            _ => unimplemented!(),
                        },
                        Value::Float(l) => match r_value {
                            Value::Int(r) => Value::Float(l + (r as f64)),
                            Value::Float(r) => Value::Float(l + r),
                            _ => unimplemented!(),
                        },
                        _ => unimplemented!(),
                    },
                    Sub => match l_value {
                        Value::Int(l) => match r_value {
                            Value::Int(r) => Value::Int(l - r),
                            Value::Float(r) => Value::Float((l as f64) - r),
                            _ => unimplemented!(),
                        },
                        Value::Float(l) => match r_value {
                            Value::Int(r) => Value::Float(l - (r as f64)),
                            Value::Float(r) => Value::Float(l - r),
                            _ => unimplemented!(),
                        },
                        _ => unimplemented!(),
                    },
                    Mul => match l_value {
                        Value::Int(l) => match r_value {
                            Value::Int(r) => Value::Int(l * r),
                            Value::Float(r) => Value::Float((l as f64) * r),
                            _ => unimplemented!(),
                        },
                        Value::Float(l) => match r_value {
                            Value::Int(r) => Value::Float(l * (r as f64)),
                            Value::Float(r) => Value::Float(l * r),
                            _ => unimplemented!(),
                        },
                        _ => unimplemented!(),
                    },
                    Div => match l_value {
                        Value::Int(l) => match r_value {
                            Value::Int(r) => Value::Int(l / r),
                            Value::Float(r) => Value::Float((l as f64) / r),
                            _ => unimplemented!(),
                        },
                        Value::Float(l) => match r_value {
                            Value::Int(r) => Value::Float(l / (r as f64)),
                            Value::Float(r) => Value::Float(l / r),
                            _ => unimplemented!(),
                        },
                        _ => unimplemented!(),
                    },
                    Rem => match l_value {
                        Value::Int(l) => match r_value {
                            Value::Int(r) => Value::Int(l % r),
                            Value::Float(r) => Value::Float((l as f64) % r),
                            _ => unimplemented!(),
                        },
                        Value::Float(l) => match r_value {
                            Value::Int(r) => Value::Float(l % (r as f64)),
                            Value::Float(r) => Value::Float(l % r),
                            _ => unimplemented!(),
                        },
                        _ => unimplemented!(),
                    },
                    Pow => match l_value {
                        Value::Int(l) => match r_value {
                            Value::Int(r) => Value::Int(l.pow(r as u32)),
                            Value::Float(r) => Value::Float((l as f64).powf(r)),
                            _ => unimplemented!(),
                        },
                        Value::Float(l) => match r_value {
                            Value::Int(r) => Value::Float(l.powi(r)),
                            Value::Float(r) => Value::Float(l.powf(r)),
                            _ => unimplemented!(),
                        },
                        _ => unimplemented!(),
                    },
                }
            }
            Node::Call(name, args) => {
                let arg_values = args
                    .iter()
                    .map(|arg| self.visit(arg.clone()))
                    .collect::<Vec<Value>>();

                let function = self.scope.get(&name);
                match function {
                    Value::Function(function) => function(&arg_values),
                    _ => panic!("{} is not a function", name),
                }
            }
            Node::Statements(nodes) => {
                let mut rtn_value = Value::Int(0);
                for node in nodes {
                    rtn_value = self.visit(node);
                }
                rtn_value
            }
            _ => unimplemented!(),
        }
    }
}

fn factorial(n: i32) -> i32 {
    let mut product = 1;
    for i in 1..=n {
        product *= i;
    }
    product
}
