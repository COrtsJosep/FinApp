use crate::modules::gui::{AppState, WINDOW_HEIGHT, WINDOW_WIDTH};
use eframe::egui;
use egui::{Align, Color32, Layout};
use egui_extras::*;

impl AppState {
    fn is_valid_last_transactions_n(&self) -> bool {
        let parsing_result = self.last_transactions_n_temptative.parse::<usize>();
        match parsing_result {
            Ok(_value) => true,
            Err(_e) => false,
        }
    }

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
                                        ui.label("Number of records:").on_hover_text("Number of income/expense records to show.");
                                        ui.text_edit_singleline(&mut self.last_transactions_n_temptative);
                                        if self.is_valid_last_transactions_n() {
                                            ui.colored_label(
                                                Color32::from_rgb(110, 255, 110),
                                                "Valid number of records!",
                                            );

                                        } else {
                                            ui.colored_label(
                                                Color32::from_rgb(255, 0, 0),
                                                "Invalid number of records!",
                                            );
                                        }
                                        ui.end_row();
                                    });
                                ui.separator();
                                ui.vertical_centered_justified(|ui| {
                                    if self.is_valid_last_transactions_n() {
                                        if ui.button("Generate!").clicked() {
                                            self.last_transactions_n = self.last_transactions_n_temptative.parse::<usize>().expect("Failed to parse the number of last transactions.");
                                            self.last_transactions_csv = self.database.last_transactions(self.last_transactions_n);
                                        }
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
                                            let mut i = 0;
                                            for element in row_line.split(",") {
                                                row_ui.col(|ui| {
                                                    if i == 8 {
                                                        // index of the last column
                                                        if ui.button("Edit/Remove").on_hover_text("Removes the party from the database, and launches the input menu with an equal party already loaded").clicked() {
                                                            // i = 0;
                                                            let party_id: i64 =
                                                                element.parse().unwrap();
                                                            self.party = self.database.party(party_id);
                                                            self.database.delete_party(party_id);
                                                            self.database.save();

                                                            self.show_input_party_window = true;
                                                            self.show_browse_window = false;
                                                        }
                                                    } else {
                                                        ui.label(element);
                                                    }
                                                });

                                                i += 1;
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
