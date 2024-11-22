use crate::gui::app::CalculatorApp;
use crate::gui::idx::new_id;
use crate::math::expr::Expression;
use eframe::{run_native, NativeOptions};
use num::complex::Complex64;

mod gui;
mod math;

fn main() {
    let options = NativeOptions::default();
    let app = CalculatorApp {
        exprs: vec![],
        complex_axis_input: 0.0
    };
    run_native("Calculator", options, Box::new(|cc| Ok(Box::new(app))))
        .expect("failed to open window");
}
