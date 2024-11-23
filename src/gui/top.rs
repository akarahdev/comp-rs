use eframe::egui::Color32;
use eframe::epaint::Hsva;
use crate::math::expr::Expression;
use crate::math::values::Value;

pub struct TopLevelExpression {
    pub expression: Expression,
    pub expression_hash: u64,
    
    pub answer_cached: Option<Value>,
    
    pub graph_cache: Vec<(f64, f64, Hsva)>
}