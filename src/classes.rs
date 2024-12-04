use chrono::prelude::*;
use std::collections::HashMap;

struct Party {
	financial_items: Vec<FinancialItem>,
	currency: Currency,
	id: u32
}

impl Party {
	fn is_valid(&self) -> bool {
		let mut aggregates: HashMap<(&Currency, String), f32> = HashMap::new();

		for financial_item in self.financial_items{
			let key: (&Currency, String) = match &financial_item {
				FinancialItem::Transaction{ currency, .. } => {(currency, "T".to_string())},
				FinancialItem::FundChange{ currency, .. } => {(currency, "F".to_string())}
			};
			let iteration_value: f32 = match &financial_item {
				FinancialItem::Transaction { value, item_type, ..} => {
					match item_type {
						ItemType::Expense => { -1.0 * value },
						ItemType::Income => { *value }
					}
				},
				FinancialItem::FundChange { value, item_type, ..} => {
					match item_type {
						ItemType::Expense => { -1.0 * value },
						ItemType::Income => { *value }
					}
				}
			};

			aggregates.entry(key).and_modify(|aggregate_value: &mut f32| *aggregate_value += iteration_value).or_insert(iteration_value);
		}

		for (key, val) in aggregates.iter() {
			if (*val).abs() >= 0.01 {return false} // one or more cents off
		}

		true
	}
}

enum FinancialItem {
	Transaction {value: f32,
	             currency: Currency,
	             date: NaiveDate,
	             item_type: ItemType,
	             category: String, // utilities, rent, salary, transport
	             subcategory: String, // train, bus, hairdresser
				 description: String,
	             entity_id: u32},
	FundChange {value: f32,
	            currency: Currency,
	            date: NaiveDate,
				item_type: ItemType,
	            account_id: u32}
}

enum ItemType {
	Income,
	Expense
}

enum Currency {
	EUR,
	CHF,
	SEK
}

enum EntityType {
	Firm,
	Human,
	State,
	NGO
}

enum AccountType {
	Deposit,
	Investment,
	Cash
}

struct Entity {
    name: String,
    country: String,
    id: u32,
    entity_type: EntityType,
    subcategory: String //supermarket, pharmacy, ... (?)
}

struct Account {
    name: String,
	country: String,
    id: u32,
    account_type: AccountType, // deposit/investing/pocket money,
    initial_balance: f32
}
