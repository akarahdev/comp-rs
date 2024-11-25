use crate::gui::idx::new_id;
use crate::math::expr::{BinaryOperation, Expression, UnaryOperation};
use std::cmp::{min, Ordering, PartialOrd};

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
enum Precedence {
    Value = 0,
    Parenthesis,
    Storage,
    Unary,
    Exponent,
    Factor,
    Term,
    Comparison,
    And,
    Or,
    Summation,
}

impl Expression {
    pub fn precedence(&self) -> Precedence {
        match self {
            Expression::Unary { .. } => Precedence::Unary,
            Expression::Binary { op, .. } => match op {
                BinaryOperation::Add => Precedence::Term,
                BinaryOperation::Sub => Precedence::Term,
                BinaryOperation::Multiply => Precedence::Factor,
                BinaryOperation::Divide => Precedence::Factor,
                BinaryOperation::Power => Precedence::Exponent,
                BinaryOperation::Root => Precedence::Exponent,
                BinaryOperation::Store => Precedence::Storage,
            },
            Expression::Vector { .. } => Precedence::Value,
            Expression::Literal { .. } => Precedence::Value,
            Expression::Parenthesis { .. } => Precedence::Parenthesis,
            Expression::GraphExpression { .. } => Precedence::Parenthesis,
            Expression::Summation { .. } => Precedence::Value,
        }
    }

    /// Enforces order of operations.
    /// Expressions such as:
    /// [self.lhs * (b.lhs + c.rhs)]
    /// get rewritten as
    /// [(self.lhs * b.lhs) + c.rhs]
    /// depending on their precedence.
    pub fn enforce_ooo(&mut self) {
        let self_precedence = self.precedence();
        match self {
            Expression::Binary { op, lhs, rhs, .. } => {
                let rhs_precedence = rhs.clone().precedence();

                lhs.enforce_ooo();
                rhs.enforce_ooo();

                let Expression::Binary {
                    lhs: ref target_value,
                    rhs: ref moving_value,
                    op: rhs_op,
                    ..
                } = **rhs
                else {
                    return;
                };

                if rhs_precedence < self_precedence {
                    return;
                }

                *self = Expression::Binary {
                    op: rhs_op.clone(),
                    lhs: Box::new(Expression::Binary {
                        op: op.clone(),
                        lhs: lhs.clone(),
                        rhs: target_value.clone(),
                        id: new_id(),
                    }),
                    rhs: moving_value.clone(),
                    id: new_id(),
                };
            }
            Expression::Unary { expr, .. } => expr.enforce_ooo(),
            Expression::GraphExpression { expr } => expr.enforce_ooo(),
            Expression::Summation {
                expression,
                maximum,
                minimum,
                ..
            } => {
                expression.enforce_ooo();
                maximum.enforce_ooo();
                minimum.enforce_ooo();
            }

            Expression::Parenthesis { expr, .. } => expr.enforce_ooo(),
            Expression::Literal { .. } => {}
            _ => {}
        }
    }
}
