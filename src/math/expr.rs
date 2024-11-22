use crate::gui::idx::new_id;
use crate::math::context::Context;
use crate::math::values::Value;
use crate::math::values::Value::Number;
use eframe::egui::{
    Color32, ComboBox, Frame, Response, Shadow, Stroke, TextEdit, Ui, Vec2, Widget,
};
use std::time::Instant;

#[derive(Clone, Debug)]
pub enum Expression {
    Unary(UnaryOperation, Box<Expression>, u64),
    Binary(BinaryOperation, Box<Expression>, Box<Expression>, u64),
    Vector(Vec<Expression>, u64),
    Literal(String, u64),
    Parenthesis(Box<Expression>, u64),

    GraphExpression(Box<Expression>)
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOperation {
    Negate,
    Sin,
    Cos,
    Tan,
    InverseSin,
    InverseCos,
    InverseTan
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
            Expression::Unary(op, value, id) => match op {
                UnaryOperation::Negate => Value::mul(&value.eval(ctx), &Number((-1.0).into())),
                UnaryOperation::Sin => Value::sin(value.eval(ctx)),
                UnaryOperation::Cos => Value::cos(value.eval(ctx)),
                UnaryOperation::Tan => Value::tan(value.eval(ctx)),
                UnaryOperation::InverseSin => Value::asin(value.eval(ctx)),
                UnaryOperation::InverseCos => Value::acos(value.eval(ctx)),
                UnaryOperation::InverseTan => Value::atan(value.eval(ctx))
            },
            Expression::Binary(op, lhs, rhs, id) => match op {
                BinaryOperation::Add => Value::add(&lhs.eval(ctx), &rhs.eval(ctx)),
                BinaryOperation::Sub => Value::sub(&lhs.eval(ctx), &rhs.eval(ctx)),
                BinaryOperation::Multiply => Value::mul(&lhs.eval(ctx), &rhs.eval(ctx)),
                BinaryOperation::Divide => Value::div(&lhs.eval(ctx), &rhs.eval(ctx)),
                BinaryOperation::Power => Value::pow(&lhs.eval(ctx), &rhs.eval(ctx)),
                BinaryOperation::Root => Value::root(&lhs.eval(ctx), &rhs.eval(ctx)),
                BinaryOperation::Store => {
                    let right = rhs.eval(ctx);
                    if let Expression::Literal(name, id) = *lhs.clone() {
                        ctx.set_variable(name.clone(), right.clone());
                    }
                    right
                }
            },
            Expression::Literal(value, id) => {
                if let Ok(result) = value.parse::<f64>() {
                    return Number(result.into());
                }
                if let Some(result) = ctx.resolve_variable(&value) {
                    return result.clone();
                }
                Value::Error(format!("unable to resolve value `{}`", value))
            }
            Expression::Parenthesis(value, id) => value.eval(ctx),
            Expression::Vector(vec, id) =>
                Value::Vector(vec.iter().map(|x| x.eval(ctx)).collect()),
            Expression::GraphExpression(inner) => inner.eval(ctx),
        }
    }

    pub fn build_nodes(&mut self, op: BinaryOperation, pat: &str) {
        let Expression::Literal(content, id) = self else {
            panic!("can not build node out of a non-literal")
        };

        *self = Expression::Binary(
            op,
            Box::new(Expression::Literal(content.replace(pat, ""), new_id())),
            Box::new(Expression::Literal("".to_string(), new_id())),
            new_id(),
        )
    }

    pub fn update(&mut self) {
        match self {
            Expression::Binary(_, lhs, rhs, id) => {
                lhs.update();
                rhs.update();

                if let Expression::Literal(lhsc, id) = *lhs.clone() {
                    if let Expression::Literal(rhsc, id) = *rhs.clone() {
                        if lhsc.is_empty() && rhsc.is_empty() {
                            *self = Expression::Literal("".to_string(), new_id());
                        }
                    }
                }
            }
            Expression::Unary(_, val, id) => {
                val.update();

                if let Expression::Literal(vsc, id) = *val.clone() {
                    if vsc.is_empty() {
                        *self = Expression::Literal("".to_string(), new_id());
                    }
                }
            }
            Expression::Literal(content, id) => {
                if content.ends_with("+") {
                    self.build_nodes(BinaryOperation::Add, "+");
                } else if content.ends_with("-") {
                    if content.starts_with("-") && content.ends_with("-") {
                        *self = Expression::Unary(
                            UnaryOperation::Negate,
                            Box::new(Expression::Literal("0".to_string(), new_id())),
                            new_id(),
                        )
                    } else {
                        self.build_nodes(BinaryOperation::Sub, "-");
                    }
                } else if content.ends_with("*") {
                    self.build_nodes(BinaryOperation::Multiply, "*");
                } else if content.ends_with("/") {
                    self.build_nodes(BinaryOperation::Divide, "/");
                } else if content.ends_with("^") {
                    self.build_nodes(BinaryOperation::Power, "^");
                } else if content.ends_with("=") {
                    self.build_nodes(BinaryOperation::Store, "=");
                } else if content.starts_with("(") {
                    *self = Expression::Parenthesis(Box::new(self.clone()), new_id())
                } else if content.starts_with("[") {
                    *self = Expression::Vector(
                        Vec::new(),
                        new_id()
                    );
                } else if content.ends_with("root") {
                    self.build_nodes(BinaryOperation::Root, "root");
                } else if content.ends_with("graph") {
                    *self = Expression::GraphExpression(Box::new(Expression::Literal("".to_string(), new_id())));
                } else if content.ends_with("sin") {
                    *self = Expression::Unary(UnaryOperation::Sin, Box::new(Expression::Literal("0".to_string(), new_id())), new_id());
                } else if content.ends_with("cos") {
                    *self = Expression::Unary(UnaryOperation::Cos, Box::new(Expression::Literal("0".to_string(), new_id())), new_id());
                } else if content.ends_with("tan") {
                    *self = Expression::Unary(UnaryOperation::Tan, Box::new(Expression::Literal("0".to_string(), new_id())), new_id());
                } else if content.ends_with("asin") {
                    *self = Expression::Unary(UnaryOperation::InverseSin, Box::new(Expression::Literal("0".to_string(), new_id())), new_id());
                } else if content.ends_with("acos") {
                    *self = Expression::Unary(UnaryOperation::InverseCos, Box::new(Expression::Literal("0".to_string(), new_id())), new_id());
                } else if content.ends_with("atan") {
                    *self = Expression::Unary(UnaryOperation::InverseTan, Box::new(Expression::Literal("0".to_string(), new_id())), new_id());
                }
            }
            Expression::Vector(exprs, id) => {
                for expr in exprs {
                    expr.update();
                }
            }
            Expression::Parenthesis(expr, id) => expr.update(),
            Expression::GraphExpression(expr) => expr.update(),
        }
    }
}
