pub mod browsing;
pub mod inputting;
pub mod plotting;
pub mod summarizing;

use crate::modules::database::*;
use crate::modules::financial::*;
use chrono::{Local, NaiveDate};
use derivative::*;
use eframe::egui;
use egui::PopupCloseBehavior;

const WINDOW_HEIGHT: f32 = 400.0;
const WINDOW_WIDTH: f32 = 600.0;

#[derive(Derivative)]
#[derivative(Default)]
pub struct AppState {
    show_input_entity_window: bool,
    show_input_account_window: bool,
    show_input_party_window: bool,
    show_input_transaction_window: bool,
    show_expense_summary_window: bool,
    show_fund_stand_window: bool,
    show_browse_window: bool,

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
    transaction_filter: String,
    #[derivative(Default(value = "PopupCloseBehavior::IgnoreClicks"))]
    transaction_entity_popup: PopupCloseBehavior,

    expense_summary_csv: String,
    #[derivative(Default(value = "Local::now().date_naive()"))]
    expense_summary_date_from: NaiveDate,
    #[derivative(Default(value = "Local::now().date_naive()"))]
    expense_summary_date_to: NaiveDate,
    expense_summary_currency: Currency,

    fund_stand_csv: String,
    fund_stand_currency: Option<Currency>,

    last_transactions_csv: String,
    last_transactions_n: usize,
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) -> () {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Welcome to you personal finances app!");
            if ui.button("Add transactions").clicked() {
                self.show_input_party_window = true;
            };

            ui.menu_button("Summaries", |ui| {
                if ui.button("Expenses by Category").clicked() {
                    self.show_expense_summary_window = true;
                }
                if ui.button("Funds by Account").clicked() {
                    self.show_fund_stand_window = true;
                }
            });

            if ui.button("Plotting").clicked() {
                self.database.last_transactions(10);
                todo!();
            };

            if ui.button("Browsing").clicked() {
                self.show_browse_window = true;
            }
        });

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

        if self.show_expense_summary_window {
            self.handle_show_expense_summary_window(ctx)
        }

        if self.show_fund_stand_window {
            self.handle_show_fund_stand_window(ctx)
        }

        if self.show_browse_window {
            self.handle_show_browse_window(ctx)
        }
    }
}
