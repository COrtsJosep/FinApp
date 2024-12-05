pub mod financial {
	use chrono::prelude::*;
	use std::collections::HashMap;

	pub struct Party {
		pub financial_items: Vec<FinancialItem>,
		pub id: u32
	}

	impl Party {
		pub(crate) fn is_valid(&self) -> bool {
			let mut aggregates: HashMap<&Currency, f32> = HashMap::new();

			for financial_item in &self.financial_items{
				let key: &Currency = match &financial_item {
					FinancialItem::Transaction{ currency, .. } => {currency},
					FinancialItem::FundChange{ currency, .. } => {currency}
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
							ItemType::Expense => { *value },
							ItemType::Income => { -1.0 * value }
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

	pub enum FinancialItem {
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

	pub enum ItemType {
		Income,
		Expense
	}

	#[derive(Debug, Hash, PartialEq, Eq)]
	pub enum Currency {
		EUR,
		CHF,
		SEK
	}

	pub enum EntityType {
		Firm,
		Human,
		State,
		NGO
	}

	pub enum AccountType {
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
}