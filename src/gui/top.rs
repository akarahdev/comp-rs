use crate::math::expr::Expression;
use crate::math::values::Value;

pub struct TopLevelExpression {
    pub expression: Expression,
    pub expression_hash: u64,
    
    pub answer_cached: Option<Value>
}