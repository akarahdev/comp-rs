use crate::math::context::Context;
use crate::math::expr::{BinaryOperation, Expression, UnaryOperation};
use crate::math::values::Value;
use crate::math::values::Value::Number;
use num::complex::Complex64;

impl Expression {
    pub fn eval(&self, ctx: &mut Context) -> Value {
        match self {
            Expression::Unary {
                operation,
                expr,
                id,
            } => Value::unary_op(operation.clone(), &expr.eval(ctx)),
            Expression::Binary { op, lhs, rhs, id } => match op {
                BinaryOperation::Store => {
                    let right = rhs.eval(ctx);
                    if let Expression::Literal { content, id } = *lhs.clone() {
                        ctx.set_variable(content.clone(), right.clone());
                    }
                    right
                }
                _ => Value::bin_op(*op, &lhs.eval(ctx), &rhs.eval(ctx)),
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

    pub fn evaluate_summation(
        minimum: &Expression,
        maximum: &Expression,
        variable: &Expression,
        expression: &Expression,
        ctx: &mut Context,
    ) -> Value {
        let Expression::Literal {
            content: ref variable_name,
            id: variable_id,
        } = variable
        else {
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
