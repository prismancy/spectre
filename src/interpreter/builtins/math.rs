use crate::{interpreter::Value, Interpreter};

impl Interpreter {
    pub fn math(&mut self) {
        self.add_var("π", Value::Float(std::f64::consts::PI));
        self.add_var("τ", Value::Float(std::f64::consts::TAU));
        self.add_var("e", Value::Float(std::f64::consts::E));
        self.add_var("𝜑", Value::Float((1.0 + 5.0_f64.sqrt()) / 2.0));
        self.add_var("𝜙", Value::Float((1.0 + 5.0_f64.sqrt()) / 2.0));
        self.add_var("∞", Value::Float(f64::INFINITY));
        self.add_var(
            "abs",
            Value::Function(|args| {
                if args.len() != 1 {
                    panic!("abs expects 1 argument, got {}", args.len());
                }
                match args[0] {
                    Value::Int(value) => Value::Int(value.abs()),
                    Value::Float(value) => Value::Float(value.abs()),
                    _ => panic!("abs expects a number"),
                }
            }),
        );
        self.add_var(
            "floor",
            Value::Function(|args| {
                if args.len() != 1 {
                    panic!("floor expects 1 argument, got {}", args.len());
                }
                match args[0] {
                    Value::Int(value) => Value::Int(value),
                    Value::Float(value) => Value::Float(value.floor()),
                    _ => panic!("floor expects a number"),
                }
            }),
        );
        self.add_var(
            "ceil",
            Value::Function(|args| {
                if args.len() != 1 {
                    panic!("ceil expects 1 argument, got {}", args.len());
                }
                match args[0] {
                    Value::Int(value) => Value::Int(value),
                    Value::Float(value) => Value::Float(value.ceil()),
                    _ => panic!("ceil expects a number"),
                }
            }),
        );
        self.add_var(
            "round",
            Value::Function(|args| {
                if args.len() != 1 {
                    panic!("round expects 1 argument, got {}", args.len());
                }
                match args[0] {
                    Value::Int(value) => Value::Int(value),
                    Value::Float(value) => Value::Float(value.round()),
                    _ => panic!("round expects a number"),
                }
            }),
        );
        self.add_var(
            "trunc",
            Value::Function(|args| {
                if args.len() != 1 {
                    panic!("trunc expects 1 argument, got {}", args.len());
                }
                match args[0] {
                    Value::Int(value) => Value::Int(value),
                    Value::Float(value) => Value::Float(value.trunc()),
                    _ => panic!("trunc expects a number"),
                }
            }),
        );
        self.add_var(
            "fract",
            Value::Function(|args| {
                if args.len() != 1 {
                    panic!("fract expects 1 argument, got {}", args.len());
                }
                match args[0] {
                    Value::Int(value) => Value::Float((value as f64).fract()),
                    Value::Float(value) => Value::Float(value.fract()),
                    _ => panic!("fract expects a number"),
                }
            }),
        );
        self.add_var(
            "sqrt",
            Value::Function(|args| {
                if args.len() != 1 {
                    panic!("sqrt expects 1 argument, got {}", args.len());
                }
                match args[0] {
                    Value::Int(value) => Value::Float((value as f64).sqrt()),
                    Value::Float(value) => Value::Float(value.sqrt()),
                    _ => panic!("sqrt expects a number"),
                }
            }),
        );
        self.add_var(
            "sqrt",
            Value::Function(|args| {
                if args.len() != 1 {
                    panic!("sqrt expects 1 argument, got {}", args.len());
                }
                match args[0] {
                    Value::Int(value) => Value::Float((value as f64).cbrt()),
                    Value::Float(value) => Value::Float(value.cbrt()),
                    _ => panic!("sqrt expects a number"),
                }
            }),
        );
        self.add_var(
            "ln",
            Value::Function(|args| {
                if args.len() != 1 {
                    panic!("ln expects 1 argument, got {}", args.len());
                }
                match args[0] {
                    Value::Int(value) => Value::Float((value as f64).ln()),
                    Value::Float(value) => Value::Float(value.ln()),
                    _ => panic!("ln expects a number"),
                }
            }),
        );
        self.add_var(
            "sin",
            Value::Function(|args| {
                if args.len() != 1 {
                    panic!("sin expects 1 argument, got {}", args.len());
                }
                match args[0] {
                    Value::Int(value) => Value::Float((value as f64).sin()),
                    Value::Float(value) => Value::Float(value.sin()),
                    _ => panic!("sin expects a number"),
                }
            }),
        );
        self.add_var(
            "cos",
            Value::Function(|args| {
                if args.len() != 1 {
                    panic!("cos expects 1 argument, got {}", args.len());
                }
                match args[0] {
                    Value::Int(value) => Value::Float((value as f64).cos()),
                    Value::Float(value) => Value::Float(value.cos()),
                    _ => panic!("cos expects a number"),
                }
            }),
        );
        self.add_var(
            "tan",
            Value::Function(|args| {
                if args.len() != 1 {
                    panic!("tan expects 1 argument, got {}", args.len());
                }
                match args[0] {
                    Value::Int(value) => Value::Float((value as f64).tan()),
                    Value::Float(value) => Value::Float(value.tan()),
                    _ => panic!("tan expects a number"),
                }
            }),
        );
        self.add_var(
            "gcd",
            Value::Function(|args| {
                if args.len() != 2 {
                    panic!("gcd expects 2 arguments, got {}", args.len());
                }
                match args[0] {
                    Value::Int(a) => match args[1] {
                        Value::Int(b) => Value::Int(gcd(a, b)),
                        _ => panic!("gcd expects 2 integers"),
                    },
                    _ => panic!("gcd expects 2 integers"),
                }
            }),
        );
        self.add_var(
            "lcm",
            Value::Function(|args| {
                if args.len() != 2 {
                    panic!("lcm expects 2 arguments, got {}", args.len());
                }
                match args[0] {
                    Value::Int(a) => match args[1] {
                        Value::Int(b) => Value::Int(a * b / gcd(a, b)),
                        _ => panic!("lcm expects 2 integers"),
                    },
                    _ => panic!("lcm expects 2 integers"),
                }
            }),
        );
        self.add_var(
            "min",
            Value::Function(|args| {
                if args.len() != 2 {
                    panic!("min expects 2 arguments, got {}", args.len());
                }
                match args[0] {
                    Value::Int(a) => match args[1] {
                        Value::Int(b) => Value::Int(a.min(b)),
                        _ => panic!("min expects 2 integers"),
                    },
                    Value::Float(a) => match args[1] {
                        Value::Float(b) => Value::Float(a.min(b)),
                        _ => panic!("min expects 2 floats"),
                    },
                    _ => panic!("min expects 2 numbers"),
                }
            }),
        );
        self.add_var(
            "max",
            Value::Function(|args| {
                if args.len() != 2 {
                    panic!("max expects 2 arguments, got {}", args.len());
                }
                match args[0] {
                    Value::Int(a) => match args[1] {
                        Value::Int(b) => Value::Int(a.max(b)),
                        _ => panic!("max expects 2 integers"),
                    },
                    Value::Float(a) => match args[1] {
                        Value::Float(b) => Value::Float(a.max(b)),
                        _ => panic!("max expects 2 floats"),
                    },
                    _ => panic!("max expects 2 numbers"),
                }
            }),
        );
    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
