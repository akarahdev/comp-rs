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
        *self = Expression::Unary(op, Box::new(Expression::Literal("".to_string(), new_id())), new_id());
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
        }
    }
}
