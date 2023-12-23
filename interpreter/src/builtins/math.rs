use crate::{Interpreter, Value};

impl Interpreter {
    pub fn math(&mut self) {
        macro_rules! add_var {
            ($name:literal, $value:expr) => {
                self.add_var($name, Value::from($value));
            };
        }

        add_var!("π", std::f64::consts::PI);
        add_var!("τ", std::f64::consts::TAU);
        add_var!("e", std::f64::consts::E);
        add_var!("𝜑", (1.0 + 5.0_f64.sqrt()) / 2.0);
        add_var!("𝜙", (1.0 + 5.0_f64.sqrt()) / 2.0);
        add_var!("∞", f64::INFINITY);
        add_var!("i", (0.0, 1.0));

        macro_rules! add_fn {
            ($name:literal, $value:expr) => {
                self.add_var($name, Value::NativeFunction($value));
            };
        }

        add_fn!("abs", |args| {
            if args.len() != 1 {
                panic!("abs expects 1 argument, got {}", args.len());
            }
            match args[0] {
                Value::Int(value) => Value::Int(value.abs()),
                Value::Float(value) => Value::Float(value.abs()),
                _ => panic!("abs expects a number"),
            }
        });
        add_fn!("floor", |args| {
            if args.len() != 1 {
                panic!("floor expects 1 argument, got {}", args.len());
            }
            match args[0] {
                Value::Int(value) => Value::Int(value),
                Value::Float(value) => Value::Float(value.floor()),
                _ => panic!("floor expects a number"),
            }
        });
        add_fn!("ceil", |args| {
            if args.len() != 1 {
                panic!("ceil expects 1 argument, got {}", args.len());
            }
            match args[0] {
                Value::Int(value) => Value::Int(value),
                Value::Float(value) => Value::Float(value.ceil()),
                _ => panic!("ceil expects a number"),
            }
        });
        add_fn!("round", |args| {
            if args.len() != 1 {
                panic!("round expects 1 argument, got {}", args.len());
            }
            match args[0] {
                Value::Int(value) => Value::Int(value),
                Value::Float(value) => Value::Float(value.round()),
                _ => panic!("round expects a number"),
            }
        });
        add_fn!("trunc", |args| {
            if args.len() != 1 {
                panic!("trunc expects 1 argument, got {}", args.len());
            }
            match args[0] {
                Value::Int(value) => Value::Int(value),
                Value::Float(value) => Value::Float(value.trunc()),
                _ => panic!("trunc expects a number"),
            }
        });
        add_fn!("fract", |args| {
            if args.len() != 1 {
                panic!("fract expects 1 argument, got {}", args.len());
            }
            match args[0] {
                Value::Int(value) => Value::Float((value as f64).fract()),
                Value::Float(value) => Value::Float(value.fract()),
                _ => panic!("fract expects a number"),
            }
        });
        add_fn!("sqrt", |args| {
            if args.len() != 1 {
                panic!("sqrt expects 1 argument, got {}", args.len());
            }
            match args[0] {
                Value::Int(value) => Value::Float((value as f64).sqrt()),
                Value::Float(value) => Value::Float(value.sqrt()),
                _ => panic!("sqrt expects a number"),
            }
        });
        add_fn!("sqrt", |args| {
            if args.len() != 1 {
                panic!("sqrt expects 1 argument, got {}", args.len());
            }
            match args[0] {
                Value::Int(value) => Value::Float((value as f64).cbrt()),
                Value::Float(value) => Value::Float(value.cbrt()),
                _ => panic!("sqrt expects a number"),
            }
        });
        add_fn!("ln", |args| {
            if args.len() != 1 {
                panic!("ln expects 1 argument, got {}", args.len());
            }
            match args[0] {
                Value::Int(value) => Value::Float((value as f64).ln()),
                Value::Float(value) => Value::Float(value.ln()),
                _ => panic!("ln expects a number"),
            }
        });
        add_fn!("sin", |args| {
            if args.len() != 1 {
                panic!("sin expects 1 argument, got {}", args.len());
            }
            match args[0] {
                Value::Int(value) => Value::Float((value as f64).sin()),
                Value::Float(value) => Value::Float(value.sin()),
                _ => panic!("sin expects a number"),
            }
        });
        add_fn!("cos", |args| {
            if args.len() != 1 {
                panic!("cos expects 1 argument, got {}", args.len());
            }
            match args[0] {
                Value::Int(value) => Value::Float((value as f64).cos()),
                Value::Float(value) => Value::Float(value.cos()),
                _ => panic!("cos expects a number"),
            }
        });
        add_fn!("tan", |args| {
            if args.len() != 1 {
                panic!("tan expects 1 argument, got {}", args.len());
            }
            match args[0] {
                Value::Int(value) => Value::Float((value as f64).tan()),
                Value::Float(value) => Value::Float(value.tan()),
                _ => panic!("tan expects a number"),
            }
        });
        add_fn!("gcd", |args| {
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
        });
        add_fn!("lcm", |args| {
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
        });
        add_fn!("min", |args| {
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
        });
        add_fn!("max", |args| {
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
        });
        add_fn!("clamp", |args| {
            if args.len() != 3 {
                panic!("clamp expects 3 arguments, got {}", args.len());
            }
            match args[0] {
                Value::Int(a) => match args[1] {
                    Value::Int(b) => match args[2] {
                        Value::Int(c) => Value::Int(a.max(b).min(c)),
                        _ => panic!("clamp expects 3 integers"),
                    },
                    _ => panic!("clamp expects 3 integers"),
                },
                Value::Float(a) => match args[1] {
                    Value::Float(b) => match args[2] {
                        Value::Float(c) => Value::Float(a.max(b).min(c)),
                        _ => panic!("clamp expects 3 floats"),
                    },
                    _ => panic!("clamp expects 3 floats"),
                },
                _ => panic!("clamp expects 3 numbers"),
            }
        });
        add_fn!("Re", |args| {
            if args.len() != 1 {
                panic!("Re expects 1 argument, got {}", args.len());
            }
            match args[0] {
                Value::Complex(re, _) => Value::Float(re),
                _ => panic!("Re expects a complex number"),
            }
        });
        add_fn!("Im", |args| {
            if args.len() != 1 {
                panic!("Im expects 1 argument, got {}", args.len());
            }
            match args[0] {
                Value::Complex(_, im) => Value::Float(im),
                _ => panic!("Im expects a complex number"),
            }
        });
        fn arg(args: &Vec<Value>) -> Value {
            if args.len() != 1 {
                panic!("arg expects 1 argument, got {}", args.len());
            }
            match args[0] {
                Value::Complex(re, im) => Value::Float(im.atan2(re)),
                _ => panic!("arg expects a complex number"),
            }
        }
        add_fn!("arg", arg);
        add_fn!("phase", arg);
        add_fn!("conj", |args| {
            if args.len() != 1 {
                panic!("conj expects 1 argument, got {}", args.len());
            }
            match args[0] {
                Value::Complex(re, im) => Value::Complex(re, -im),
                _ => panic!("conj expects a complex number"),
            }
        });
        add_fn!("cis", |args| {
            if args.len() != 1 {
                panic!("cis expects 1 argument, got {}", args.len());
            }
            match args[0] {
                Value::Float(x) => Value::Complex(x.cos(), x.sin()),
                _ => panic!("cis expects a number"),
            }
        });
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
