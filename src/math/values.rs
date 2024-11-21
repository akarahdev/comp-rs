use std::fmt::{Display, Formatter, Write};

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Value {
    Number(f64),
    Complex(f64, f64),
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
            Value::Complex(real, imag) => {
                f.write_fmt(format_args!("{}", real))?;
                if *imag > 0.0 {
                    f.write_char('+')?;
                }
                f.write_fmt(format_args!("{}", imag))?;
                f.write_char('i')?;
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
            (Value::Complex(lnr, lni), Value::Number(rn)) => Value::Complex(lnr + rn, *lni),
            (Value::Number(ln), Value::Complex(rnr, rni)) => Value::Complex(rnr + ln, *rni),
            (Value::Complex(lnr, lni), Value::Complex(rnr, rni)) => Value::Complex(lnr + rnr, lni + rni),
            (Value::Vector(lhsv), Value::Vector(rhsv)) => {
                if lhsv.len() != rhsv.len() {
                    return Value::Error("vectors must have same length to be added".to_string());
                }
                Value::Vector(
                    lhsv.iter().zip(rhsv.iter())
                        .map(|(x, y)| Value::add(x, y))
                        .collect()
                )
            }
            (lhs, rhs) => Value::Error(format!(
                "Operation on {} and {} not supported",
                lhs, rhs
            )),
        }
    }

    pub fn sub(lhs: &Value, rhs: &Value) -> Value {
        match (lhs, rhs) {
            (Value::Number(ln), Value::Number(rn)) => Value::Number(ln - rn),
            (Value::Vector(lhsv), Value::Vector(rhsv)) => {
                if lhsv.len() != rhsv.len() {
                    return Value::Error("vectors must have same length to be subtracted".to_string());
                }
                Value::Vector(
                    lhsv.iter().zip(rhsv.iter())
                        .map(|(x, y)| Value::sub(x, y))
                        .collect()
                )
            }
            (lhs, rhs) => Value::Error(format!(
                "Operation on {} and {} not supported",
                lhs, rhs
            )),
        }
    }

    pub fn mul(lhs: &Value, rhs: &Value) -> Value {
        match (lhs, rhs) {
            (Value::Number(ln), Value::Number(rn)) => Value::Number(ln * rn),
            (Value::Vector(lhsv), Value::Vector(rhsv)) => {
                if lhsv.len() != rhsv.len() {
                    return Value::Error("vectors must have same length to be multiplied".to_string());
                }
                Value::Vector(
                    lhsv.iter().zip(rhsv.iter())
                        .map(|(x, y)| Value::mul(x, y))
                        .collect()
                )
            }
            (lhs, rhs) => Value::Error(format!(
                "Operation on {} and {} not supported",
                lhs, rhs
            )),
        }
    }

    pub fn div(lhs: &Value, rhs: &Value) -> Value {
        match (lhs, rhs) {
            (Value::Number(ln), Value::Number(rn)) => Value::Number(ln / rn),(Value::Vector(lhsv), Value::Vector(rhsv)) => {
                if lhsv.len() != rhsv.len() {
                    return Value::Error("vectors must have same length to be divided".to_string());
                }
                Value::Vector(
                    lhsv.iter().zip(rhsv.iter())
                        .map(|(x, y)| Value::div(x, y))
                        .collect()
                )
            }
            (lhs, rhs) => Value::Error(format!(
                "Operation on {} and {} not supported",
                lhs, rhs
            )),
        }
    }

    pub fn pow(lhs: &Value, rhs: &Value) -> Value {
        match (lhs, rhs) {
            (Value::Number(ln), Value::Number(rn)) => Value::Number(ln.powf(*rn)),
            (lhs, rhs) => Value::Error(format!(
                "Operation on {} and {} not supported",
                lhs, rhs
            )),
        }
    }

    pub fn root(lhs: &Value, rhs: &Value) -> Value {
        match (lhs, rhs) {
            (Value::Number(ln), Value::Number(rn)) => {
                if *rn < 0.0 {
                    Value::Complex(0.0, (rn * -1.0).powf(1.0 / ln))
                } else {
                    Value::Number(rn.powf(1.0 / ln))
                }
            },
            (lhs, rhs) => Value::Error(format!(
                "Operation on {} and {} not supported",
                lhs, rhs
            )),
        }
    }
}
