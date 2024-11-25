use crate::gui::idx::new_id;
use crate::gui::top::TopLevelExpression;
use crate::math::context::Context as MathContext;
use crate::math::expr::Expression;
use crate::math::expr::Expression::GraphExpression;
use crate::math::values::Value;
use eframe::egui::{CentralPanel, Context, ScrollArea, SidePanel, Slider, SliderClamping, Ui};
use eframe::epaint::Hsva;
use eframe::{App, Frame};
use egui_plot::{Line, Points};
use egui_plot::{Plot, PlotPoints, PlotUi};
use num::complex::Complex64;
use parking_lot::Mutex;
use std::cmp::min;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;
use std::time::Instant;

pub struct CalculatorApp {
    pub(crate) exprs: Vec<Arc<Mutex<TopLevelExpression>>>,
    pub complex_axis_input: f64,
}

impl CalculatorApp {
    fn render_left_panel(&mut self, ui: &mut Ui) {
        let mut ctx = MathContext::default();

        let mut index = 0;
        let mut mark_remove: i32 = -1;

        ui.vertical(|ui| {
            for mutex_expr in &self.exprs {
                let mut expr = mutex_expr.lock();
                expr.expression.render(ui);
                expr.expression.update();
                expr.expression.enforce_ooo();

                let mut hasher = DefaultHasher::new();
                expr.expression.hash(&mut hasher);
                if hasher.finish() != expr.expression_hash {
                    println!(
                        "Reseting hash of {:?} {} vs {}",
                        expr.expression,
                        hasher.finish(),
                        expr.expression_hash
                    );
                    expr.expression_hash = hasher.finish();
                    expr.answer_cached = None;
                }
                if let Some(answer) = &expr.answer_cached {
                    ui.label(format!("= {}", answer));
                } else {
                    let async_expr = mutex_expr.clone();
                    std::thread::spawn(move || {
                        let mut expr = async_expr.lock();
                        let answer = expr.expression.eval(&mut MathContext::default());
                        expr.answer_cached = Some(answer);
                    });
                    ui.label("= Computing...");
                }

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
                self.exprs.push(Arc::new(Mutex::new(TopLevelExpression {
                    expression: Expression::Literal {
                        content: "".to_string(),
                        id: new_id(),
                    },
                    expression_hash: u64::MAX,
                    answer_cached: None,
                    graph_cache: vec![],
                    graph_data_cache: (0.0, 0.0, 0.0, 0.0, 0.0),
                })));
            }
        });
    }

    fn render_plot(&mut self, ui: &mut Ui) {
        let slider = Slider::new(&mut self.complex_axis_input, -2.0..=2.0)
            .clamping(SliderClamping::Never)
            .text("Complex Axis Input")
            .step_by(0.01);

        let plot = Plot::new("Graph").view_aspect(1.0);

        ui.spacing_mut().slider_width *= 4.0;
        ui.add(slider);

        plot.show(ui, |plot_ui| {
            let bounds = plot_ui.plot_bounds();
            let min_x = bounds.min()[0];
            let max_x = bounds.max()[0];
            let min_y = bounds.min()[1];
            let max_y = bounds.max()[1];

            let STEPS: i32 = 10000;
            let step_dist = (max_x - min_x) / STEPS as f64;
            let cai = self.complex_axis_input;

            for mutex_expr in &self.exprs {
                let mutex_result = mutex_expr.lock();
                let GraphExpression { ref expr } = mutex_result.expression else {
                    break;
                };

                for point in &mutex_result.graph_cache {
                    plot_ui.points(
                        Points::new(PlotPoints::new(vec![[point.0, point.1]])).color(point.2),
                    )
                }

                if mutex_result.graph_data_cache.0 != min_x
                    || mutex_result.graph_data_cache.1 != max_x
                    || mutex_result.graph_data_cache.2 != min_y
                    || mutex_result.graph_data_cache.3 != max_y
                    || mutex_result.graph_data_cache.4 != cai
                {
                    let cloned_mutex_for_graph_size = mutex_expr.clone();
                    std::thread::spawn(move || {
                        cloned_mutex_for_graph_size.lock().graph_data_cache =
                            (min_x, max_x, min_y, max_y, cai);
                    });

                    let cloned_mutex_expr = mutex_expr.clone();
                    let cloned_expr = expr.clone();

                    std::thread::spawn(move || {
                        let mut results = vec![];
                        for step_count in 0..STEPS {
                            let x = min_x + (step_dist * step_count as f64);
                            let mut ctx = MathContext::default();
                            ctx.set_variable(
                                "x".to_string(),
                                Value::Number(Complex64::new(x, cai)),
                            );
                            let result = cloned_expr.eval(&mut ctx);
                            match result {
                                Value::Number(num) => {
                                    let mut color = Hsva::new(0.5, 1.0, 1.0, 1.0);
                                    color.h += (num.im / 10.0) as f32;
                                    results.push((x, num.re, color));
                                }
                                _ => {}
                            }
                        }
                        cloned_mutex_expr.lock().graph_cache = results;
                    });
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

fn render_plot_point(value: &Value, x: f64, ui: &mut PlotUi) {}
