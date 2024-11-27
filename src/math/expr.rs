use std::cell::RefCell;
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
        unbox_to_binop: bool
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
    Lambda {
        variable: Box<Expression>,
        expr: Box<Expression>,
    }
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
    
    HyperbolicSin,
    HyperbolicCos,
    HyperbolicTan,
    InverseHyperbolicSin,
    InverseHyperbolicCos,
    InverseHyperbolicTan,
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
            UnaryOperation::HyperbolicSin => "sinh",
            UnaryOperation::HyperbolicCos => "cosh",
            UnaryOperation::HyperbolicTan => "tanh",
            UnaryOperation::InverseHyperbolicSin => "sinh^-1",
            UnaryOperation::InverseHyperbolicCos => "cosh^-1",
            UnaryOperation::InverseHyperbolicTan => "tanh^-1"
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
    Invoke,

    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Equal
}

impl ToString for BinaryOperation {
    fn to_string(&self) -> String {
        match self {
            BinaryOperation::Add => "+",
            BinaryOperation::Sub => "-",
            BinaryOperation::Multiply => "*",
            BinaryOperation::Divide => "÷",
            BinaryOperation::Power => "^",
            BinaryOperation::Root => "√",
            BinaryOperation::Store => "=",
            BinaryOperation::Invoke => "(",
            BinaryOperation::GreaterThan => ">",
            BinaryOperation::LessThan => "<",
            BinaryOperation::GreaterThanOrEqual => ">=",
            BinaryOperation::LessThanOrEqual => "<=",
            BinaryOperation::Equal => "==",
        }
        .to_string()
    }
}
