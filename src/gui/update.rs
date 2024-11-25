use crate::gui::idx::new_id;
use crate::math::expr::{BinaryOperation, Expression, UnaryOperation};

impl Expression {
    pub fn update(&mut self) {
        match self {
            Expression::Binary { op, lhs, rhs, id: _id } => {
                lhs.update();
                rhs.update();

                if let Expression::Literal { content: lhs, id, .. } = *lhs.clone() {
                    if let Expression::Literal { content: rhs, id, .. } = *rhs.clone() {
                        if lhs.is_empty() && rhs.is_empty() {
                            *self = Expression::Literal { content: "".to_string(), id: new_id() };
                        }
                    }
                }
            }
            Expression::Unary { operation, expr, id } => {
                expr.update();

                if let Expression::Literal { content, id, .. } = *expr.clone() {
                    if content.is_empty() {
                        *self = Expression::Literal { content: "".to_string(), id: new_id() };
                    }
                }
            }
            Expression::Literal { content, id } => match content {
                _ if content.ends_with("+") => self.build_binop(BinaryOperation::Add, "+"),
                c if content.ends_with("-") => {
                    if c.starts_with("-") && c.ends_with("-") {
                        *self = Expression::Unary {
                            operation: UnaryOperation::Negate,
                            expr: Box::new(Expression::Literal { content: "0".to_string(), id: new_id() }),
                            id: new_id(),
                        }
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
                _ if content.starts_with("(") => {
                    *self = Expression::Parenthesis { expr: Box::new(Expression::Literal {
                        content: "0".to_string(), id: new_id()
                    }), id: new_id() }
                }
                _ if content.starts_with("[") => *self = Expression::Vector { exprs: Vec::new(), id: new_id() },
                _ if content.starts_with("graph") => {
                    *self = Expression::GraphExpression {
                        expr: Box::new(Expression::Literal {
                            content: "".to_string(),
                            id: new_id(),
                        })
                    }
                }
                _ if content.ends_with("sum") => {
                    *self = Expression::Summation {
                        minimum: Box::new(Expression::Literal { content: "0".to_string(), id: new_id() }),
                        maximum: Box::new(Expression::Literal { content: "0".to_string(), id: new_id() }),
                        variable: Box::new(Expression::Literal { content: "0".to_string(), id: new_id() }),
                        expression: Box::new(Expression::Literal { content: "0".to_string(), id: new_id() }),
                    }
                }
                _ if content.starts_with("sin") => self.build_unop(UnaryOperation::Sin),
                _ if content.starts_with("cos") => self.build_unop(UnaryOperation::Cos),
                _ if content.starts_with("tan") => self.build_unop(UnaryOperation::Tan),
                _ if content.starts_with("asin") => self.build_unop(UnaryOperation::InverseSin),
                _ if content.starts_with("acos") => self.build_unop(UnaryOperation::InverseCos),
                _ if content.starts_with("atan") => self.build_unop(UnaryOperation::InverseTan),
                _ => {}
            },
            Expression::Vector { exprs, id } => {
                for expr in exprs {
                    expr.update();
                }
            }
            Expression::Parenthesis { expr, id } => expr.update(),
            Expression::GraphExpression { expr } => expr.update(),
            Expression::Summation {
                minimum,
                maximum,
                variable,
                expression,
            } => {
                minimum.update();
                maximum.update();
                variable.update();
                expression.update();

                if let Expression::Literal { content: minimum_text, id: minimum_id } = &**minimum {
                    if let Expression::Literal { content: maximum_text, id: maximum_id } = &**maximum {
                        if let Expression::Literal { content: variable_text, id: variable_id } = &**variable {
                            if let Expression::Literal{ content: expression_text, id: expression_id } =
                                &**expression
                            {
                                if minimum_text.is_empty()
                                    && maximum_text.is_empty()
                                    && variable_text.is_empty()
                                    && expression_text.is_empty()
                                {
                                    *self = Expression::Literal { content: "".to_string(), id: new_id() };
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
