use std::collections::HashMap;

use crate::interpreter::Value;

#[derive(Default)]
pub struct Scope {
    pub variables: HashMap<String, Value>,
}

impl Scope {
    pub fn get(&self, name: &str) -> Value {
        match self.variables.get(name) {
            Some(value) => value.clone(),
            None => panic!("{} is not defined", name),
        }
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
}
