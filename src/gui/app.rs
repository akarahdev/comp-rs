use crate::math::context::Context as MathContext;
use crate::math::expr::Expression;
use eframe::egui::{CentralPanel, Context};
use eframe::{App, Frame};
use std::fmt::format;
use crate::gui::graph::generate_frame;
use crate::gui::idx::new_id;

pub struct CalculatorApp {
    pub(crate) exprs: Vec<Expression>,
}

impl App for CalculatorApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let mut ctx = MathContext::new();

            let mut index = 0;
            let mut mark_remove = -1;

            for expr in &mut self.exprs {

                generate_frame(ui, |ui| {
                        expr.render(ui);
                        expr.update();
                        ui.label(format!("= {}", expr.eval(&mut ctx)));
                        ui.horizontal(|ui| {
                            ui.spacing();
                        });

                        if ui.button("Delete Expression").clicked() {
                            mark_remove = index.clone();
                        }
                    });

                index += 1;
            }

            if mark_remove != -1 {
                self.exprs.remove(mark_remove as usize);
            }
            if ui.button("Add Expression").clicked() {
                self.exprs.push(Expression::Literal("".to_string(), new_id()));
            }
        });
    }
}
