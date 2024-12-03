use crate::gui::idx::new_id;
use crate::math::expr::{BinaryOperation, Expression, UnaryOperation};
use eframe::egui::{Color32, ComboBox, Frame, Id, Response, Sense, Stroke, TextEdit, Ui, Vec2};

impl Expression {
    pub fn render(&mut self, ui: &mut Ui) -> Response {
        match self {
            Expression::Unary {
                operation,
                expr,
                id,
            } => generate_frame(ui, |ui| {
                generate_frame(ui, |ui| {
                    ui.horizontal(|ui| {
                        generate_unop_box(ui, operation, *id);
                        expr.render(ui);
                    });
                });
            }),
            Expression::Binary { op, lhs, rhs, id } => {
                match op {
                    BinaryOperation::Divide => generate_frame(ui, |ui| {
                        ui.vertical(|ui| {
                            lhs.render(ui);
                            generate_binop_box(ui, op, *id);
                            rhs.render(ui);
                        });
                    }),
                    BinaryOperation::Invoke => generate_frame(ui, |ui| {
                        ui.horizontal(|ui| {
                            lhs.render(ui);
                            generate_binop_box(ui, op, *id);
                            rhs.render(ui);
                            ui.label(")");
                        });
                    }),
                    _ => generate_frame(ui, |ui| {
                        ui.horizontal(|ui| {
                            lhs.render(ui);
                            generate_binop_box(ui, op, *id);
                            rhs.render(ui);
                        });
                    })
                }
            }
            Expression::Literal { content, id, new_literal } => {
                let len = content.len().clone();
                let mut text_edit = TextEdit::singleline(content);
                text_edit = text_edit.id(Id::new(*id));

                let resp = ui.add_sized(
                    Vec2::new(f32::min((len * 15 + 20) as f32, 40.0), 15.0),
                    text_edit,
                );
                if *new_literal {
                    *new_literal = false;
                    ui.memory_mut(|mem| {
                        mem.focused().inspect(|id| mem.surrender_focus(*id));
                        mem.request_focus(resp.id);
                    });

                    if resp.has_focus() {
                        resp.interact(Sense::click());
                        resp.interact(Sense::click());
                        println!("Focused!");
                    }
                }


                resp

            }
            Expression::Parenthesis { expr, id, unbox_to_binop } => generate_frame(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("(");
                    expr.render(ui);
                    if ui.button(")").clicked() {
                        *unbox_to_binop = true;
                    }
                });
            }),
            Expression::Vector { exprs, id } => render_vec(ui, exprs),
            Expression::GraphExpression { expr } => generate_frame(ui, |ui| {
                ui.label("Graph f(x)=");
                expr.render(ui);
            }),
            Expression::Summation {
                minimum,
                maximum,
                variable,
                expression,
            } => render_summation(ui, minimum, maximum, variable, expression),
            Expression::Lambda { variable, expr } => generate_frame(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("fn(");
                    variable.render(ui);
                    ui.label(") = ");
                    expr.render(ui);
                });
            })
        }
    }
}

fn render_vec(ui: &mut Ui, exprs: &mut Vec<Expression>) -> Response {
    generate_frame(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label("[");
            let mut index = 0;
            let mut remove = -1;
            let len = exprs.len().clone();
            for expr in &mut *exprs {
                expr.render(ui);
                index += 1;
                if ui.small_button("-").clicked() {
                    remove = index as i64 - 1;
                }
                if index != len {
                    ui.label(",");
                }
            }
            if remove != -1 {
                exprs.remove(remove as usize);
            }
            if ui.button("+").clicked() {
                exprs.push(Expression::Literal {
                    content: "".to_string(),
                    id: new_id(),
                    new_literal: true,
                });
            };
            ui.label("]");
        });
    })
}

