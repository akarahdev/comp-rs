use crate::math::expr::Expression;
use crate::math::values::Value;
use eframe::egui::Color32;
use eframe::epaint::Hsva;

/// Represents an expression on a sidebar with lots of metadata.
/// The metadata is used to cache expression information and to compute it
/// more efficiently and across multiple threads.
pub struct TopLevelExpression {
    /// The main expression associated with this top level expression.
    pub expression: Expression,
    /// The last known hash of the Expression. Used to prevent having to compute
    /// equality of a potentially really deep expression.
    pub expression_hash: u64,
    /// Represents the last known answer to the solution of the
    /// provided expression. This is not used by the graphing system.
    pub answer_cached: Option<Value>,
    /// Represents a cache of each point currently on the graph
    /// and it's expected color (the color represents a value in the complex axis)
    pub graph_cache: Vec<(f64, f64, Hsva)>,
    /// The last known size of the graph UI boundaries.
    /// The elements are as such: minimum X, maximum X, minimum Y, maximum Y
    pub graph_size_cache: (f64, f64, f64, f64),
}
