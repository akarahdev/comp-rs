use std::cell::RefCell;
use crate::gui::idx::new_id;
use crate::math::context::Context;
use crate::math::values::Value;
use crate::math::values::Value::Number;
use num::complex::Complex64;
use num::Complex;
use std::cmp::min;
use std::env::var;
use std::rc::{Rc, Weak};

#[derive(Clone, Debug, Hash)]
pub enum Expression {
    Unary {
        operation: UnaryOperation,
        expr: Box<Expression>,
        id: u64,
    },
    Binary {
        op: BinaryOperation,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
        id: u64,
    },
    Vector { exprs: Vec<Expression>, id: u64 },
    Literal { content: String, id: u64 },
    Parenthesis { expr: Box<Expression>, id: u64 },
    GraphExpression { expr: Box<Expression> },
    Summation {
        minimum: Box<Expression>,
        maximum: Box<Expression>,
        variable: Box<Expression>,
        expression: Box<Expression>,
    },
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum UnaryOperation {
    Negate,
    Sin,
    Cos,
    Tan,
    InverseSin,
    InverseCos,
    InverseTan,
}

impl ToString for UnaryOperation {
    fn to_string(&self) -> String {
        match self {
            UnaryOperation::Negate => "-",
            UnaryOperation::Sin => "sin",
            UnaryOperation::Cos => "cos",
            UnaryOperation::Tan => "tan",
            UnaryOperation::InverseSin => "sin^-1",
            UnaryOperation::InverseCos => "cos^-1",
            UnaryOperation::InverseTan => "tan^-1",
        }
            .to_string()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BinaryOperation {
    Add,
    Sub,
    Multiply,
    Divide,
    Power,
    Root,
    Store,
}

impl ToString for BinaryOperation {
    fn to_string(&self) -> String {
        match self {
            BinaryOperation::Add => "+",
            BinaryOperation::Sub => "-",
            BinaryOperation::Multiply => "*",
            BinaryOperation::Divide => "÷",
            &BinaryOperation::Power => "^",
            BinaryOperation::Root => "√",
            BinaryOperation::Store => "=",
        }
            .to_string()
    }
}

impl Expression {
    pub fn eval(&self, ctx: &mut Context) -> Value {
        match self {
            Expression::Unary { operation, expr, id } => match operation {
                UnaryOperation::Negate => Value::mul(&expr.eval(ctx), &Number((-1.0).into())),
                UnaryOperation::Sin => Value::sin(expr.eval(ctx)),
                UnaryOperation::Cos => Value::cos(expr.eval(ctx)),
                UnaryOperation::Tan => Value::tan(expr.eval(ctx)),
                UnaryOperation::InverseSin => Value::asin(expr.eval(ctx)),
                UnaryOperation::InverseCos => Value::acos(expr.eval(ctx)),
                UnaryOperation::InverseTan => Value::atan(expr.eval(ctx)),
            },
            Expression::Binary { op, lhs, rhs, id } => match op {
                BinaryOperation::Add => Value::add(&lhs.eval(ctx), &rhs.eval(ctx)),
                BinaryOperation::Sub => Value::sub(&lhs.eval(ctx), &rhs.eval(ctx)),
                BinaryOperation::Multiply => Value::mul(&lhs.eval(ctx), &rhs.eval(ctx)),
                BinaryOperation::Divide => Value::div(&lhs.eval(ctx), &rhs.eval(ctx)),
                BinaryOperation::Power => Value::pow(&lhs.eval(ctx), &rhs.eval(ctx)),
                BinaryOperation::Root => Value::root(&lhs.eval(ctx), &rhs.eval(ctx)),
                BinaryOperation::Store => {
                    let right = rhs.eval(ctx);
                    if let Expression::Literal { content, id } = *lhs.clone() {
                        ctx.set_variable(content.clone(), right.clone());
                    }
                    right
                }
            },
            Expression::Literal { content, id } => {
                if let Ok(result) = content.parse::<f64>() {
                    return Number(result.into());
                }
                if let Some(result) = ctx.resolve_variable(&content) {
                    return result.clone();
                }
                Value::Error(format!("unable to resolve value `{}`", content))
            }
            Expression::Parenthesis { expr, id } => expr.eval(ctx),
            Expression::Vector { exprs, id } => {
                Value::Vector(exprs.iter().map(|x| x.eval(ctx)).collect())
            }
            Expression::GraphExpression { expr } => expr.eval(ctx),
            Expression::Summation {
                minimum,
                maximum,
                variable,
                expression,
            } => Self::evaluate_summation(minimum, maximum, variable, expression, ctx),
        }
    }

    pub fn build_binop(&mut self, op: BinaryOperation, pat: &str) {
        let Expression::Literal { content, id } = self else {
            panic!("can not build node out of a non-literal")
        };

        *self = Expression::Binary {
            op,
            lhs: Box::new(Expression::Literal { content: content.replace(pat, ""), id: new_id() }),
            rhs: Box::new(Expression::Literal { content: "".to_string(), id: new_id() }),
            id: new_id(),
        }
    }

    pub fn build_unop(&mut self, op: UnaryOperation) {
        *self = Expression::Unary {
            operation: op,
            expr: Box::new(Expression::Literal { content: "0".to_string(), id: new_id() }),
            id: new_id(),
        };
    }

    pub fn evaluate_summation(
        minimum: &Expression,
        maximum: &Expression,
        variable: &Expression,
        expression: &Expression,
        ctx: &mut Context,
    ) -> Value {
        let Expression::Literal { content: ref variable_name, id: variable_id } = variable else {
            return Value::Error("variables must be a literal".to_string());
        };
        let min_val = minimum.eval(ctx).round().clone();
        let max_val = maximum.eval(ctx).round().clone();
        let Value::Number(min_val) = min_val else {
            return Value::Error("minimum of summation must be a number".to_string());
        };
        let Value::Number(max_val) = max_val else {
            return Value::Error("maximum of summation must be a number".to_string());
        };
        if min_val.im != 0.0 {
            return Value::Error("summation minimum can not be complex".to_string());
        };
        if max_val.im != 0.0 {
            return Value::Error("summation maximum can not be complex".to_string());
        };
        if min_val.re >= max_val.re {
            return Value::Error("summation maximum can not be larger than minimum".to_string());
        };
        let old_value = ctx.resolve_variable(variable_name).cloned();
        let mut base = Value::Number(Complex64::new(0.0, 0.0));
        for intermediate_value in (min_val.re as i64)..=(max_val.re as i64) {
            ctx.set_variable(
                variable_name.clone(),
                Value::Number(Complex64::new(intermediate_value as f64, 0.0)),
            );
            let result = expression.eval(ctx);
            base = Value::add(&base, &result);
        }
        if let Some(old_value) = old_value {
            ctx.set_variable(variable_name.clone(), old_value.clone());
        }
        base
    }
}
