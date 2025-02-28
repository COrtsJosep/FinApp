use crate::modules::financial::*;
use chrono::Local;
use eframe::egui;
use egui::{Align, Layout};
use egui_extras::*;
use crate::modules::gui::{AppState, WINDOW_HEIGHT, WINDOW_WIDTH};

impl AppState {
pub fn handle_show_monthly_summary_window(&mut self, ctx: &egui::Context) -> () {
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

            if self.monthly_summary == String::default() {
                self.monthly_summary = self
                    .database
                    .monthly_summary(Local::now().date_naive(), &Currency::CHF);
            }

            let header_line: String =
                self.monthly_summary.split("\n").collect::<Vec<&str>>()[0].to_string();
            let row_lines: Vec<&str> =
                self.monthly_summary.split("\n").collect::<Vec<&str>>()[1..].to_vec();
            let column_count: usize = header_line.split(",").count();

            egui::CentralPanel::default().show(ctx, |ui| {
                TableBuilder::new(ui)
                    .columns(Column::auto().resizable(true), column_count)
                    .striped(true)
                    .cell_layout(Layout::right_to_left(Align::Center))
                    .header(20.0, |mut header| {
                        for column_name in header_line.split(",") {
                            header.col(|ui| {
                                ui.strong(column_name).on_hover_text(column_name);
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
            });
            if ctx.input(|i| i.viewport().close_requested()) {
                self.show_monthly_summary_window = false;
            }
        },
    )
}
}