#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Value {
    Number(f64),
    Vector(Vec<f64>),
    Error(String),
}

impl Value {
    pub fn add(lhs: Value, rhs: Value) -> Value {
        match (lhs, rhs) {
            (Value::Number(ln), Value::Number(rn)) => Value::Number(ln + rn),
            (lhs, rhs) => Value::Error(format!("Operation on {:?} and {:?} not supported", lhs, rhs)),
        }
    }

    pub fn sub(lhs: Value, rhs: Value) -> Value {
        match (lhs, rhs) {
            (Value::Number(ln), Value::Number(rn)) => Value::Number(ln - rn),
            (lhs, rhs) => Value::Error(format!("Operation on {:?} and {:?} not supported", lhs, rhs)),
        }
    }

    pub fn mul(lhs: Value, rhs: Value) -> Value {
        match (lhs, rhs) {
            (Value::Number(ln), Value::Number(rn)) => Value::Number(ln * rn),
            (lhs, rhs) => Value::Error(format!("Operation on {:?} and {:?} not supported", lhs, rhs)),
        }
    }

    pub fn div(lhs: Value, rhs: Value) -> Value {
        match (lhs, rhs) {
            (Value::Number(ln), Value::Number(rn)) => Value::Number(ln / rn),
            (lhs, rhs) => Value::Error(format!("Operation on {:?} and {:?} not supported", lhs, rhs)),
        }
    }

    pub fn pow(lhs: Value, rhs: Value) -> Value {
        match (lhs, rhs) {
            (Value::Number(ln), Value::Number(rn)) => Value::Number(ln.powf(rn)),
            (lhs, rhs) => Value::Error(format!("Operation on {:?} and {:?} not supported", lhs, rhs)),
        }
    }

    pub fn root(lhs: Value, rhs: Value) -> Value {
        match (lhs, rhs) {
            (Value::Number(ln), Value::Number(rn)) => Value::Number(rn.powf(1.0/ln)),
            (lhs, rhs) => Value::Error(format!("Operation on {:?} and {:?} not supported", lhs, rhs)),
        }
    }
}
