use eframe::egui;
use FinApp::modules::gui::*;

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Financial Application",
        options,
        Box::new(|_cc| Ok(Box::<AppState>::default())),
    )
}
