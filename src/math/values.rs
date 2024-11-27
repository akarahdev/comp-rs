use crate::math::expr::{BinaryOperation, Expression, UnaryOperation};
use num::complex::{Complex64, ComplexFloat};
use num::traits::real::Real;
use std::fmt::{Display, Formatter, Write};
use crate::math::context::Context;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Value {
    Number(Complex64),
    Vector(Vec<Value>),
    Lambda(String, Expression),
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
            Value::Lambda(var, expr) => {
                f.write_str("fn(")?;
                f.write_str(var)?;
                f.write_str("): <...>")?;
                Ok(())
            }
        }
    }
}

impl Value {
    pub fn bin_op(op: BinaryOperation, lhs: &Value, rhs: &Value, ctx: &mut Context) -> Value {
        match op {
            BinaryOperation::Add => Value::add(lhs, rhs),
            BinaryOperation::Sub => Value::sub(lhs, rhs),
            BinaryOperation::Multiply => Value::mul(lhs, rhs),
            BinaryOperation::Divide => Value::div(lhs, rhs),
            BinaryOperation::Power => Value::pow(lhs, rhs),
            BinaryOperation::Root => Value::root(lhs, rhs),
            BinaryOperation::Store => rhs.clone(),
            BinaryOperation::Invoke => {
                let Value::Lambda(lambda_var, lambda_expr) = lhs else {
                    return Value::Error("left-side must be a function".to_string());
                };
                ctx.push_frame();
                ctx.set_variable(lambda_var.clone(), rhs.clone());
                let result = lambda_expr.eval(ctx);
                ctx.pop_frame();
                result
            }
        }
    }

