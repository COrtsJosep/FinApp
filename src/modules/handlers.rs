use derivative::*;
use crate::modules::database::*;
use crate::modules::financial::*;
use chrono::{Local, NaiveDate};
use eframe::egui;
use eframe::egui::ComboBox;
use egui_extras::*;
use strum::IntoEnumIterator;
use egui_autocomplete::AutoCompleteTextEdit;

const WINDOW_WIDTH: f32 = 600.0;
const WINDOW_HEIGHT: f32 = 400.0;

#[derive(Derivative)]
#[derivative(Default)]
pub struct AppState {
    show_input_window: bool,
    show_input_entity_window: bool,
    show_input_account_window: bool,
    show_input_party_window: bool,
    show_input_transaction_window: bool,
    show_plotting_window: bool,

    database: DataBase,

    entity_name: String,
    entity_country: String,
    entity_type: EntityType,
    entity_subtype: String,
    
    account_name: String,
    account_country: String,
    account_currency: Currency,
    account_type: AccountType,
    account_initial_balance: f64,
    account_initial_balance_tentative: String,

    party: Party,

    transaction_value: f64,
    transaction_value_tentative: String,
    transaction_currency: Currency,
    #[derivative(Default(value="Local::now().date_naive()"))]
    transaction_date: NaiveDate,
    transaction_category: String,
    transaction_subcategory: String,
    transaction_description: String,
    transaction_entity_id: i64,
    transaction_entity_string: String,
    transaction_account_id: i64,
    transaction_account_string: String,
    transaction_type: TransactionType,
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

        if self.show_input_account_window {
            self.handle_show_input_account_window(ctx);
        }

        if self.show_input_party_window {
            self.handle_show_input_party_window(ctx);
        }

        if self.show_input_transaction_window {
            self.handle_show_input_transaction_window(ctx)
        }

        if self.show_plotting_window {
            // todo
        }
    }
}

impl AppState {
    fn clear_fields(&mut self) -> () {
        *self = AppState::default();
    }

