use std::cmp::min;
use num::Complex;
use num::complex::Complex64;
use crate::gui::idx::new_id;
use crate::math::context::Context;
use crate::math::values::Value;
use crate::math::values::Value::Number;

#[derive(Clone, Debug)]
pub enum Expression {
    Unary(UnaryOperation, Box<Expression>, u64),
    Binary(BinaryOperation, Box<Expression>, Box<Expression>, u64),
    Vector(Vec<Expression>, u64),
    Literal(String, u64),
    Parenthesis(Box<Expression>, u64),
    GraphExpression(Box<Expression>),

    Summation {
        minimum: Box<Expression>,
        maximum: Box<Expression>,
        variable: Box<Expression>,
        expression: Box<Expression>
    },
}

#[derive(Clone, Debug, PartialEq)]
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
            UnaryOperation::InverseTan => "tan^-1"
        }
            .to_string()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
            Expression::Unary(op, value, _id) => match op {
                UnaryOperation::Negate => Value::mul(&value.eval(ctx), &Number((-1.0).into())),
                UnaryOperation::Sin => Value::sin(value.eval(ctx)),
                UnaryOperation::Cos => Value::cos(value.eval(ctx)),
                UnaryOperation::Tan => Value::tan(value.eval(ctx)),
                UnaryOperation::InverseSin => Value::asin(value.eval(ctx)),
                UnaryOperation::InverseCos => Value::acos(value.eval(ctx)),
                UnaryOperation::InverseTan => Value::atan(value.eval(ctx))
            },
            Expression::Binary(op, lhs, rhs, _id) => match op {
                BinaryOperation::Add => Value::add(&lhs.eval(ctx), &rhs.eval(ctx)),
                BinaryOperation::Sub => Value::sub(&lhs.eval(ctx), &rhs.eval(ctx)),
                BinaryOperation::Multiply => Value::mul(&lhs.eval(ctx), &rhs.eval(ctx)),
                BinaryOperation::Divide => Value::div(&lhs.eval(ctx), &rhs.eval(ctx)),
                BinaryOperation::Power => Value::pow(&lhs.eval(ctx), &rhs.eval(ctx)),
                BinaryOperation::Root => Value::root(&lhs.eval(ctx), &rhs.eval(ctx)),
                BinaryOperation::Store => {
                    let right = rhs.eval(ctx);
                    if let Expression::Literal(name, _id) = *lhs.clone() {
                        ctx.set_variable(name.clone(), right.clone());
                    }
                    right
                }
            },
            Expression::Literal(value, _id) => {
                if let Ok(result) = value.parse::<f64>() {
                    return Number(result.into());
                }
                if let Some(result) = ctx.resolve_variable(&value) {
                    return result.clone();
                }
                Value::Error(format!("unable to resolve value `{}`", value))
            }
            Expression::Parenthesis(value, _id) => value.eval(ctx),
            Expression::Vector(vec, _id) =>
                Value::Vector(vec.iter().map(|x| x.eval(ctx)).collect()),
            Expression::GraphExpression(inner) => inner.eval(ctx),
            Expression::Summation { minimum, maximum, variable, expression } => {
                let Expression::Literal(ref variable_name, variable_id) = **variable else {
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
                let old_value = ctx.resolve_variable(&variable_name).cloned();
                let mut base = Value::Number(Complex64::new(0.0, 0.0));
                for intermediate_value in (min_val.re as i64)..=(max_val.re as i64) {
                    ctx.set_variable(variable_name.clone(), Value::Number(Complex64::new(intermediate_value as f64, 0.0)));
                    let result = expression.eval(ctx);
                    base = Value::add(&base, &result);
                }
                if let Some(old_value) = old_value {
                    ctx.set_variable(variable_name.clone(), old_value.clone());
                }
                base
            }
        }
    }

    pub fn build_binop(&mut self, op: BinaryOperation, pat: &str) {
        let Expression::Literal(content, _id) = self else {
            panic!("can not build node out of a non-literal")
        };

        *self = Expression::Binary(
            op,
            Box::new(Expression::Literal(content.replace(pat, ""), new_id())),
            Box::new(Expression::Literal("".to_string(), new_id())),
            new_id(),
        )
    }

    pub fn build_unop(&mut self, op: UnaryOperation) {
        *self = Expression::Unary(op, Box::new(Expression::Literal("0".to_string(), new_id())), new_id());
    }

    pub fn update(&mut self) {
        match self {
            Expression::Binary(_, lhs, rhs, _id) => {
                lhs.update();
                rhs.update();

                if let Expression::Literal(lhsc, _id) = *lhs.clone() {
                    if let Expression::Literal(rhsc, _id) = *rhs.clone() {
                        if lhsc.is_empty() && rhsc.is_empty() {
                            *self = Expression::Literal("".to_string(), new_id());
                        }
                    }
                }
            }
            Expression::Unary(_, val, _id) => {
                val.update();

                if let Expression::Literal(vsc, _id) = *val.clone() {
                    if vsc.is_empty() {
                        *self = Expression::Literal("".to_string(), new_id());
                    }
                }
            }
            Expression::Literal(content, _id) => {
                match content {
                    _ if content.ends_with("+") => self.build_binop(BinaryOperation::Add, "+"),
                    c if content.ends_with("-") => {
                        if c.starts_with("-") && c.ends_with("-") {
                            *self = Expression::Unary(
                                UnaryOperation::Negate,
                                Box::new(Expression::Literal("0".to_string(), new_id())),
                                new_id(),
                            )
                        } else {
                            self.build_binop(BinaryOperation::Sub, "-");
                        }
                    }
                    _ if content.ends_with("*") => self.build_binop(BinaryOperation::Multiply, "*"),
                    _ if content.ends_with("/") => self.build_binop(BinaryOperation::Divide, "/"),
                    _ if content.ends_with("^") => self.build_binop(BinaryOperation::Power, "^"),
                    _ if content.ends_with("=") => self.build_binop(BinaryOperation::Store, "="),
                    _ if content.ends_with("rt") => self.build_binop(BinaryOperation::Root, "rt"),
                    _ if content.ends_with("root") => self.build_binop(BinaryOperation::Root, "root"),
                    _ if content.starts_with("(") =>
                        *self = Expression::Parenthesis(Box::new(self.clone()), new_id()),
                    _ if content.starts_with("[") =>
                        *self = Expression::Vector(
                            Vec::new(),
                            new_id(),
                        ),
                    _ if content.starts_with("graph") =>
                        *self = Expression::GraphExpression(Box::new(Expression::Literal("".to_string(), new_id()))),
                    _ if content.ends_with("sum") =>
                        *self = Expression::Summation {
                            minimum: Box::new(Expression::Literal("0".to_string(), new_id())),
                            maximum: Box::new(Expression::Literal("0".to_string(), new_id())),
                            variable: Box::new(Expression::Literal("0".to_string(), new_id())),
                            expression: Box::new(Expression::Literal("0".to_string(), new_id())),
                        },
                    _ if content.starts_with("sin") => self.build_unop(UnaryOperation::Sin),
                    _ if content.starts_with("cos") => self.build_unop(UnaryOperation::Cos),
                    _ if content.starts_with("tan") => self.build_unop(UnaryOperation::Tan),
                    _ if content.starts_with("asin") => self.build_unop(UnaryOperation::InverseSin),
                    _ if content.starts_with("acos") => self.build_unop(UnaryOperation::InverseCos),
                    _ if content.starts_with("atan") => self.build_unop(UnaryOperation::InverseTan),
                    _ => {}
                }
            }
            Expression::Vector(exprs, _id) => {
                for expr in exprs {
                    expr.update();
                }
            }
            Expression::Parenthesis(expr, _id) => expr.update(),
            Expression::GraphExpression(expr) => expr.update(),
            Expression::Summation { minimum, maximum,
                variable, expression } => {
                minimum.update();
                maximum.update();
                variable.update();
                expression.update();

                if let Expression::Literal(minimum_text, minimum_id) = &**minimum {
                    if let Expression::Literal(maximum_text, maximum_id) = &**maximum {
                        if let Expression::Literal(variable_text, variable_id) = &**variable {
                            if let Expression::Literal(expression_text, expression_id) = &**expression {
                                if minimum_text.is_empty()
                                    && maximum_text.is_empty()
                                    && variable_text.is_empty()
                                    && expression_text.is_empty() {
                                    *self = Expression::Literal("".to_string(), new_id());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
