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
            Node::Assignment(name, node) => {
                let value = self.visit(*node);
                self.scope.set(name, value.clone());
                value
            }
            Node::Unary(op, node) => {
                let value = self.visit(*node);

                use UnaryOp::*;
                match op {
                    Pos => value,
                    Neg => match value {
                        Value::Int(x) => Value::Int(-x),
                        Value::Float(x) => Value::Float(-x),
                        Value::Complex(r, i) => Value::Complex(-r, -i),
                        _ => unimplemented!(),
                    },
                    Abs => match value {
                        Value::Int(x) => Value::Int(x.abs()),
                        Value::Float(x) => Value::Float(x.abs()),
                        Value::Complex(r, i) => Value::Float(r.hypot(i)),
                        _ => unimplemented!(),
                    },
                    Floor => match value {
                        Value::Int(x) => Value::Int(x),
                        Value::Float(x) => Value::Float(x.floor()),
                        _ => unimplemented!(),
                    },
                    Ceil => match value {
                        Value::Int(x) => Value::Int(x),
                        Value::Float(x) => Value::Float(x.ceil()),
                        _ => unimplemented!(),
                    },
                    Round => match value {
                        Value::Int(x) => Value::Int(x),
                        Value::Float(x) => Value::Float(x.round()),
                        _ => unimplemented!(),
                    },
                    Degree => match value {
                        Value::Int(x) => Value::Float((x as f64).to_radians()),
                        Value::Float(x) => Value::Float(x.to_radians()),
                        _ => unimplemented!(),
                    },
                    Fact => match value {
                        Value::Int(x) => Value::Int(factorial(x)),
                        Value::Float(x) => Value::Int(factorial(x as i32)),
                        _ => unimplemented!(),
                    },
                    Sqrt => match value {
                        Value::Int(x) => Value::Float((x as f64).sqrt()),
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

                macro_rules! simple_binary_op {
                    ($op:tt) => {
                        match l_value {
                            Value::Int(a) => match r_value {
                                Value::Int(b) => Value::Int(a $op b),
                                Value::Float(b) => Value::Float((a as f64) $op b),
                                Value::Complex(r, i) => Value::Complex((a as f64) $op r, i),
                                _ => unimplemented!(),
                            },
                            Value::Float(a) => match r_value {
                                Value::Int(b) => Value::Float(a $op (b as f64)),
                                Value::Float(b) => Value::Float(a $op b),
                                Value::Complex(r, i) => Value::Complex(a $op r, i),
                                _ => unimplemented!(),
                            },
                            Value::Complex(r, i) => match r_value {
                                Value::Int(x) => Value::Complex(r $op (x as f64), i),
                                Value::Float(x) => Value::Complex(r $op x, i),
                                Value::Complex(r2, i2) => Value::Complex(r $op r2, i $op i2),
                                _ => unimplemented!(),
                            },
                            _ => unimplemented!(),
                        }
                    };
                }

                use BinaryOp::*;
                match op {
                    Add => simple_binary_op!(+),
                    Sub => simple_binary_op!(-),
                    Mul => match l_value {
                        Value::Int(a) => match r_value {
                            Value::Int(b) => Value::Int(a * b),
                            Value::Float(b) => Value::Float((a as f64) * b),
                            Value::Complex(r, i) => Value::Complex((a as f64) * r, (a as f64) * i),
                            _ => unimplemented!(),
                        },
                        Value::Float(a) => match r_value {
                            Value::Int(b) => Value::Float(a * (b as f64)),
                            Value::Float(b) => Value::Float(a * b),
                            Value::Complex(r, i) => Value::Complex(a * r, a * i),
                            _ => unimplemented!(),
                        },
                        Value::Complex(r, i) => match r_value {
                            Value::Int(x) => Value::Complex(r * (x as f64), i),
                            Value::Float(x) => Value::Complex(r * x, i),
                            Value::Complex(r2, i2) => {
                                let ii = i * i2;
                                Value::Complex(r * r2 - ii, r * i2 + ii)
                            }
                            _ => unimplemented!(),
                        },
                        _ => unimplemented!(),
                    },
                    Div => match l_value {
                        Value::Int(a) => match r_value {
                            Value::Int(b) => Value::Int(a / b),
                            Value::Float(b) => Value::Float((a as f64) / b),
                            Value::Complex(r, i) => Value::Complex((a as f64) / r, (a as f64) / i),
                            _ => unimplemented!(),
                        },
                        Value::Float(a) => match r_value {
                            Value::Int(b) => Value::Float(a / (b as f64)),
                            Value::Float(b) => Value::Float(a / b),
                            Value::Complex(r, i) => Value::Complex(a / r, a / i),
                            _ => unimplemented!(),
                        },
                        Value::Complex(r, i) => match r_value {
                            Value::Int(x) => Value::Complex(r / (x as f64), i),
                            Value::Float(x) => Value::Complex(r / x, i),
                            Value::Complex(r2, i2) => Value::Complex(
                                (r * r2 + i * i2) / (r2 * r2 + i2 * i2),
                                (i * r2 - r * i2) / (r2 * r2 + i2 * i2),
                            ),
                            _ => unimplemented!(),
                        },
                        _ => unimplemented!(),
                    },
                    Rem => match l_value {
                        Value::Int(a) => match r_value {
                            Value::Int(b) => Value::Int(a % b),
                            Value::Float(b) => Value::Float((a as f64) % b),
                            _ => unimplemented!(),
                        },
                        Value::Float(a) => match r_value {
                            Value::Int(b) => Value::Float(a % (b as f64)),
                            Value::Float(b) => Value::Float(a % b),
                            _ => unimplemented!(),
                        },
                        _ => unimplemented!(),
                    },
                    Pow => match l_value {
                        Value::Int(a) => match r_value {
                            Value::Int(b) => Value::Int(a.pow(b as u32)),
                            Value::Float(b) => Value::Float((a as f64).powf(b)),
                            Value::Complex(r, i) => {
                                let r = (a as f64).powf(r);
                                let i = (a as f64).powf(i);
                                Value::Complex(r * i.cos(), r * i.sin())
                            }
                            _ => unimplemented!(),
                        },
                        Value::Float(a) => match r_value {
                            Value::Int(b) => Value::Float(a.powi(b)),
                            Value::Float(b) => Value::Float(a.powf(b)),
                            Value::Complex(r, i) => {
                                let r = a.powf(r);
                                let i = a.powf(i);
                                Value::Complex(r * i.cos(), r * i.sin())
                            }
                            _ => unimplemented!(),
                        },
                        Value::Complex(r, i) => match r_value {
                            Value::Int(x) => {
                                let r = r.powi(x);
                                let i = i.powi(x);
                                Value::Complex(r * i.cos(), r * i.sin())
                            }
                            Value::Float(x) => {
                                let r = r.powf(x);
                                let i = i.powf(x);
                                Value::Complex(r * i.cos(), r * i.sin())
                            }
                            Value::Complex(r2, i2) => {
                                let r = r.hypot(i);
                                let i = i.atan2(r);
                                let r2 = r2.hypot(i2);
                                let i2 = i2.atan2(r2);
                                let r = r.powf(r2);
                                let i = i.powf(i2);
                                Value::Complex(r * i.cos(), r * i.sin())
                            }
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
            Node::EOF => Value::Int(0),
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