fn render_summation(
    ui: &mut Ui,
    minimum: &mut Expression,
    maximum: &mut Expression,
    variable: &mut Expression,
    expression: &mut Expression,
) -> Response {
    generate_frame(ui, |ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                maximum.render(ui);
                ui.label("Summation");
                ui.horizontal(|ui| {
                    variable.render(ui);
                    ui.label("=");
                    minimum.render(ui);
                });
            });
            ui.label("=");
            expression.render(ui);
        });
    })
}

fn generate_frame<F: FnOnce(&mut Ui)>(ui: &mut Ui, f: F) -> Response {
    Frame::default()
        .stroke(Stroke::new(1.0, Color32::from_black_alpha(50)))
        .inner_margin(2.0)
        .show(ui, f)
        .response
}



fn generate_binop_box(ui: &mut Ui, op: &mut BinaryOperation, id: u64) -> Response {
    ComboBox::new(id, "")
        .width(3.0)
        .selected_text(op.to_string())
        .show_ui(ui, |ui| {
            ui.selectable_value(op, BinaryOperation::Add, BinaryOperation::Add.to_string());
            ui.selectable_value(op, BinaryOperation::Sub, BinaryOperation::Sub.to_string());
            ui.selectable_value(op, BinaryOperation::Multiply, BinaryOperation::Multiply.to_string());
            ui.selectable_value(op, BinaryOperation::Divide, BinaryOperation::Divide.to_string());
            ui.selectable_value(op, BinaryOperation::Power, BinaryOperation::Power.to_string());
            ui.selectable_value(op, BinaryOperation::Root, BinaryOperation::Root.to_string());
            ui.selectable_value(op, BinaryOperation::Store, BinaryOperation::Store.to_string());
            ui.selectable_value(op, BinaryOperation::Equal, BinaryOperation::Equal.to_string());
            ui.selectable_value(op, BinaryOperation::GreaterThan, BinaryOperation::GreaterThan.to_string());
            ui.selectable_value(op, BinaryOperation::LessThan, BinaryOperation::LessThan.to_string());
            ui.selectable_value(op, BinaryOperation::GreaterThanOrEqual, BinaryOperation::GreaterThanOrEqual.to_string());
            ui.selectable_value(op, BinaryOperation::LessThanOrEqual, BinaryOperation::LessThanOrEqual.to_string());
        })
        .response
}

fn generate_unop_box(ui: &mut Ui, op: &mut UnaryOperation, id: u64) -> Response {
    ComboBox::new(id, "")
        .width(4.0)
        .selected_text(op.to_string())
        .show_ui(ui, |ui| {
            ui.selectable_value(op, UnaryOperation::Negate, UnaryOperation::Negate.to_string());
            ui.selectable_value(op, UnaryOperation::Sin, UnaryOperation::Sin.to_string());
            ui.selectable_value(op, UnaryOperation::Cos, UnaryOperation::Cos.to_string());
            ui.selectable_value(op, UnaryOperation::Tan, UnaryOperation::Tan.to_string());
            ui.selectable_value(op, UnaryOperation::InverseSin, UnaryOperation::InverseSin.to_string());
            ui.selectable_value(op, UnaryOperation::InverseCos, UnaryOperation::InverseCos.to_string());
            ui.selectable_value(op, UnaryOperation::InverseTan, UnaryOperation::InverseTan.to_string());
            ui.selectable_value(op, UnaryOperation::HyperbolicSin, UnaryOperation::HyperbolicSin.to_string());
            ui.selectable_value(op, UnaryOperation::HyperbolicCos, UnaryOperation::HyperbolicCos.to_string());
            ui.selectable_value(op, UnaryOperation::HyperbolicTan, UnaryOperation::HyperbolicTan.to_string());
            ui.selectable_value(op, UnaryOperation::InverseHyperbolicSin, UnaryOperation::InverseHyperbolicSin.to_string());
            ui.selectable_value(op, UnaryOperation::InverseHyperbolicCos, UnaryOperation::InverseHyperbolicCos.to_string());
            ui.selectable_value(op, UnaryOperation::InverseHyperbolicTan, UnaryOperation::InverseHyperbolicTan.to_string());
        })
        .response
}
