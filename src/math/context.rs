use crate::math::values::Value;
use num::complex::Complex64;
use std::collections::HashMap;

pub struct Context {
    variables: HashMap<String, Value>,
}

impl Default for Context {
    fn default() -> Context {
        let mut ctx = Context::new();
        ctx.set_variable("i".to_string(), Value::Number(Complex64::new(0.0, 1.0)));
        ctx
    }
}

impl Context {
    pub fn new() -> Self {
        Context {
            variables: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn resolve_variable(&self, name: &String) -> Option<&Value> {
        self.variables.get(name)
    }
}
