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

#[cfg(test)]
mod tests {
	use crate::classes::financial::*;
	use chrono::prelude::*;

	#[test]
	fn correct_party() {
		let t1 = Transaction::Expense {
			value: 200.0,
			currency: Currency::EUR,
			date: NaiveDate::from_ymd(2024, 12, 1),
			category: "Utilities".to_string(),
			subcategory: "Electricity".to_string(),
			description: "Monthly electricity bill".to_string(),
			entity_id: 1,
		};

		let t2 = Transaction::Expense {
			value: 100.0,
			currency: Currency::EUR,
			date: NaiveDate::from_ymd(2024, 12, 1),
			category: "Utilities".to_string(),
			subcategory: "Gas".to_string(),
			description: "Monthly gas bill".to_string(),
			entity_id: 1,
		};

		let t3 = Transaction::Debit {
			value: 300.0,
			currency: Currency::EUR,
			date: NaiveDate::from_ymd(2024, 12, 2),
			account_id: 42,
		};

		// Example data
		let items = vec![t1, t2, t3];

		let party: Party = Party {
			transactions: items,
			id: 4
		};

		assert!(party.is_valid());
	}

	#[test]
	fn incorrect_party() {
		let t1 = Transaction::Expense {
			value: 102.0,
			currency: Currency::EUR,
			date: NaiveDate::from_ymd(2024, 12, 1),
			category: "Utilities".to_string(),
			subcategory: "Electricity".to_string(),
			description: "Monthly electricity bill".to_string(),
			entity_id: 1,
		};

		let t2 = Transaction::Debit {
			value: 120.0,
			currency: Currency::EUR,
			date: NaiveDate::from_ymd(2024, 12, 2),
			account_id: 42,
		};

		// Example data
		let items = vec![t1, t2];

		let party: Party = Party {
			transactions: items,
			id: 4
		};

		assert!(!party.is_valid());
	}

	#[test]
	fn multicurrency_party() {
		let t1 = Transaction::Earning {
			value: 120.0,
			currency: Currency::EUR,
			date: NaiveDate::from_ymd(2024, 12, 1),
			category: "Salary".to_string(),
			subcategory: "Regular salary".to_string(),
			description: "Finally got the bread".to_string(),
			entity_id: 1,
		};

		let t2 = Transaction::Expense {
			value: 100.0,
			currency: Currency::SEK,
			date: NaiveDate::from_ymd(2024, 12, 1),
			category: "Drugs".to_string(),
			subcategory: "Alcohol".to_string(),
			description: "Bought some beers to celebrate".to_string(),
			entity_id: 11,
		};

		let t3 = Transaction::Credit {
			value: 120.0,
			currency: Currency::EUR,
			date: NaiveDate::from_ymd(2024, 12, 2),
			account_id: 2,
		};

		let t4 = Transaction::Debit {
			value: 100.0,
			currency: Currency::SEK,
			date: NaiveDate::from_ymd(2024, 12, 2),
			account_id: 42,
		};

		// Example data
		let items = vec![t1, t2, t3, t4];

		let party: Party = Party {
			transactions: items,
			id: 4
		};

		assert!(party.is_valid());
	}
}