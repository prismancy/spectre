use std::rc::Rc;

use parser::{BinaryOp, Node, UnaryOp};

use crate::{Scope, Value};

pub struct Interpreter {
    pub scope: Scope,
}

impl Default for Interpreter {
    fn default() -> Self {
        let mut interpreter = Self {
            scope: Scope::default(),
        };
        interpreter.math();
        interpreter.io();
        interpreter
    }
}

impl Interpreter {
    pub fn run(&mut self, ast: Node) -> Value {
        self.visit(ast)
    }

    pub fn add_var(&mut self, name: &str, value: Value) {
        self.scope.set(name.into(), value);
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
                        Value::Int(x) => Value::Int((1..=x).product()),
                        Value::Float(x) => Value::Int((1..=(x as i32)).product()),
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
                    Not => Value::Bool(!(bool::from(value))),
                }
            }
            Node::Binary(left, op, right) => {
                let l_value = self.visit(*left);
                let r_value = self.visit(*right);

                macro_rules! simple_binary_op {
                    ($op:tt) => {
                        match (l_value, r_value) {
                            (Value::Int(a), Value::Int(b)) => Value::Int(a $op b),
                            (Value::Int(a), Value::Float(b)) => Value::Float((a as f64) $op b),
                            (Value::Int(a), Value::Complex(r, i)) => Value::Complex((a as f64) $op r, i),
                            (Value::Float(a), Value::Int(b)) => Value::Float(a $op (b as f64)),
                            (Value::Float(a), Value::Float(b)) => Value::Float(a $op b),
                            (Value::Float(a), Value::Complex(r, i)) => Value::Complex(a $op r, i),
                            (Value::Complex(r, i), Value::Int(x)) => Value::Complex(r $op (x as f64), i),
                            (Value::Complex(r, i), Value::Float(x)) => Value::Complex(r $op x, i),
                            (Value::Complex(r, i), Value::Complex(r2, i2)) => Value::Complex(r $op r2, i $op i2),
                            _ => unimplemented!(),
                        }
                    };
                }

                use BinaryOp::*;
                match op {
                    Add => simple_binary_op!(+),
                    Sub => simple_binary_op!(-),
                    Mul => match (l_value, r_value) {
                        (Value::Int(a), Value::Int(b)) => Value::Int(a * b),
                        (Value::Int(a), Value::Float(b)) => Value::Float((a as f64) * b),
                        (Value::Int(a), Value::Complex(r, i)) => {
                            Value::Complex((a as f64) * r, (a as f64) * i)
                        }
                        (Value::Int(a), Value::Function(name, arg_names, body)) => Value::Function(
                            name,
                            arg_names,
                            Box::new(Node::Binary(Box::new(Node::Int(a)), Mul, body)),
                        ),
                        (Value::Float(a), Value::Int(b)) => Value::Float(a * (b as f64)),
                        (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
                        (Value::Float(a), Value::Complex(r, i)) => Value::Complex(a * r, a * i),
                        (Value::Float(a), Value::Function(name, arg_names, body)) => {
                            Value::Function(
                                name,
                                arg_names,
                                Box::new(Node::Binary(Box::new(Node::Float(a)), Mul, body)),
                            )
                        }
                        (Value::Complex(r, i), Value::Int(x)) => Value::Complex(r * (x as f64), i),
                        (Value::Complex(r, i), Value::Float(x)) => Value::Complex(r * x, i),
                        (Value::Complex(r, i), Value::Complex(r2, i2)) => {
                            let ii = i * i2;
                            Value::Complex(r * r2 - ii, r * i2 + ii)
                        }
                        (Value::Function(name, arg_names, body), Value::Int(a)) => Value::Function(
                            name,
                            arg_names,
                            Box::new(Node::Binary(body, Mul, Box::new(Node::Int(a)))),
                        ),
                        (Value::Function(name, arg_names, body), Value::Float(a)) => {
                            Value::Function(
                                name,
                                arg_names,
                                Box::new(Node::Binary(body, Mul, Box::new(Node::Float(a)))),
                            )
                        }
                        (l, r) => panic!("Cannot multiply {} by {}", l, r),
                    },
                    Div => match (l_value, r_value) {
                        (Value::Int(a), Value::Int(b)) => Value::Int(a / b),
                        (Value::Int(a), Value::Float(b)) => Value::Float((a as f64) / b),
                        (Value::Int(a), Value::Complex(r, i)) => {
                            Value::Complex((a as f64) / r, (a as f64) / i)
                        }
                        (Value::Int(a), Value::Function(name, arg_names, body)) => Value::Function(
                            name,
                            arg_names,
                            Box::new(Node::Binary(Box::new(Node::Int(a)), Div, body)),
                        ),
                        (Value::Float(a), Value::Int(b)) => Value::Float(a / (b as f64)),
                        (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
                        (Value::Float(a), Value::Complex(r, i)) => Value::Complex(a / r, a / i),
                        (Value::Float(a), Value::Function(name, arg_names, body)) => {
                            Value::Function(
                                name,
                                arg_names,
                                Box::new(Node::Binary(Box::new(Node::Float(a)), Mul, body)),
                            )
                        }
                        (Value::Complex(r, i), Value::Int(x)) => Value::Complex(r / (x as f64), i),
                        (Value::Complex(r, i), Value::Float(x)) => Value::Complex(r / x, i),
                        (Value::Complex(r, i), Value::Complex(r2, i2)) => Value::Complex(
                            (r * r2 + i * i2) / (r2 * r2 + i2 * i2),
                            (i * r2 - r * i2) / (r2 * r2 + i2 * i2),
                        ),
                        (Value::Function(name, arg_names, body), Value::Int(a)) => Value::Function(
                            name,
                            arg_names,
                            Box::new(Node::Binary(body, Div, Box::new(Node::Int(a)))),
                        ),
                        (Value::Function(name, arg_names, body), Value::Float(a)) => {
                            Value::Function(
                                name,
                                arg_names,
                                Box::new(Node::Binary(body, Div, Box::new(Node::Float(a)))),
                            )
                        }
                        (l, r) => panic!("Cannot divide {} by {}", l, r),
                    },
                    Rem => match (l_value, r_value) {
                        (Value::Int(a), Value::Int(b)) => Value::Int(a % b),
                        (Value::Int(a), Value::Float(b)) => Value::Float((a as f64) % b),
                        (Value::Int(a), Value::Function(name, arg_names, body)) => Value::Function(
                            name,
                            arg_names,
                            Box::new(Node::Binary(Box::new(Node::Int(a)), Div, body)),
                        ),
                        (Value::Float(a), Value::Int(b)) => Value::Float(a % (b as f64)),
                        (Value::Float(a), Value::Float(b)) => Value::Float(a % b),
                        (Value::Float(a), Value::Function(name, arg_names, body)) => {
                            Value::Function(
                                name,
                                arg_names,
                                Box::new(Node::Binary(Box::new(Node::Float(a)), Rem, body)),
                            )
                        }
                        (Value::Function(name, arg_names, body), Value::Int(a)) => Value::Function(
                            name,
                            arg_names,
                            Box::new(Node::Binary(body, Rem, Box::new(Node::Int(a)))),
                        ),
                        (Value::Function(name, arg_names, body), Value::Float(a)) => {
                            Value::Function(
                                name,
                                arg_names,
                                Box::new(Node::Binary(body, Rem, Box::new(Node::Float(a)))),
                            )
                        }
                        (l, r) => panic!("Cannot take remainder of {} and {}", l, r),
                    },
                    Pow => match (l_value, r_value) {
                        (Value::Int(a), Value::Int(b)) => Value::Int(a.pow(b as u32)),
                        (Value::Int(a), Value::Float(b)) => Value::Float((a as f64).powf(b)),
                        (Value::Int(a), Value::Complex(r, i)) => {
                            let r = (a as f64).powf(r);
                            let i = (a as f64).powf(i);
                            Value::Complex(r * i.cos(), r * i.sin())
                        }
                        (Value::Int(a), Value::Function(name, arg_names, body)) => Value::Function(
                            name,
                            arg_names,
                            Box::new(Node::Binary(Box::new(Node::Int(a)), Pow, body)),
                        ),
                        (Value::Float(a), Value::Int(b)) => Value::Float(a.powi(b)),
                        (Value::Float(a), Value::Float(b)) => Value::Float(a.powf(b)),
                        (Value::Float(a), Value::Complex(r, i)) => {
                            let r = a.powf(r);
                            let i = a.powf(i);
                            Value::Complex(r * i.cos(), r * i.sin())
                        }
                        (Value::Float(a), Value::Function(name, arg_names, body)) => {
                            Value::Function(
                                name,
                                arg_names,
                                Box::new(Node::Binary(Box::new(Node::Float(a)), Pow, body)),
                            )
                        }
                        (Value::Complex(r, i), Value::Int(x)) => {
                            let r = r.powi(x);
                            let i = i.powi(x);
                            Value::Complex(r * i.cos(), r * i.sin())
                        }
                        (Value::Complex(r, i), Value::Float(x)) => {
                            let r = r.powf(x);
                            let i = i.powf(x);
                            Value::Complex(r * i.cos(), r * i.sin())
                        }
                        (Value::Complex(r, i), Value::Complex(r2, i2)) => {
                            let r = r.hypot(i);
                            let i = i.atan2(r);
                            let r2 = r2.hypot(i2);
                            let i2 = i2.atan2(r2);
                            let r = r.powf(r2);
                            let i = i.powf(i2);
                            Value::Complex(r * i.cos(), r * i.sin())
                        }
                        (Value::Function(name, arg_names, body), Value::Int(a)) => Value::Function(
                            name,
                            arg_names,
                            Box::new(Node::Binary(body, Pow, Box::new(Node::Int(a)))),
                        ),
                        (Value::Function(name, arg_names, body), Value::Float(a)) => {
                            Value::Function(
                                name,
                                arg_names,
                                Box::new(Node::Binary(body, Pow, Box::new(Node::Float(a)))),
                            )
                        }
                        (l, r) => panic!("Cannot raise {} to the power of {}", l, r),
                    },
                    EqEq => Value::Bool(l_value == r_value),
                    Neq => Value::Bool(l_value != r_value),
                    Lt => Value::Bool(match (l_value, r_value) {
                        (Value::Int(a), Value::Int(b)) => a < b,
                        (Value::Int(a), Value::Float(b)) => (a as f64) < b,
                        (Value::Int(x), Value::Complex(r, i)) => ((x * x) as f64) < (r * r + i * i),
                        (Value::Float(a), Value::Int(b)) => a < (b as f64),
                        (Value::Float(a), Value::Float(b)) => a < b,
                        (Value::Float(x), Value::Complex(r, i)) => (x * x) < (r * r + i * i),
                        (Value::Complex(r, i), Value::Int(x)) => (r * r + i * i) < ((x * x) as f64),
                        (Value::Complex(r, i), Value::Float(x)) => (r * r + i * i) < (x * x),
                        (Value::Complex(r, i), Value::Complex(r2, i2)) => {
                            (r * r + i * i) < (r2 * r2 + i2 * i2)
                        }
                        (l, r) => panic!("Cannot compare {} {} {}", l, Lt, r),
                    }),
                    Lte => Value::Bool(match (l_value, r_value) {
                        (Value::Int(a), Value::Int(b)) => a <= b,
                        (Value::Int(a), Value::Float(b)) => (a as f64) <= b,
                        (Value::Int(x), Value::Complex(r, i)) => {
                            ((x * x) as f64) <= (r * r + i * i)
                        }
                        (Value::Float(a), Value::Int(b)) => a <= (b as f64),
                        (Value::Float(a), Value::Float(b)) => a <= b,
                        (Value::Float(x), Value::Complex(r, i)) => (x * x) <= (r * r + i * i),
                        (Value::Complex(r, i), Value::Int(x)) => {
                            (r * r + i * i) <= ((x * x) as f64)
                        }
                        (Value::Complex(r, i), Value::Float(x)) => (r * r + i * i) <= (x * x),
                        (Value::Complex(r, i), Value::Complex(r2, i2)) => {
                            (r * r + i * i) <= (r2 * r2 + i2 * i2)
                        }
                        (l, r) => panic!("Cannot compare {} {} {}", l, Lte, r),
                    }),
                    Gt => Value::Bool(match (l_value, r_value) {
                        (Value::Int(a), Value::Int(b)) => a > b,
                        (Value::Int(a), Value::Float(b)) => (a as f64) > b,
                        (Value::Int(x), Value::Complex(r, i)) => ((x * x) as f64) > (r * r + i * i),
                        (Value::Float(a), Value::Int(b)) => a > (b as f64),
                        (Value::Float(a), Value::Float(b)) => a > b,
                        (Value::Float(x), Value::Complex(r, i)) => (x * x) > (r * r + i * i),
                        (Value::Complex(r, i), Value::Int(x)) => (r * r + i * i) > ((x * x) as f64),
                        (Value::Complex(r, i), Value::Float(x)) => (r * r + i * i) > (x * x),
                        (Value::Complex(r, i), Value::Complex(r2, i2)) => {
                            (r * r + i * i) > (r2 * r2 + i2 * i2)
                        }
                        (l, r) => panic!("Cannot compare {} {} {}", l, Gt, r),
                    }),
                    Gte => Value::Bool(match (l_value, r_value) {
                        (Value::Int(a), Value::Int(b)) => a >= b,
                        (Value::Int(a), Value::Float(b)) => (a as f64) >= b,
                        (Value::Int(x), Value::Complex(r, i)) => {
                            ((x * x) as f64) >= (r * r + i * i)
                        }
                        (Value::Float(a), Value::Int(b)) => a >= (b as f64),
                        (Value::Float(a), Value::Float(b)) => a >= b,
                        (Value::Float(x), Value::Complex(r, i)) => (x * x) >= (r * r + i * i),
                        (Value::Complex(r, i), Value::Int(x)) => {
                            (r * r + i * i) >= ((x * x) as f64)
                        }
                        (Value::Complex(r, i), Value::Float(x)) => (r * r + i * i) >= (x * x),
                        (Value::Complex(r, i), Value::Complex(r2, i2)) => {
                            (r * r + i * i) >= (r2 * r2 + i2 * i2)
                        }
                        (l, r) => panic!("Cannot compare {} {} {}", l, Gte, r),
                    }),
                    And => Value::Bool(l_value.into() && r_value.into()),
                    Or => Value::Bool(l_value.into() || r_value.into()),
                }
            }
            Node::If(cond, then, else_case) => {
                let cond = self.visit(*cond);
                if bool::from(cond) {
                    self.visit(*then)
                } else {
                    match else_case {
                        Some(else_case) => self.visit(*else_case),
                        None => Value::Int(0),
                    }
                }
            }
            Node::While(cond, body) => {
                let mut rtn_value = Value::Int(0);
                while bool::from(self.visit(*cond.clone())) {
                    rtn_value = self.visit(*body.clone());
                }
                rtn_value
            }
            Node::FnDef(name, arg_names, node) => {
                self.scope
                    .set(Rc::clone(&name), Value::Function(name, arg_names, node));
                Value::Int(0)
            }
            Node::Call(name, args) => {
                let arg_values = args
                    .into_iter()
                    .map(|arg| self.visit(arg))
                    .collect::<Vec<Value>>();

                let function = self.scope.get(&name);
                match function {
                    Value::Function(_, arg_names, body) => {
                        let mut interpreter = Interpreter::default();
                        for (name, value) in arg_names.iter().zip(arg_values) {
                            interpreter.add_var(name, value.clone());
                        }
                        interpreter.visit(*body)
                    }
                    Value::NativeFunction(function) => function(&arg_values),
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
            Node::Eof => Value::Int(0),
        }
    }
}
