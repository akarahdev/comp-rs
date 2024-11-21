use crate::math::context::Context;
use crate::math::expr::{BinaryOperation, Expression, UnaryOperation};
use eframe::egui::{Color32, ComboBox, Frame, Response, Stroke, TextEdit, Ui, Vec2};
use std::fmt::format;
use std::time::Instant;
use crate::gui::idx::new_id;

impl Expression {
    pub fn render(&mut self, ui: &mut Ui) -> Response {
        match self {
            Expression::Unary(op, val, id) => generate_frame(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(op.to_string());
                    val.render(ui);
                });
            }),
            Expression::Binary(op, lhs, rhs, id) => {
                if let BinaryOperation::Divide = op {
                    generate_frame(ui, |ui| {
                        ui.vertical(|ui| {
                            let upper = lhs.render(ui);
                            generate_binop_box(ui, op, *id);
                            rhs.render(ui);
                        });
                    })
                } else {
                    generate_frame(ui, |ui| {
                        ui.horizontal(|ui| {
                            lhs.render(ui);
                            generate_binop_box(ui, op, *id);
                            rhs.render(ui);
                        });
                    })
                }
            }
            Expression::Literal(str, id) => {
                ui.add_sized(Vec2::new(50.0, 20.0), TextEdit::singleline(str))
            }
            Expression::Parenthesis(val, id) => generate_frame(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("(");
                    val.render(ui);
                    ui.label(")");
                });
            }),
            Expression::Vector(exprs, id) => generate_frame(ui, |ui| {
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
                        exprs.push(Expression::Literal("".to_string(), new_id()));
                    };
                    ui.label("]");
                });

            })
        }
    }
}

pub fn generate_frame<F: FnMut(&mut Ui)>(ui: &mut Ui, f: F) -> Response {
    Frame::default()
        .stroke(Stroke::new(3.0, Color32::from_black_alpha(50)))
        .inner_margin(6.0)
        .show(ui, f)
        .response
}

fn generate_binop_box(ui: &mut Ui, op: &mut BinaryOperation, id: u64) -> Response {
    ComboBox::new(id, "Operand")
        .width(4.0)
        .selected_text(op.to_string())
        .show_ui(ui, |ui| {
            ui.selectable_value(op, BinaryOperation::Add, BinaryOperation::Add.to_string());
            ui.selectable_value(op, BinaryOperation::Sub, BinaryOperation::Sub.to_string());
            ui.selectable_value(
                op,
                BinaryOperation::Multiply,
                BinaryOperation::Multiply.to_string(),
            );
            ui.selectable_value(
                op,
                BinaryOperation::Divide,
                BinaryOperation::Divide.to_string(),
            );
            ui.selectable_value(
                op,
                BinaryOperation::Power,
                BinaryOperation::Power.to_string(),
            );
            ui.selectable_value(op, BinaryOperation::Root, BinaryOperation::Root.to_string());
            ui.selectable_value(
                op,
                BinaryOperation::Store,
                BinaryOperation::Store.to_string(),
            );
        })
        .response
}

fn generate_unop_box(ui: &mut Ui, op: &mut UnaryOperation, id: u64) -> Response {
    ComboBox::new(id, "")
        .width(4.0)
        .selected_text(op.to_string())
        .show_ui(ui, |ui| {
            ui.selectable_value(
                op,
                UnaryOperation::Negate,
                UnaryOperation::Negate.to_string(),
            );
        })
        .response
}
