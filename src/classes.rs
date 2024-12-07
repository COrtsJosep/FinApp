pub mod financial {
	use chrono::prelude::*;
	use std::collections::HashMap;

	pub struct Party {
		pub transactions: Vec<Transaction>,
		pub id: u32
	}

	impl Party {
		pub(crate) fn is_valid(&self) -> bool {
			let mut aggregates: HashMap<&Currency, f32> = HashMap::new();

			for transaction in &self.transactions{
				let value: f32 = transaction.get_value() * transaction.get_sign();
				let currency: &Currency = transaction.get_currency();

				aggregates.entry(currency).and_modify(|aggregate: &mut f32| *aggregate += value).or_insert(value);
			}

			println!("Balance: {:?}", aggregates);
			for (key, val) in aggregates.iter() {
				if (*val).abs() >= 0.01 {return false} // one or more cents off
			}

			true
		}
	}

	pub enum Transaction {
		Earning {value: f32,
			currency: Currency,
			date: NaiveDate,
			category: String, // salary, interest
			subcategory: String, // regular salary, 13-month salary
			description: String,
			entity_id: u32},
		Expense {value: f32,
			currency: Currency,
			date: NaiveDate,
			category: String, // utilities, rent, transport
			subcategory: String, // train, bus, hairdresser
			description: String,
			entity_id: u32},
		Credit {value: f32,
			currency: Currency,
			date: NaiveDate,
			account_id: u32},
		Debit {value: f32,
			currency: Currency,
			date: NaiveDate,
			account_id: u32}
	}

	impl Transaction {
		fn get_value(&self) -> f32 {
			match self {
				Transaction::Earning {..} => 1.0,
				Transaction::Expense {..} => -1.0,
				Transaction::Credit  {..} => -1.0,
				Transaction::Debit {..} => 1.0,
			}
		}

		fn get_sign(&self) -> f32 {
			match self {
				Transaction::Earning { value, .. }
				| Transaction::Expense { value, .. }
				| Transaction::Credit { value, .. }
				| Transaction::Debit { value, .. } => *value,
			}
		}

		fn get_currency(&self) -> &Currency {
			match self {
				Transaction::Earning { currency, .. }
				| Transaction::Expense { currency, .. }
				| Transaction::Credit { currency, .. }
				| Transaction::Debit { currency, .. } => currency,
			}
		}
	}


	#[derive(Debug, Hash, PartialEq, Eq)]
	pub enum Currency {
		EUR,
		CHF,
		SEK
	}

	struct Entity {
		name: String,
		country: String,
		id: u32,
		entity_type: EntityType,
		subtype: String //supermarket, pharmacy, ... (?)
	}

	struct Account {
		name: String,
		country: String,
		id: u32,
		account_type: AccountType,
		initial_balance: f32
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
}