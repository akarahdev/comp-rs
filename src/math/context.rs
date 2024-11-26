use crate::math::values::Value;
use num::complex::Complex64;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

static GLOBAL_MATH_CONTEXT: LazyLock<Mutex<Context>> = LazyLock::new(Mutex::default);

pub struct GlobalContext;
impl GlobalContext {
    pub fn set_variable(name: String, value: Value) {
        GLOBAL_MATH_CONTEXT.lock().unwrap().set_variable(name, value);
    }

    pub fn resolve_variable(name: &String) -> Option<Value> {
        GLOBAL_MATH_CONTEXT.lock().unwrap().resolve_variable(name).cloned()
    }
}


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
