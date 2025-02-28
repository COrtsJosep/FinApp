pub mod inputting;
pub mod plotting;
pub mod summarizing;

use crate::modules::database::*;
use crate::modules::financial::*;
use chrono::{Local, NaiveDate};
use derivative::*;
use eframe::egui;

const WINDOW_HEIGHT: f32 = 400.0;
const WINDOW_WIDTH: f32 = 600.0;

#[derive(Derivative)]
#[derivative(Default)]
pub struct AppState {
    show_input_window: bool,
    show_input_entity_window: bool,
    show_input_account_window: bool,
    show_input_party_window: bool,
    show_input_transaction_window: bool,
    show_plotting_window: bool,
    show_monthly_summary_window: bool,

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
    #[derivative(Default(value = "Local::now().date_naive()"))]
    transaction_date: NaiveDate,
    transaction_category: String,
    transaction_subcategory: String,
    transaction_description: String,
    transaction_entity_id: i64,
    transaction_entity_string: String,
    transaction_account_id: i64,
    transaction_account_string: String,
    transaction_type: TransactionType,

    monthly_summary: String,
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
            self.handle_show_monthly_summary_window(ctx)
            // todo!()
        }
    }
}