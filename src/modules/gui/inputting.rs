use crate::modules::gui::{AppState, WINDOW_HEIGHT, WINDOW_WIDTH};

use crate::modules::financial::*;
use chrono::Local;
use eframe::egui;
use eframe::egui::{Color32, ComboBox};
use egui_autocomplete::AutoCompleteTextEdit;
use egui_extras::*;
use strum::IntoEnumIterator;

impl AppState {
    fn clear_fields(&mut self) -> () {
        *self = AppState::default();
    }

    fn clear_transaction_fields(&mut self) -> () {
        self.transaction_category = String::default();
        self.transaction_subcategory = String::default();
        self.transaction_description = String::default();
        self.transaction_entity_id = i64::default();
        self.transaction_entity_string = String::default();
        self.transaction_account_id = i64::default();
        self.transaction_account_string = String::default();
        self.transaction_type = TransactionType::default();
    }

    fn are_valid_entity_fields(&self) -> bool {
        (self.entity_name.len() > 0) & (self.entity_country.len() > 0)
    }

    fn is_valid_initial_balance(&self) -> bool {
        let parsing_result = self.account_initial_balance_tentative.parse::<f64>();
        match parsing_result {
            Ok(_value) => true,
            Err(_e) => false,
        }
    }

    fn are_valid_account_fields(&self) -> bool {
        (self.account_name.len() > 0)
            & (self.account_country.len() > 0)
            & self.is_valid_initial_balance()
    }

    fn is_valid_transaction_value(&self) -> bool {
        let parsing_result = self.transaction_value_tentative.parse::<f64>();
        match parsing_result {
            Ok(_value) => true,
            Err(_e) => false,
        }
    }

    fn is_valid_transaction_currency(&self) -> bool {
        &self.transaction_currency
            == self
                .database
                .account(self.transaction_account_id)
                .currency()
    }

    fn are_valid_transaction_fields(&self) -> bool {
        ((self.transaction_category.len() > 0)
            | (self.transaction_type.is_fund_change() & self.is_valid_transaction_currency()))
            & self.is_valid_transaction_value()
    }

