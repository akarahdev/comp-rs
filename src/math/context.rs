use crate::math::values::Value;
use num::complex::Complex64;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

pub static GLOBAL_MATH_CONTEXT: LazyLock<Mutex<Context>> = LazyLock::new(Mutex::default);

pub struct GlobalContext;
impl GlobalContext {
    pub fn set_variable(name: String, value: Value) {
        GLOBAL_MATH_CONTEXT.lock().unwrap().set_variable(name, value);
    }

    pub fn resolve_variable(name: &String) -> Option<Value> {
        GLOBAL_MATH_CONTEXT.lock().unwrap().resolve_variable(name).clone()
    }
}


pub struct Context {
    pub(crate) frames: Vec<Frame>
}

pub struct Frame {
    pub(crate) variables: HashMap<String, Value>,
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
            frames: vec![Frame {
                variables: HashMap::new()
            }]
        }
    }

    pub fn push_frame(&mut self) {
        self.frames.push(Frame {
            variables: HashMap::new(),
        })
    }

    pub fn pop_frame(&mut self) {
        self.frames.pop();
    }

    pub fn set_variable(&mut self, name: String, value: Value) {
        self.frames.last_mut().unwrap().variables.insert(name, value);
    }

    pub fn resolve_variable(&self, name: &String) -> Option<Value> {
        for frame in self.frames.iter().rev() {
            match frame.variables.get(name) {
                Some(value) => return Some(value.clone()),
                None => ()
            }
        }
        None
    }
}