    fn clear_transaction_fields(&mut self) -> () {
        self.transaction_value = f64::default();
        self.transaction_value_tentative = String::default();
        self.transaction_currency = Currency::default();
        self.transaction_date = Local::now().date_naive();
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

    fn are_valid_account_fields(&self) -> bool {
        let parsing_result = self.account_initial_balance_tentative.parse::<f64>();
        let valid_initial_balance: bool = match parsing_result {
            Ok(_value) => true,
            Err(_e) => false,
        };

        (self.account_name.len() > 0) & (self.account_country.len() > 0) & valid_initial_balance
    }

    fn are_valid_transaction_fields(&self) -> bool {
        let parsing_result = self.transaction_value_tentative.parse::<f64>();
        let valid_transaction_value: bool = match parsing_result {
            Ok(_value) => true,
            Err(_e) => false,
        };

        ((self.transaction_category.len() > 0) | self.transaction_type.is_fund_change())
            & valid_transaction_value
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
                    ui.label("Input window");
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
                    ui.label("Input window");
                    ui.horizontal(|ui| {
                        let entity_name_label = ui.label("Entity name: ");
                        ui.text_edit_singleline(&mut self.entity_name)
                            .labelled_by(entity_name_label.id);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Entity country: ");
                        ui.add(
                            AutoCompleteTextEdit::new(
                                &mut self.entity_country, 
                                self.database.entity_countries(),
                            )
                                .max_suggestions(10)
                                .highlight_matches(true),
                        );
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

                    //ui.horizontal(|ui| {
                    //    let entity_subtype_label = ui.label("Entity subtype: ");
                    //    ui.text_edit_singleline(&mut self.entity_subtype)
                    //        .labelled_by(entity_subtype_label.id);
                    //});

                    ui.horizontal(|ui| {
                        ui.label("Entity subtype: ");
                        ui.add(
                            AutoCompleteTextEdit::new(
                                &mut self.entity_subtype,
                                self.database.entity_subtypes(),
                            )
                                .max_suggestions(10)
                                .highlight_matches(true),
                        );
                    });

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
                    } else {
                        ui.label("Invalid fields");
                    }
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
                    ui.label("Input window");
                    ui.horizontal(|ui| {
                        let account_name_label = ui.label("Account name: ");
                        ui.text_edit_singleline(&mut self.account_name)
                            .labelled_by(account_name_label.id);
                    });
                    //ui.horizontal(|ui| {
                    //    let account_country_label = ui.label("Account country: ");
                    //    ui.text_edit_singleline(&mut self.account_country)
                    //        .labelled_by(account_country_label.id);
                    //});

                    ui.horizontal(|ui| {
                        ui.label("Account country: ");
                        ui.add(
                            AutoCompleteTextEdit::new(
                                &mut self.account_country,
                                self.database.account_countries(),
                            )
                                .max_suggestions(10)
                                .highlight_matches(true),
                        );
                    });

                    ComboBox::from_label("Account currency")
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

                    ComboBox::from_label("Account type")
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

                    ui.horizontal(|ui| {
                        let account_initial_balance_label = ui.label("Account initial balance: ");
                        ui.text_edit_singleline(&mut self.account_initial_balance_tentative)
                            .labelled_by(account_initial_balance_label.id);
                    });

                    if self.are_valid_account_fields() {
                        self.account_initial_balance = self
                            .account_initial_balance_tentative
                            .parse::<f64>()
                            .expect("Error parsing account initial balance");

                        if ui.button("Add new account").clicked() {
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
                .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
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
        self.transaction_entity_string = self
            .database
            .entity(self.transaction_entity_id)
            .to_string();
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
                    ui.label("Input window");
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

                    ui.horizontal(|ui| {
                        let transaction_value_label = ui.label("Transaction value: ");
                        ui.text_edit_singleline(&mut self.transaction_value_tentative)
                            .labelled_by(transaction_value_label.id);
                    });

                    ComboBox::from_label("Transaction currency")
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

                    ui.horizontal(|ui| {
                        ui.add(DatePickerButton::new(&mut self.transaction_date));
                        if ui.button("Today").clicked() {
                            self.transaction_date = Local::now().date_naive();
                        }
                        ui.end_row();
                    });

                    if self.transaction_type.is_fund_change() {
                        ComboBox::from_label("Transaction account")
                            .selected_text(format!("{}", self.transaction_account_string))
                            .show_ui(ui, |ui| {
                                for account_id in self.database.iter_account_ids() {
                                    ui.selectable_value(
                                        &mut self.transaction_account_id,
                                        account_id,
                                        format!(
                                            "{:}",
                                            self.database.account(account_id).to_string()
                                        ),
                                    );
                                }
                            });
                    } else {
                        // it is not fund change
                        ComboBox::from_label("Transaction entity")
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

                        /*
                        ui.horizontal(|ui| {
                            let transaction_category_label = ui.label("Transaction category: ");
                            ui.text_edit_singleline(&mut self.transaction_category)
                                .labelled_by(transaction_category_label.id);
                        });
                        ui.horizontal(|ui| {
                            let transaction_subcategory_label =
                                ui.label("Transaction subcategory: ");
                            ui.text_edit_singleline(&mut self.transaction_subcategory)
                                .labelled_by(transaction_subcategory_label.id);
                        });
                        */

                        ui.horizontal(|ui| {
                            ui.label("Transaction category: ");
                            ui.add(
                                AutoCompleteTextEdit::new(
                                    &mut self.transaction_category,
                                    self.database.transaction_categories(&self.transaction_type),
                                )
                                    .max_suggestions(10)
                                    .highlight_matches(true),
                            );
                        });

                        ui.horizontal(|ui| {
                            ui.label("Transaction subcategory: ");
                            ui.add(
                                AutoCompleteTextEdit::new(
                                    &mut self.transaction_subcategory,
                                    self.database.transaction_subcategories(&self.transaction_type, self.transaction_category.clone()),
                                )
                                    .max_suggestions(10)
                                    .highlight_matches(true),
                            );
                        });

                        ui.horizontal(|ui| {
                            let transaction_description_label =
                                ui.label("Transaction description: ");
                            ui.text_edit_singleline(&mut self.transaction_description)
                                .labelled_by(transaction_description_label.id);
                        });
                    }

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
                if ctx.input(|i| i.viewport().close_requested()) {
                    self.show_input_transaction_window = false;
                }
            },
        );
    }
}
