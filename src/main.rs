use crate::gui::app::CalculatorApp;
use crate::gui::idx::new_id;
use crate::math::expr::Expression;
use eframe::{run_native, NativeOptions};

mod gui;
mod math;

fn main() {
    let options = NativeOptions::default();
    let app = CalculatorApp {
        exprs: vec![
            Expression::Literal(String::new(), new_id()),
            Expression::Literal(String::new(), new_id()),
            Expression::Literal(String::new(), new_id()),
            Expression::Literal(String::new(), new_id()),
            Expression::Literal(String::new(), new_id()),
        ],
    };
    run_native("Calculator", options, Box::new(|cc| Ok(Box::new(app))))
        .expect("failed to open window");
}
