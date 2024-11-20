use std::collections::HashMap;
use crate::math::values::Value;

pub struct Context {
    variables: HashMap<String, Value>
}

impl Context {
    pub fn new() -> Self {
        Context {
            variables: HashMap::new()
        }
    }

    pub fn set_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn resolve_variable(&self, name: &String) -> Option<&Value> {
        self.variables.get(name)
    }
}