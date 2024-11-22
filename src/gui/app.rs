use crate::gui::idx::new_id;
use crate::math::context::Context as MathContext;
use crate::math::expr::Expression;
use crate::math::expr::Expression::GraphExpression;
use crate::math::values::Value;
use eframe::egui::{CentralPanel, Context, ScrollArea, SidePanel, Slider, SliderClamping, Ui};
use eframe::epaint::Hsva;
use eframe::{App, Frame};
use egui_plot::Line;
use egui_plot::{Plot, PlotPoints, PlotUi};
use num::complex::Complex64;
use std::time::Instant;

pub struct CalculatorApp {
    pub(crate) exprs: Vec<Expression>,
    pub complex_axis_input: f64,
}

impl CalculatorApp {
    fn render_left_panel(&mut self, ui: &mut Ui) {
        let mut ctx = MathContext::default();

        let mut index = 0;
        let mut mark_remove: i32 = -1;

        ui.vertical(|ui| {
            for expr in &mut self.exprs {
                expr.render(ui);
                expr.update();
                ui.label(format!("= {}", expr.eval(&mut ctx)));
                ui.horizontal(|ui| {
                    ui.spacing();
                });

                if ui.button("Delete Expression").clicked() {
                    mark_remove = index.clone();
                }

                index += 1;
            }

            if mark_remove != -1 {
                self.exprs.remove(mark_remove as usize);
            }

            let add_btn = ui.button("+");
            if add_btn.clicked() {
                self.exprs.push(Expression::Literal("".to_string(), new_id()));
            }
        });
    }

    fn render_plot(&mut self, ui: &mut Ui) {
        let slider = Slider::new(&mut self.complex_axis_input, -2.0..=2.0)
            .clamping(SliderClamping::Never)
            .text("Complex Axis Input")
            .step_by(0.01);

        let plot = Plot::new("Graph")
            .view_aspect(1.0);

        ui.spacing_mut().slider_width *= 4.0;
        ui.add(slider);

        plot.show(ui, |mut plot_ui| {
            let bounds = plot_ui.plot_bounds();
            let min_x = bounds.min()[0];
            let max_x = bounds.max()[0];

            // this is actually awful for performance!! (time per frame go BRRRRR)
            // i really need to optimize this whole thing by figuring out the perfect STEPS
            // value
            const STEPS: i32 = 5000;
            let step_dist = (max_x - min_x) / STEPS as f64;

            for expr in &mut self.exprs {
                let GraphExpression(expr) = expr else {
                    break;
                };

                for step_count in 0..STEPS {
                    let x = min_x + (step_dist * step_count as f64);
                    let mut ctx = MathContext::default();
                    ctx.set_variable("x".to_string(), Value::Number(Complex64::new(x, self.complex_axis_input)));
                    let result = expr.eval(&mut ctx);
                    render_plot_point(&result, x, &mut plot_ui);
                }
            }
        });
    }
}

impl App for CalculatorApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let start = Instant::now();

        SidePanel::left("left_panel")
            .default_width(400.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ScrollArea::horizontal()
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            self.render_left_panel(ui);
                        });
                });
            });

        CentralPanel::default().show(ctx, |ui| {
            self.render_plot(ui);
        });
        let end = Instant::now();

        println!("render time: {:?}ms", (end - start).as_millis());
    }
}

fn render_plot_point(value: &Value, x: f64, ui: &mut PlotUi) {
    match value {
        Value::Number(num) => {
            let mut color = Hsva::new(0.5, 1.0, 1.0, 1.0);
            color.h += (num.im / 10.0) as f32;
            ui.line(
                Line::new(PlotPoints::new(vec![[x, num.re]]))
                    .color(color),
            )
        }
        Value::Vector(vec) => {
            vec.iter().for_each(|vx| render_plot_point(vx, x, ui));
        }
        Value::Error(_err) => {}
    }
}