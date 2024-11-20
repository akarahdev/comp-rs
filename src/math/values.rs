use std::fmt::{Display, Formatter, Write};

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Value {
    Number(f64),
    Vector(Vec<f64>),
    Error(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(value) => {
                f.write_fmt(format_args!("{}", value))?;
                Ok(())
            }
            Value::Vector(values) => {
                f.write_char('<')?;
                for v in values {
                    f.write_fmt(format_args!("{}, ", v))?;
                    f.write_char(' ')?;
                }
                f.write_char('>')?;
                Ok(())
            }
            Value::Error(err) => {
                f.write_str("error: ")?;
                f.write_char('"')?;
                f.write_str(err.as_str())?;
                f.write_char('"')?;
                Ok(())
            }
        }
    }
}

impl Value {
    pub fn add(lhs: Value, rhs: Value) -> Value {
        match (lhs, rhs) {
            (Value::Number(ln), Value::Number(rn)) => Value::Number(ln + rn),
            (lhs, rhs) => Value::Error(format!(
                "Operation on {:?} and {} not supported",
                lhs, rhs
            )),
        }
    }

    pub fn sub(lhs: Value, rhs: Value) -> Value {
        match (lhs, rhs) {
            (Value::Number(ln), Value::Number(rn)) => Value::Number(ln - rn),
            (lhs, rhs) => Value::Error(format!(
                "Operation on {} and {} not supported",
                lhs, rhs
            )),
        }
    }

    pub fn mul(lhs: Value, rhs: Value) -> Value {
        match (lhs, rhs) {
            (Value::Number(ln), Value::Number(rn)) => Value::Number(ln * rn),
            (lhs, rhs) => Value::Error(format!(
                "Operation on {} and {} not supported",
                lhs, rhs
            )),
        }
    }

    pub fn div(lhs: Value, rhs: Value) -> Value {
        match (lhs, rhs) {
            (Value::Number(ln), Value::Number(rn)) => Value::Number(ln / rn),
            (lhs, rhs) => Value::Error(format!(
                "Operation on {} and {} not supported",
                lhs, rhs
            )),
        }
    }

    pub fn pow(lhs: Value, rhs: Value) -> Value {
        match (lhs, rhs) {
            (Value::Number(ln), Value::Number(rn)) => Value::Number(ln.powf(rn)),
            (lhs, rhs) => Value::Error(format!(
                "Operation on {} and {} not supported",
                lhs, rhs
            )),
        }
    }

    pub fn root(lhs: Value, rhs: Value) -> Value {
        match (lhs, rhs) {
            (Value::Number(ln), Value::Number(rn)) => Value::Number(rn.powf(1.0 / ln)),
            (lhs, rhs) => Value::Error(format!(
                "Operation on {} and {} not supported",
                lhs, rhs
            )),
        }
    }
}
