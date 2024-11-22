use crate::gui::app::CalculatorApp;
use eframe::{run_native, NativeOptions};

mod gui;
mod math;

fn main() {
    let options = NativeOptions::default();
    let app = CalculatorApp {
        exprs: vec![],
        complex_axis_input: 0.0
    };
    run_native("Calculator", options, Box::new(|_cc| Ok(Box::new(app))))
        .expect("failed to open window");
}
