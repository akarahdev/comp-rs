use crate::gui::idx::new_id;
use crate::math::expr::{BinaryOperation, Expression, UnaryOperation};

impl Expression {
    pub fn build_binop(&mut self, op: BinaryOperation, pat: &str) {
        let Expression::Literal { content, id, new_literal } = self else {
            panic!("can not build node out of a non-literal")
        };

        *self = Expression::Binary {
            op,
            lhs: Box::new(Expression::Literal {
                content: content.replace(pat, ""),
                id: new_id(),
                new_literal: false,
            }),
            rhs: Box::new(Expression::Literal {
                content: "".to_string(),
                id: new_id(),
                new_literal: true,
            }),
            id: new_id(),
        }
    }

    pub fn build_unop(&mut self, op: UnaryOperation) {
        *self = Expression::Unary {
            operation: op,
            expr: Box::new(Expression::Literal {
                content: "?".to_string(),
                id: new_id(),
                new_literal: true,
            }),
            id: new_id(),
        };
    }
}
