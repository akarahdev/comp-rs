use crate::gui::idx::new_id;
use crate::math::expr::{BinaryOperation, Expression, UnaryOperation};

impl Expression {
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
            Expression::Literal(content, _id) => match content {
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
                _ if content.starts_with("(") => {
                    *self = Expression::Parenthesis(Box::new(self.clone()), new_id())
                }
                _ if content.starts_with("[") => *self = Expression::Vector(Vec::new(), new_id()),
                _ if content.starts_with("graph") => {
                    *self = Expression::GraphExpression(Box::new(Expression::Literal(
                        "".to_string(),
                        new_id(),
                    )))
                }
                _ if content.ends_with("sum") => {
                    *self = Expression::Summation {
                        minimum: Box::new(Expression::Literal("0".to_string(), new_id())),
                        maximum: Box::new(Expression::Literal("0".to_string(), new_id())),
                        variable: Box::new(Expression::Literal("0".to_string(), new_id())),
                        expression: Box::new(Expression::Literal("0".to_string(), new_id())),
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
            Expression::Vector(exprs, _id) => {
                for expr in exprs {
                    expr.update();
                }
            }
            Expression::Parenthesis(expr, _id) => expr.update(),
            Expression::GraphExpression(expr) => expr.update(),
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

                if let Expression::Literal(minimum_text, minimum_id) = &**minimum {
                    if let Expression::Literal(maximum_text, maximum_id) = &**maximum {
                        if let Expression::Literal(variable_text, variable_id) = &**variable {
                            if let Expression::Literal(expression_text, expression_id) =
                                &**expression
                            {
                                if minimum_text.is_empty()
                                    && maximum_text.is_empty()
                                    && variable_text.is_empty()
                                    && expression_text.is_empty()
                                {
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
