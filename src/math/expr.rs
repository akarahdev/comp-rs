use crate::gui::idx::new_id;
use crate::math::context::Context;
use crate::math::values::Value;
use crate::math::values::Value::Number;
use num::complex::Complex64;
use num::Complex;
use std::cell::RefCell;
use std::cmp::min;
use std::env::var;
use std::rc::{Rc, Weak};

#[derive(Clone, Debug)]
pub struct ExprRef(Rc<RefCell<Expression>>);
pub struct WeakExprRef(Weak<RefCell<Expression>>);

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
    Vector {
        exprs: Vec<Expression>,
        id: u64,
    },
    Literal {
        content: String,
        id: u64,
    },
    Parenthesis {
        expr: Box<Expression>,
        id: u64,
    },
    GraphExpression {
        expr: Box<Expression>,
    },
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
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
