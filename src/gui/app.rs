use crate::math::context::Context as MathContext;
use crate::math::expr::Expression;
use eframe::egui::{CentralPanel, Context};
use eframe::{App, Frame};
use std::fmt::format;

pub struct CalculatorApp {
    pub(crate) exprs: Vec<Expression>,
}

impl App for CalculatorApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello world!");

            let mut ctx = MathContext::new();

            for expr in &mut self.exprs {
                expr.render(ui);
                expr.update();
                ui.label(format!("= {:?}", expr.eval(&mut ctx)));
                ui.horizontal(|ui| {
                    ui.spacing();
                });
            }
        });
    }
}
