use num::complex::{Complex64, ComplexFloat};
use num::traits::real::Real;
use std::fmt::{Display, Formatter, Write};

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Value {
    Number(Complex64),
    Vector(Vec<Value>),
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
    pub fn add(lhs: &Value, rhs: &Value) -> Value {
        match (lhs, rhs) {
            (Value::Number(ln), Value::Number(rn)) => Value::Number(ln + rn),
            (Value::Vector(lhsv), Value::Vector(rhsv)) => {
                if lhsv.len() != rhsv.len() {
                    return Value::Error("vectors must have same length to be added".to_string());
                }
                Value::Vector(
                    lhsv.iter()
                        .zip(rhsv.iter())
                        .map(|(x, y)| Value::add(x, y))
                        .collect(),
                )
            }
            (lhs, rhs) => Value::Error(format!("Operation on {} and {} not supported", lhs, rhs)),
        }
    }

    pub fn sub(lhs: &Value, rhs: &Value) -> Value {
        match (lhs, rhs) {
            (Value::Number(ln), Value::Number(rn)) => Value::Number(ln - rn),
            (Value::Vector(lhsv), Value::Vector(rhsv)) => {
                if lhsv.len() != rhsv.len() {
                    return Value::Error(
                        "vectors must have same length to be subtracted".to_string(),
                    );
                }
                Value::Vector(
                    lhsv.iter()
                        .zip(rhsv.iter())
                        .map(|(x, y)| Value::sub(x, y))
                        .collect(),
                )
            }
            (lhs, rhs) => Value::Error(format!("Operation on {} and {} not supported", lhs, rhs)),
        }
    }

    pub fn mul(lhs: &Value, rhs: &Value) -> Value {
        match (lhs, rhs) {
            (Value::Number(ln), Value::Number(rn)) => Value::Number(ln * rn),
            (Value::Vector(lhsv), Value::Vector(rhsv)) => {
                if lhsv.len() != rhsv.len() {
                    return Value::Error(
                        "vectors must have same length to be multiplied".to_string(),
                    );
                }
                Value::Vector(
                    lhsv.iter()
                        .zip(rhsv.iter())
                        .map(|(x, y)| Value::mul(x, y))
                        .collect(),
                )
            }
            (lhs, rhs) => Value::Error(format!("Operation on {} and {} not supported", lhs, rhs)),
        }
    }

    pub fn div(lhs: &Value, rhs: &Value) -> Value {
        match (lhs, rhs) {
            (Value::Number(ln), Value::Number(rn)) => Value::Number(ln / rn),
            (Value::Vector(lhsv), Value::Vector(rhsv)) => {
                if lhsv.len() != rhsv.len() {
                    return Value::Error("vectors must have same length to be divided".to_string());
                }
                Value::Vector(
                    lhsv.iter()
                        .zip(rhsv.iter())
                        .map(|(x, y)| Value::div(x, y))
                        .collect(),
                )
            }
            (lhs, rhs) => Value::Error(format!("Operation on {} and {} not supported", lhs, rhs)),
        }
    }

    pub fn pow(lhs: &Value, rhs: &Value) -> Value {
        match (lhs, rhs) {
            (Value::Number(ln), Value::Number(rn)) => Value::Number(ln.powc(*rn)),
            (lhs, rhs) => Value::Error(format!("Operation on {} and {} not supported", lhs, rhs)),
        }
    }

    pub fn root(lhs: &Value, rhs: &Value) -> Value {
        match (lhs, rhs) {
            (Value::Number(ln), Value::Number(rn)) => Value::Number(rn.powc(1.0 / ln)),
            (lhs, rhs) => Value::Error(format!("Operation on {} and {} not supported", lhs, rhs)),
        }
    }

    pub fn sin(self) -> Value {
        match &self {
            Value::Number(num) => Value::Number(num.sin()),
            Value::Vector(vals) => {
                Value::Vector(vals.iter().map(|x| Value::sin(x.clone())).collect())
            }
            Value::Error(_err) => self,
        }
    }

    pub fn cos(self) -> Value {
        match &self {
            Value::Number(num) => Value::Number(num.cos()),
            Value::Vector(vals) => {
                Value::Vector(vals.iter().map(|x| Value::cos(x.clone())).collect())
            }
            Value::Error(_err) => self,
        }
    }

    pub fn tan(self) -> Value {
        match &self {
            Value::Number(num) => Value::Number(num.tan()),
            Value::Vector(vals) => {
                Value::Vector(vals.iter().map(|x| Value::tan(x.clone())).collect())
            }
            Value::Error(_err) => self,
        }
    }

    pub fn asin(self) -> Value {
        match &self {
            Value::Number(num) => Value::Number(num.asin()),
            Value::Vector(vals) => {
                Value::Vector(vals.iter().map(|x| Value::asin(x.clone())).collect())
            }
            Value::Error(_err) => self,
        }
    }

    pub fn acos(self) -> Value {
        match &self {
            Value::Number(num) => Value::Number(num.acos()),
            Value::Vector(vals) => {
                Value::Vector(vals.iter().map(|x| Value::acos(x.clone())).collect())
            }
            Value::Error(_err) => self,
        }
    }

    pub fn atan(self) -> Value {
        match &self {
            Value::Number(num) => Value::Number(num.atan()),
            Value::Vector(vals) => {
                Value::Vector(vals.iter().map(|x| Value::atan(x.clone())).collect())
            }
            Value::Error(_err) => self,
        }
    }

    pub fn abs(self) -> Value {
        match &self {
            Value::Number(num) => Value::Number(Complex64::new(num.abs(), 0.0)),
            Value::Vector(vals) => {
                Value::Vector(vals.iter().map(|x| Value::abs(x.clone())).collect())
            }
            Value::Error(_err) => self,
        }
    }

    pub fn round(self) -> Value {
        match &self {
            Value::Number(num) => Value::Number(Complex64::new(num.re.round(), num.im.round())),
            Value::Vector(vals) => {
                Value::Vector(vals.iter().map(|x| Value::abs(x.clone())).collect())
            }
            Value::Error(_err) => self,
        }
    }
}
