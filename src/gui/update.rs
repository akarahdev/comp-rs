use crate::gui::idx::new_id;
use crate::math::expr::{new_expr_ref, BinaryOperation, Expression, UnaryOperation};

impl Expression {
    pub fn update(&mut self) {
        match self {
            Expression::Binary { op, lhs, rhs, id: _id } => {
                lhs.lock().update();
                rhs.lock().update();

                if let Expression::Literal { content: lhs, id, .. } = lhs.clone().lock().clone() {
                    if let Expression::Literal { content: rhs, id, .. } = rhs.clone().lock().clone() {
                        if lhs.is_empty() && rhs.is_empty() {
                            *self = Expression::Literal { content: "".to_string(), id: new_id() };
                        }
                    }
                }
            }
            Expression::Unary { operation, expr, id } => {
                expr.lock().update();

                if let Expression::Literal { content, id, .. } = expr.clone().lock().clone() {
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
                            expr: new_expr_ref(Expression::Literal { content: "0".to_string(), id: new_id() }),
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
                    *self = Expression::Parenthesis { expr: new_expr_ref(self.clone()), id: new_id() }
                }
                _ if content.starts_with("[") => *self = Expression::Vector { exprs: Vec::new(), id: new_id() },
                _ if content.starts_with("graph") => {
                    *self = Expression::GraphExpression {
                        expr: new_expr_ref(Expression::Literal {
                            content: "".to_string(),
                            id: new_id(),
                        })
                    }
                }
                _ if content.ends_with("sum") => {
                    *self = Expression::Summation {
                        minimum: new_expr_ref(Expression::Literal { content: "0".to_string(), id: new_id() }),
                        maximum: new_expr_ref(Expression::Literal { content: "0".to_string(), id: new_id() }),
                        variable: new_expr_ref(Expression::Literal { content: "0".to_string(), id: new_id() }),
                        expression: new_expr_ref(Expression::Literal { content: "0".to_string(), id: new_id() }),
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
                    expr.lock().update();
                }
            }
            Expression::Parenthesis { expr, id } => expr.lock().update(),
            Expression::GraphExpression { expr } => expr.lock().update(),
            Expression::Summation {
                minimum,
                maximum,
                variable,
                expression,
            } => {
                minimum.lock().update();
                maximum.lock().update();
                variable.lock().update();
                expression.lock().update();

                if let Expression::Literal { content: minimum_text, id: minimum_id } = minimum.clone().lock().clone() {
                    if let Expression::Literal { content: maximum_text, id: maximum_id } = maximum.clone().lock().clone() {
                        if let Expression::Literal { content: variable_text, id: variable_id } = variable.clone().lock().clone() {
                            if let Expression::Literal { content: expression_text, id: expression_id } = expression.clone().lock().clone() {
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
