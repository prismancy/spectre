use crate::{interpreter::Value, Interpreter};

impl Interpreter {
    pub fn math(&mut self) {
        self.add_var("PI", Value::Float(std::f64::consts::PI));

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
    }
}
