use crate::modules::financial::*;
use crate::modules::gui::{AppState, WINDOW_HEIGHT, WINDOW_WIDTH};
use eframe::egui;
use egui::{Align, ComboBox, Layout};
use egui_extras::*;
use strum::IntoEnumIterator;

impl AppState {
    pub fn handle_show_expense_summary_window(&mut self, ctx: &egui::Context) -> () {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("input_monthly_summary_window"),
            egui::ViewportBuilder::default()
                .with_title("Monthly summary window")
                .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    let expense_summary_csv = self.expense_summary_csv.clone();
                    let header_line: String = expense_summary_csv.split("\n").collect::<Vec<&str>>()[0].to_string();
                    let row_lines: Vec<&str> = expense_summary_csv.split("\n").collect::<Vec<&str>>()[1..].to_vec();
                    let column_count: usize = header_line.split(",").count();

                    StripBuilder::new(ui)
                        .size(Size::exact(40.0))
                        .size(Size::remainder().at_least(120.0))
                        .vertical(|mut strip| {
                            strip.cell(|ui| {
                                egui::Grid::new("expense_summary")
                                    .num_columns(3)
                                    .spacing([45.0, 4.0])
                                    .show(ui, |ui| {
                                        ui.label("Start date:").on_hover_text("Include expenses in the summary starting on the specified date, included.");
                                        ui.add(DatePickerButton::new(&mut self.expense_summary_date_from).id_salt("date_from"));
                                        ui.end_row();

                                        ui.label("End date:").on_hover_text("Include expenses in the summary until the specified date, included.");
                                        ui.add(DatePickerButton::new(&mut self.expense_summary_date_to).id_salt("date_to"));
                                        ui.end_row();

                                        ui.label("Currency:").on_hover_text("Currency in which to express the ammounts.");
                                        ComboBox::from_id_salt("Expense summary currency")
                                .selected_text(format!("{}", self.expense_summary_currency))
                                .show_ui(ui, |ui| {
                                    for possible_expense_summary_currency in Currency::iter() {
                                        ui.selectable_value(
                                            &mut self.expense_summary_currency,
                                            possible_expense_summary_currency.clone(),
                                            format!("{possible_expense_summary_currency}"),
                                        );
                                    }
                                });
                                        ui.end_row();

                                        ui.label("");
                                        if ui.button("Generate!").clicked() {
                                            self.expense_summary_csv = self.database.expenses_summary(
                                                self.expense_summary_date_from,
                                                self.expense_summary_date_to,
                                                &self.expense_summary_currency
                                            );
                                        }

                                    });
                                ui.separator();
                            });
                            strip.cell(|ui| {
                                                TableBuilder::new(ui)
                                        .columns(Column::auto().resizable(true), column_count)
                                        .striped(true)
                                        .cell_layout(Layout::right_to_left(Align::Center))
                                        .header(20.0, |mut header| {
                                            for column_name in header_line.split(",") {
                                                header.col(|ui| {
                                                    ui.strong(column_name)
                                                        .on_hover_text(column_name);
                                                });
                                            }
                                        })
                                        .body(|mut body| {
                                            for row_line in row_lines {
                                                body.row(30.0, |mut row_ui| {
                                                    let mut is_last_row: bool = false;
                                                    for element in row_line.split(",") {
                                                        if element == "Total" {
                                                            is_last_row = true;
                                                        }
                                                        row_ui.col(|ui| {
                                                            if is_last_row {
                                                                ui.strong(element);
                                                            } else {
                                                                ui.label(element);
                                                            }
                                                        });
                                                    }
                                                });
                                            }                         
                                        });
                                ui.separator();
                            });
                        });
                });
                if ctx.input(|i| i.viewport().close_requested()) {
                    self.show_expense_summary_window = false;
                }
            },
        )
    }
}