    pub fn unary_op(op: UnaryOperation, value: &Value) -> Value {
        match op {
            UnaryOperation::Negate => Value::mul(value, &Value::Number(Complex64::new(-1.0, -1.0))),
            UnaryOperation::Sin => value.sin(),
            UnaryOperation::Cos => value.cos(),
            UnaryOperation::Tan => value.tan(),
            UnaryOperation::InverseSin => value.asin(),
            UnaryOperation::InverseCos => value.acos(),
            UnaryOperation::InverseTan => value.atan(),
            UnaryOperation::HyperbolicSin => value.sinh(),
            UnaryOperation::HyperbolicCos => value.cosh(),
            UnaryOperation::HyperbolicTan => value.tanh(),
            UnaryOperation::InverseHyperbolicSin => value.asinh(),
            UnaryOperation::InverseHyperbolicCos => value.acosh(),
            UnaryOperation::InverseHyperbolicTan => value.atanh(),
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

    pub fn sin(&self) -> Value {
        match &self {
            Value::Number(num) => Value::Number(num.sin()),
            Value::Vector(vals) => Value::Vector(vals.iter().map(|x| Value::sin(x)).collect()),
            Value::Lambda(var, expr) => Value::Error("WIP".to_string()),
            Value::Error(_err) => self.clone(),
        }
    }

    pub fn cos(&self) -> Value {
        match &self {
            Value::Number(num) => Value::Number(num.cos()),
            Value::Vector(vals) => Value::Vector(vals.iter().map(|x| Value::cos(x)).collect()),
            Value::Lambda(var, expr) => Value::Error("WIP".to_string()),
            Value::Error(_err) => self.clone(),
        }
    }

    pub fn tan(&self) -> Value {
        match &self {
            Value::Number(num) => Value::Number(num.tan()),
            Value::Vector(vals) => Value::Vector(vals.iter().map(|x| Value::tan(x)).collect()),
            Value::Lambda(var, expr) => Value::Error("WIP".to_string()),
            Value::Error(_err) => self.clone(),
        }
    }

    pub fn asin(&self) -> Value {
        match &self {
            Value::Number(num) => Value::Number(num.asin()),
            Value::Vector(vals) => Value::Vector(vals.iter().map(|x| Value::asin(x)).collect()),
            Value::Lambda(var, expr) => Value::Error("WIP".to_string()),
            Value::Error(_err) => self.clone(),
        }
    }

    pub fn acos(&self) -> Value {
        match &self {
            Value::Number(num) => Value::Number(num.acos()),
            Value::Vector(vals) => Value::Vector(vals.iter().map(|x| Value::acos(x)).collect()),
            Value::Lambda(var, expr) => Value::Error("WIP".to_string()),
            Value::Error(_err) => self.clone(),
        }
    }

    pub fn atan(&self) -> Value {
        match &self {
            Value::Number(num) => Value::Number(num.atan()),
            Value::Vector(vals) => Value::Vector(vals.iter().map(|x| Value::atan(x)).collect()),
            Value::Lambda(var, expr) => Value::Error("WIP".to_string()),
            Value::Error(_err) => self.clone(),
        }
    }

    pub fn sinh(&self) -> Value {
        match &self {
            Value::Number(num) => Value::Number(num.sinh()),
            Value::Vector(vals) => Value::Vector(vals.iter().map(|x| Value::sin(x)).collect()),
            Value::Lambda(var, expr) => Value::Error("WIP".to_string()),
            Value::Error(_err) => self.clone(),
        }
    }

    pub fn cosh(&self) -> Value {
        match &self {
            Value::Number(num) => Value::Number(num.cosh()),
            Value::Vector(vals) => Value::Vector(vals.iter().map(|x| Value::cosh(x)).collect()),
            Value::Lambda(var, expr) => Value::Error("WIP".to_string()),
            Value::Error(_err) => self.clone(),
        }
    }

    pub fn tanh(&self) -> Value {
        match &self {
            Value::Number(num) => Value::Number(num.tanh()),
            Value::Vector(vals) => Value::Vector(vals.iter().map(|x| Value::tanh(x)).collect()),
            Value::Lambda(var, expr) => Value::Error("WIP".to_string()),
            Value::Error(_err) => self.clone(),
        }
    }

    pub fn asinh(&self) -> Value {
        match &self {
            Value::Number(num) => Value::Number(num.asinh()),
            Value::Vector(vals) => Value::Vector(vals.iter().map(|x| Value::asinh(x)).collect()),
            Value::Lambda(var, expr) => Value::Error("WIP".to_string()),
            Value::Error(_err) => self.clone(),
        }
    }

    pub fn acosh(&self) -> Value {
        match &self {
            Value::Number(num) => Value::Number(num.acosh()),
            Value::Vector(vals) => Value::Vector(vals.iter().map(|x| Value::acosh(x)).collect()),
            Value::Lambda(var, expr) => Value::Error("WIP".to_string()),
            Value::Error(_err) => self.clone(),
        }
    }

    pub fn atanh(&self) -> Value {
        match &self {
            Value::Number(num) => Value::Number(num.atanh()),
            Value::Vector(vals) => Value::Vector(vals.iter().map(|x| Value::atanh(x)).collect()),
            Value::Lambda(var, expr) => Value::Error("WIP".to_string()),
            Value::Error(_err) => self.clone(),
        }
    }

    pub fn abs(&self) -> Value {
        match &self {
            Value::Number(num) => Value::Number(Complex64::new(num.abs(), 0.0)),
            Value::Vector(vals) => Value::Vector(vals.iter().map(|x| Value::abs(x)).collect()),
            Value::Lambda(var, expr) => Value::Error("WIP".to_string()),
            Value::Error(_err) => self.clone(),
        }
    }

    pub fn round(&self) -> Value {
        match &self {
            Value::Number(num) => Value::Number(Complex64::new(num.re.round(), num.im.round())),
            Value::Vector(vals) => Value::Vector(vals.iter().map(|x| Value::round(x)).collect()),
            Value::Lambda(var, expr) => Value::Error("WIP".to_string()),
            Value::Error(_err) => self.clone(),
        }
    }
}
