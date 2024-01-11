use crate::{Interpreter, Value};

impl Interpreter {
    pub fn io(&mut self) {
        self.add_var(
            "print",
            Value::NativeFunction(|args| {
                println!("{}", args[0]);
                Value::Int(0)
            }),
        );
    }
}