    pub fn handle_show_input_window(&mut self, ctx: &egui::Context) -> () {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("input_window"),
            egui::ViewportBuilder::default()
                .with_title("Input window")
                .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    if ui.button("Add new entity").clicked() {
                        // unsure whether to involve show_input_window
                        self.show_input_entity_window = self.show_input_window & true;
                    }
                    if ui.button("Add new account").clicked() {
                        // unsure whether to involve show_input_window
                        self.show_input_account_window = self.show_input_window & true;
                    }
                    if ui.button("Add new transaction party").clicked() {
                        self.show_input_party_window = self.show_input_window & true;
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
                .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );
                egui::CentralPanel::default().show(ctx, |ui| {
                    egui::Grid::new("my_grid")
                        .num_columns(3)
                        .spacing([45.0, 4.0])
                        //.striped(true)
                        .show(ui, |ui| {
                            ui.label("Entity name:")
                                .on_hover_text("Name of the entity. For instance, the shop's name");
                            ui.text_edit_singleline(&mut self.entity_name);
                            if self.entity_name.len() > 0 {
                                ui.colored_label(
                                    Color32::from_rgb(110, 255, 110),
                                    "Valid entity name!",
                                );
                            } else {
                                ui.colored_label(
                                    Color32::from_rgb(255, 0, 0),
                                    "Please enter an entity name!",
                                );
                            }
                            ui.end_row();

                            ui.label("Entity country:")
                                .on_hover_text("Country where the entity is based.");
                            ui.add(
                                AutoCompleteTextEdit::new(
                                    &mut self.entity_country,
                                    self.database.entity_countries(),
                                )
                                .max_suggestions(10)
                                .highlight_matches(true),
                            );
                            if self.entity_country.len() > 0 {
                                ui.colored_label(
                                    Color32::from_rgb(110, 255, 110),
                                    "Valid entity country!",
                                );
                            } else {
                                ui.colored_label(
                                    Color32::from_rgb(255, 0, 0),
                                    "Please enter an entity country!",
                                );
                            }
                            ui.end_row();

                            ui.label("Entity type:")
                                .on_hover_text("Category of the entity.");
                            ComboBox::from_id_salt("Entity type")
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
                            ui.end_row();

                            ui.label("Entity subtype:")
                                .on_hover_text("Sub-category of the entity.");
                            ui.add(
                                AutoCompleteTextEdit::new(
                                    &mut self.entity_subtype,
                                    self.database.entity_subtypes(),
                                )
                                .max_suggestions(10)
                                .highlight_matches(true),
                            );
                            ui.end_row();
                        });

                    ui.separator();
                    ui.vertical_centered_justified(|ui| {
                        if self.are_valid_entity_fields() {
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
                        }
                    });
                });

                if ctx.input(|i| i.viewport().close_requested()) {
                    self.show_input_entity_window = false;
                }
            },
        );
    }
    pub fn handle_show_input_account_window(&mut self, ctx: &egui::Context) -> () {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("input_account_window"),
            egui::ViewportBuilder::default()
                .with_title("Input account window")
                .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    egui::Grid::new("my_grid")
                        .num_columns(3)
                        .spacing([45.0, 4.0])
                        //.striped(true)
                        .show(ui, |ui| {
                            ui.label("Account name: ").on_hover_text("Name of the account. For instance, the name of the bank, or the investment fund.");
                            ui.text_edit_singleline(&mut self.account_name);
                            if self.account_name.len() > 0 {
                                ui.colored_label(
                                    Color32::from_rgb(110, 255, 110),
                                    "Valid account name!",
                                );
                            } else {
                                ui.colored_label(
                                    Color32::from_rgb(255, 0, 0),
                                    "Invalid account name!",
                                );
                            }
                            ui.end_row();

                            ui.label("Account country: ").on_hover_text("Country where the account is based.");
                            ui.add(
                                AutoCompleteTextEdit::new(
                                    &mut self.account_country,
                                    self.database.account_countries(),
                                )
                                    .max_suggestions(10)
                                    .highlight_matches(true),
                            );
                            if self.account_country.len() > 0 {
                                ui.colored_label(
                                    Color32::from_rgb(110, 255, 110),
                                    "Valid account country!",
                                );
                            } else {
                                ui.colored_label(
                                    Color32::from_rgb(255, 0, 0),
                                    "Invalid account country!",
                                );
                            }
                            ui.end_row();

                            ui.label("Account currency: ").on_hover_text("Currency of the account. If multiple, consider creating various accounts with different currencies.");
                            ComboBox::from_id_salt("Account currency")
                                .selected_text(format!("{}", self.account_currency))
                                .show_ui(ui, |ui| {
                                    for possible_account_currency in Currency::iter() {
                                        ui.selectable_value(
                                            &mut self.account_currency,
                                            possible_account_currency.clone(),
                                            format!("{possible_account_currency}"),
                                        );
                                    }
                                });
                            ui.end_row();

                            ui.label("Account type: ").on_hover_text("Category of the account.");
                            ComboBox::from_id_salt("Account type")
                                .selected_text(format!("{}", self.account_type))
                                .show_ui(ui, |ui| {
                                    for possible_account_type in AccountType::iter() {
                                        ui.selectable_value(
                                            &mut self.account_type,
                                            possible_account_type.clone(),
                                            format!("{possible_account_type}"),
                                        );
                                    }
                                });
                            ui.end_row();

                            ui.label("Account initial balance: ").on_hover_text("Amount of money stored in the account, in the given currency, in this very moment.");
                            ui.text_edit_singleline(&mut self.account_initial_balance_tentative);
                            if self.is_valid_initial_balance() {
                                ui.colored_label(
                                    Color32::from_rgb(110, 255, 110),
                                    "Valid initial balance!",
                                );
                            } else {
                                ui.colored_label(
                                    Color32::from_rgb(255, 0, 0),
                                    "Invalid initial balance!",
                                );
                            }
                            ui.end_row();
                        });

                    ui.separator();
                    ui.vertical_centered_justified(|ui| {
                        if self.are_valid_account_fields() {
                            self.account_initial_balance = self
                                .account_initial_balance_tentative
                                .parse::<f64>()
                                .expect("Error parsing account initial balance");

                            if ui.button("Add new account").on_hover_text("Save account into the database.").clicked() {
                                let account: Account = Account::new(
                                    self.account_name.clone(),
                                    self.account_country.clone(),
                                    self.account_currency.clone(),
                                    self.account_type.clone(),
                                    self.account_initial_balance,
                                );
                                self.database.insert_account(&account);
                                self.database.save();
                                self.clear_fields();

                                self.show_input_account_window = false;
                            }
                        }
                    });
                });

                if ctx.input(|i| i.viewport().close_requested()) {
                    self.show_input_account_window = false;
                }
            },
        );
    }
    pub fn handle_show_input_party_window(&mut self, ctx: &egui::Context) -> () {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("input_party_window"),
            egui::ViewportBuilder::default()
                .with_title("Input party window")
                .with_inner_size([WINDOW_WIDTH * 1.5, WINDOW_HEIGHT]),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label("Input window");

                    ui.label("Transactions:");
                    for transaction in self.party.iter() {
                        ui.label(transaction.to_string());
                    }

                    if ui.button("Add new transaction").clicked() {
                        self.show_input_transaction_window = self.show_input_party_window & true;
                    }

                    if self.party.is_valid() {
                        if ui.button("Add party").clicked() {
                            self.database.insert_party(&mut self.party);
                            self.database.save();
                            self.clear_fields();

                            self.show_input_party_window = false;
                        }
                    }
                });
                if ctx.input(|i| i.viewport().close_requested()) {
                    self.show_input_party_window = false;
                }
            },
        );
    }
    pub fn handle_show_input_transaction_window(&mut self, ctx: &egui::Context) -> () {
        self.transaction_entity_string =
            self.database.entity(self.transaction_entity_id).to_string();
        self.transaction_account_string = self
            .database
            .account(self.transaction_account_id)
            .to_string();

        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("input_transaction_window"),
            egui::ViewportBuilder::default()
                .with_title("Input transaction window")
                .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
            |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    egui::Grid::new("my_grid")
                        .num_columns(3)
                        .spacing([45.0, 4.0])
                        //.striped(true)
                        .show(ui, |ui| {
                            ui.label("Transaction type:")
                                .on_hover_text("Category of the transaction");
                            // TODO: Consider adding much more detail to the hover text. This is an
                            // important point!
                            ui.horizontal(|ui| {
                                for transaction_type in TransactionType::iter() {
                                    ui.selectable_value(
                                        &mut self.transaction_type,
                                        transaction_type.clone(),
                                        transaction_type.to_string(),
                                    );
                                }
                            });
                            ui.end_row();

                            ui.label("Transaction value:")
                                .on_hover_text("Monetary value of the transaction.");
                            ui.text_edit_singleline(&mut self.transaction_value_tentative);
                            if self.is_valid_transaction_value() {
                                ui.colored_label(
                                    Color32::from_rgb(110, 255, 110),
                                    "Valid transaction value!",
                                );
                            } else {
                                ui.colored_label(
                                    Color32::from_rgb(255, 0, 0),
                                    "Invalid transaction value!",
                                );
                            }
                            ui.end_row();

                            ui.label("Transaction currency: ")
                                .on_hover_text("Currency of the transaction.");
                            ComboBox::from_id_salt("Transaction currency")
                                .selected_text(format!("{}", self.transaction_currency))
                                .show_ui(ui, |ui| {
                                    for possible_transaction_currency in Currency::iter() {
                                        ui.selectable_value(
                                            &mut self.transaction_currency,
                                            possible_transaction_currency.clone(),
                                            format!("{possible_transaction_currency}"),
                                        );
                                    }
                                });
                            if !self.is_valid_transaction_currency()
                                & self.transaction_type.is_fund_change()
                            {
                                ui.colored_label(
                                    Color32::from_rgb(255, 0, 0),
                                    "Mismatch between transaction and account currencies!",
                                );
                            }
                            ui.end_row();

                            ui.label("Transaction date:")
                                .on_hover_text("Date in which the transaction happened.");
                            ui.add(DatePickerButton::new(&mut self.transaction_date));
                            ui.end_row();

                            if self.transaction_type.is_fund_change() {
                                ui.label("Transaction account:")
                                    .on_hover_text("Account that is affected by the transaction.");
                                ComboBox::from_id_salt("Transaction account")
                                    .selected_text(format!("{}", self.transaction_account_string))
                                    .show_ui(ui, |ui| {
                                        for account_id in self.database.iter_account_ids() {
                                            if self.database.account(account_id).currency()
                                                == &self.transaction_currency
                                            {
                                                ui.selectable_value(
                                                    &mut self.transaction_account_id,
                                                    account_id,
                                                    format!(
                                                        "{:}",
                                                        self.database
                                                            .account(account_id)
                                                            .to_string()
                                                    ),
                                                );
                                            }
                                        }
                                    });
                            } else {
                                // it is not fund change
                                ui.label("Transaction entity:")
                                    .on_hover_text("Entity with whom the transaction is made.");
                                ComboBox::from_id_salt("Transaction entity")
                                    .selected_text(format!("{}", self.transaction_entity_string))
                                    .show_ui(ui, |ui| {
                                        for entity_id in self.database.iter_entity_ids() {
                                            ui.selectable_value(
                                                &mut self.transaction_entity_id,
                                                entity_id,
                                                format!(
                                                    "{:}",
                                                    self.database.entity(entity_id).to_string()
                                                ),
                                            );
                                        }
                                    });
                            };
                            ui.end_row();

                            ui.label("Transaction category:")
                                .on_hover_text("Category of the transaction.");
                            ui.add(
                                AutoCompleteTextEdit::new(
                                    &mut self.transaction_category,
                                    self.database.transaction_categories(&self.transaction_type),
                                )
                                .max_suggestions(10)
                                .highlight_matches(true),
                            );
                            ui.end_row();

                            ui.label("Transaction subcategory:")
                                .on_hover_text("Subcategory of the transaction.");
                            ui.add(
                                AutoCompleteTextEdit::new(
                                    &mut self.transaction_subcategory,
                                    self.database.transaction_subcategories(
                                        &self.transaction_type,
                                        self.transaction_category.clone(),
                                    ),
                                )
                                .max_suggestions(10)
                                .highlight_matches(true),
                            );
                            ui.end_row();

                            ui.label("Transaction description:")
                                .on_hover_text("Text description of the transaction.");
                            ui.text_edit_singleline(&mut self.transaction_description);
                            ui.end_row()
                        });

                    ui.separator();
                    ui.vertical_centered_justified(|ui| {
                        if self.are_valid_transaction_fields() {
                            self.transaction_value = self
                                .transaction_value_tentative
                                .parse::<f64>()
                                .expect("Error parsing transaction value");

                            let transaction: Transaction = match self.transaction_type {
                                TransactionType::Income => Transaction::Income {
                                    value: self.transaction_value,
                                    currency: self.transaction_currency.clone(),
                                    date: self.transaction_date,
                                    category: self.transaction_category.clone(),
                                    subcategory: self.transaction_subcategory.clone(),
                                    description: self.transaction_description.clone(),
                                    entity_id: self.transaction_entity_id,
                                },
                                TransactionType::Expense => Transaction::Expense {
                                    value: self.transaction_value,
                                    currency: self.transaction_currency.clone(),
                                    date: self.transaction_date,
                                    category: self.transaction_category.clone(),
                                    subcategory: self.transaction_subcategory.clone(),
                                    description: self.transaction_description.clone(),
                                    entity_id: self.transaction_entity_id,
                                },
                                TransactionType::Credit => Transaction::Credit {
                                    value: self.transaction_value,
                                    currency: self.transaction_currency.clone(),
                                    date: self.transaction_date,
                                    account_id: self.transaction_account_id,
                                },
                                TransactionType::Debit => Transaction::Debit {
                                    value: self.transaction_value,
                                    currency: self.transaction_currency.clone(),
                                    date: self.transaction_date,
                                    account_id: self.transaction_account_id,
                                },
                            };

                            if ui.button("Add transaction").clicked() {
                                self.party.add_transaction(transaction);
                                self.clear_transaction_fields();

                                self.show_input_transaction_window = false;
                            }
                        } else {
                            ui.label("Invalid transaction fields");
                        }
                    });
                });
                if ctx.input(|i| i.viewport().close_requested()) {
                    self.show_input_transaction_window = false;
                }
            },
        )
    }
}
