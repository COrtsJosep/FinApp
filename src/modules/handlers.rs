use crate::modules::database::*;
use eframe::egui;

#[derive(Default)]
pub struct AppState {
    show_input_window: bool,
    show_input_entity: bool,
    show_input_account: bool,
    show_input_party: bool,
    show_input_income: bool,
    show_input_expense: bool,
    show_input_fundchange: bool,
    show_plotting_window: bool,
    database: DataBase,
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("welcome to you personal finances app!");
            if ui.button("Add records").clicked() {
                self.show_input_window = true;
            };

            if ui.button("Plotting").clicked() {
                self.show_plotting_window = true;
            };
        });

        if self.show_input_window {
            self.handle_show_input_window(ctx);
        }
    }
}

impl AppState {
    pub fn handle_show_input_window(&mut self, ctx: &egui::Context) {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("immediate_viewport"),
            egui::ViewportBuilder::default()
                .with_title("Immediate Viewport")
                .with_inner_size([200.0, 100.0]),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label("Hello from immediate viewport");
                });

                if ctx.input(|i| i.viewport().close_requested()) {
                    self.show_input_window = false;
                }
            },
        );
    }
}
