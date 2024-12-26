use crate::modules::database::*;
use crate::modules::financial::*;
use eframe::egui;
use eframe::egui::ComboBox;
use strum::IntoEnumIterator;

#[derive(Default)]
pub struct AppState {
    show_input_window: bool,
    show_input_entity_window: bool,
    show_input_account_window: bool,
    show_input_party_window: bool,
    show_input_income_window: bool,
    show_input_expense_window: bool,
    show_input_fundchange_window: bool,
    show_plotting_window: bool,

    database: DataBase,

    entity_name: String,
    entity_country: String,
    entity_type: EntityType,
    entity_subtype: String,
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) -> () {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Welcome to you personal finances app!");
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

        if self.show_input_entity_window {
            self.handle_show_input_entity_window(ctx);
        }

        if self.show_plotting_window {
            // todo
        }
    }
}

impl AppState {
    fn clear_fields(&mut self) -> () {
        self.entity_name = String::default();
        self.entity_country = String::default();
        self.entity_type = EntityType::default();
        self.entity_subtype = String::default();
    }

    pub fn handle_show_input_window(&mut self, ctx: &egui::Context) -> () {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("input_window"),
            egui::ViewportBuilder::default()
                .with_title("Input window")
                .with_inner_size([400.0, 250.0]),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label("Input window");
                    if ui.button("Add new entity").clicked() {
                        // unsure whether to involve show_input_window
                        self.show_input_entity_window = self.show_input_window & true;
                    }
                });

                if ctx.input(|i| i.viewport().close_requested()) {
                    self.show_input_window = false;
                }
            },
        )
    }

    pub fn handle_show_input_entity_window(&mut self, ctx: &egui::Context) -> () {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("input_entity_window"),
            egui::ViewportBuilder::default()
                .with_title("Input entity window")
                .with_inner_size([400.0, 250.0]),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label("Input window");
                    ui.horizontal(|ui| {
                        let entity_name_label = ui.label("Entity name: ");
                        ui.text_edit_singleline(&mut self.entity_name)
                            .labelled_by(entity_name_label.id);
                    });
                    ui.horizontal(|ui| {
                        let entity_country_label = ui.label("Entity country: ");
                        ui.text_edit_singleline(&mut self.entity_country)
                            .labelled_by(entity_country_label.id);
                    });
                    ComboBox::from_label("Entity type")
                        .selected_text(format!("{}", self.entity_type))
                        .show_ui(ui, |ui| {
                            for possible_entity_type in EntityType::iter() {
                                ui.selectable_value(
                                    &mut self.entity_type,
                                    possible_entity_type.clone(),
                                    format!("{possible_entity_type}"),
                                );
                            }
                        });

                    ui.horizontal(|ui| {
                        let entity_subtype_label = ui.label("Entity subtype: ");
                        ui.text_edit_singleline(&mut self.entity_subtype)
                            .labelled_by(entity_subtype_label.id);
                    });

                    if ui.button("Add new entity").clicked() {
                        let entity: Entity = Entity::new(
                            self.entity_name.clone(),
                            self.entity_country.clone(),
                            self.entity_type.clone(),
                            self.entity_subtype.clone(),
                        );

                        self.database.insert_entity(&entity);
                        self.database.save();
                        self.clear_fields();

                        self.show_input_entity_window = false;
                    }
                });

                if ctx.input(|i| i.viewport().close_requested()) {
                    self.show_input_entity_window = false;
                }
            },
        );
    }
}
