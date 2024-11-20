use crate::math::values::Value;
use crate::math::values::Value::Number;
use eframe::egui::{
    Color32, ComboBox, Frame, Response, Shadow, Stroke, TextEdit, Ui, Vec2, Widget,
};
use std::time::Instant;
use crate::gui::idx::new_id;

#[derive(Clone, Debug)]
pub enum Expression {
    Unary(UnaryOperation, Box<Expression>, u64),
    Binary(BinaryOperation, Box<Expression>, Box<Expression>, u64),
    Literal(String, u64),
    Parenthesis(Box<Expression>, u64)
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOperation {
    Negate
}

impl ToString for UnaryOperation {
    fn to_string(&self) -> String {
        match self {
            UnaryOperation::Negate => "-",
        }.to_string()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinaryOperation {
    Add,
    Sub,
    Multiply,
    Divide,
    Power,
    Root
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
        }
        .to_string()
    }
}

impl Expression {
    pub fn eval(&self) -> Value {
        match self {
            Expression::Unary(op, value, id) => match op {
                UnaryOperation::Negate => Value::mul(value.eval(), Number(-1.0)),
            },
            Expression::Binary(op, lhs, rhs, id) => match op {
                BinaryOperation::Add => Value::add(lhs.eval(), rhs.eval()),
                BinaryOperation::Sub => Value::sub(lhs.eval(), rhs.eval()),
                BinaryOperation::Multiply => Value::mul(lhs.eval(), rhs.eval()),
                BinaryOperation::Divide => Value::div(lhs.eval(), rhs.eval()),
                BinaryOperation::Power => Value::pow(lhs.eval(), rhs.eval()),
                BinaryOperation::Root => Value::root(lhs.eval(), rhs.eval()),
            },
            Expression::Literal(value, id) => {
                if let Ok(result) = value.parse::<f64>() {
                    return Number(result)
                };
                Value::Error(format!("unable to resolve value `{}`", value))
            },
            Expression::Parenthesis(value, id) => value.eval()
        }
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
                    *self = Expression::Binary(
                        BinaryOperation::Add,
                        Box::new(Expression::Literal(content.replace("+", ""), new_id())),
                        Box::new(Expression::Literal("".to_string(), new_id())),
                        new_id()
                    )
                } else if content.ends_with("root") {
                    *self = Expression::Binary(
                        BinaryOperation::Root,
                        Box::new(Expression::Literal(content.replace("root", ""), new_id())),
                        Box::new(Expression::Literal("".to_string(), new_id())),
                        new_id()
                    )
                } else if content.starts_with("(") {
                    *self = Expression::Parenthesis(
                        Box::new(self.clone()),
                        new_id()
                    )
                } else if content.ends_with("-") {
                    if content.starts_with("-") && content.ends_with("-") {
                        *self = Expression::Unary(
                            UnaryOperation::Negate,
                            Box::new(Expression::Literal("0".to_string(), new_id())),
                            new_id()
                        )
                    } else {
                        *self = Expression::Binary(
                            BinaryOperation::Sub,
                            Box::new(Expression::Literal(content.replace("-", ""), new_id())),
                            Box::new(Expression::Literal("".to_string(), new_id())),
                            new_id()
                        )
                    }
                } else if content.ends_with("*") {
                    *self = Expression::Binary(
                        BinaryOperation::Multiply,
                        Box::new(Expression::Literal(content.replace("*", ""), new_id())),
                        Box::new(Expression::Literal("".to_string(), new_id())),
                        new_id()
                    )
                } else if content.ends_with("/") {
                    *self = Expression::Binary(
                        BinaryOperation::Divide,
                        Box::new(Expression::Literal(content.replace("/", ""), new_id())),
                        Box::new(Expression::Literal("".to_string(), new_id())),
                        new_id()
                    )
                }
            }
            _ => {}
        }
    }
}
