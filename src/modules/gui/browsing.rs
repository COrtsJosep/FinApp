use crate::modules::financial::*;
use crate::modules::gui::{AppState, WINDOW_HEIGHT, WINDOW_WIDTH};
use eframe::egui;
use egui::{Align, ComboBox, Layout};
use egui_extras::*;
use strum::IntoEnumIterator;

impl AppState {
    pub fn handle_show_browse_window(&mut self, ctx: &egui::Context) -> () {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("browse_window"),
            egui::ViewportBuilder::default()
                .with_title("Last transactions window")
                .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    let last_transactions_csv = self.last_transactions_csv.clone();
                    let header_line: String =
                        last_transactions_csv.split("\n").collect::<Vec<&str>>()[0].to_string();
                    let row_lines: Vec<&str> =
                        last_transactions_csv.split("\n").collect::<Vec<&str>>()[1..].to_vec();
                    let column_count: usize = header_line.split(",").count();

                    StripBuilder::new(ui)
                        .size(Size::exact(40.0))
                        .size(Size::remainder().at_least(120.0))
                        .vertical(|mut strip| {
                            strip.cell(|ui| {
                                egui::Grid::new("last_transactions")
                                    .num_columns(3)
                                    .spacing([45.0, 4.0])
                                    .show(ui, |ui| {
                                        ui.label("");
                                        if ui.button("Generate!").clicked() {
                                            self.last_transactions_csv =
                                                self.database.last_transactions(10);
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
                                                ui.strong(column_name).on_hover_text(column_name);
                                            });
                                        }
                                    })
                                    .body(|mut body| {
                                        for row_line in row_lines {
                                            body.row(30.0, |mut row_ui| {
                                                for element in row_line.split(",") {
                                                    row_ui.col(|ui| {
                                                        ui.label(element);
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
                    self.show_browse_window = false;
                }
            },
        );
    }
}
