pub mod financial {
	use chrono::prelude::*;
	use std::collections::HashMap;
	use std::fmt::Display;

	/// A party is a balanced set of accounting transactions that happened together and that are
	/// related to each other.
	/// I know "party" is not the right word for that, but it's the literal translation from
	/// Spanish ("partida financiera"), I cannot think of a better name, and adds a festive
	/// touch to the code.
	pub struct Party {
		pub transactions: Vec<Transaction>,
		pub creation_date: NaiveDate
	}

	impl Party {
		/// Checks whether the party is balanced. This means that for every currency involved in
		/// the party, the amount expended/earned is associated with a decrease/increase of funds
		/// of the same amount and opposite sign.
		/// For instance, a bill of 50€ grocery shopping has to be balanced with, let's say, a 50€
		/// withdrawal from a bank account. The relationship does not need to be 1:1, for
		/// instance, a 350 SEK bill for clothing and a 230 SEK bill for presents can be balanced
		/// with a 500 SEK withdrawal from a bank account and an 80 SEK withdrawal from pocket money.
		pub(crate) fn is_valid(&self) -> bool {
			let mut aggregates: HashMap<&Currency, f32> = HashMap::new();

			for transaction in &self.transactions {
				let value: f32 = transaction.get_value() * transaction.get_sign();
				let currency: &Currency = transaction.get_currency();

				aggregates.entry(currency).and_modify(|aggregate: &mut f32| *aggregate += value).or_insert(value);
			}

			for (_, val) in aggregates.iter() {
				if (*val).abs() >= 0.01 {return false} // one or more cents off
			}

			true
		}

		/// Adds a new transaction to the party.
		pub(crate) fn add_transaction(&mut self, transaction: Transaction) -> () {
			self.transactions.push(transaction);
		}
	}

	/// Basic entity of the accounting system. Earnings and expenses reflect what event provoked
	/// the movement, credit and debit record what funds were used.
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
		/// Value getter.
		fn get_value(&self) -> f32 {
			match self {
				Transaction::Earning {..} => 1.0,
				Transaction::Expense {..} => -1.0,
				Transaction::Credit  {..} => -1.0,
				Transaction::Debit {..} => 1.0,
			}
		}

		/// Sign getter.
		fn get_sign(&self) -> f32 {
			match self {
				Transaction::Earning { value, .. }
				| Transaction::Expense { value, .. }
				| Transaction::Credit { value, .. }
				| Transaction::Debit { value, .. } => *value,
			}
		}

		/// Currency getter.
		fn get_currency(&self) -> &Currency {
			match self {
				Transaction::Earning { currency, .. }
				| Transaction::Expense { currency, .. }
				| Transaction::Credit { currency, .. }
				| Transaction::Debit { currency, .. } => currency,
			}
		}
	}

	/// Conversion to string
	impl Display for Transaction {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			let str = match self {
				Transaction::Earning { .. } => "Earning".to_string(),
				Transaction::Expense { .. } => "Expense".to_string(),
				Transaction::Credit { .. } => "Credit".to_string(),
				Transaction::Debit { .. } => "Debit".to_string()
			};
			write!(f, "{}", str)
		}
	}


	#[derive(Debug, Hash, PartialEq, Eq)]
	pub enum Currency {
		EUR,
		CHF,
		SEK
	}

	// Conversion to string
	impl Display for Currency {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			let str = match self {
				Currency::EUR => "EUR".to_string(),
				Currency::CHF => "CHF".to_string(),
				Currency::SEK => "SEK".to_string()
			};
			write!(f, "{}", str)
		}
	}

	/// Entity to which the expense is paid or, alternatively, that hands in the income.
	struct Entity {
		name: String,
		country: String,
		id: u32,
		entity_type: EntityType,
		subtype: String //supermarket, pharmacy, ... (?)
	}

	/// Account where funds are stored.
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
			date: NaiveDate::from_ymd_opt(2024, 12, 1).unwrap(),
			category: "Utilities".to_string(),
			subcategory: "Electricity".to_string(),
			description: "Monthly electricity bill".to_string(),
			entity_id: 1,
		};

		let t2 = Transaction::Expense {
			value: 100.0,
			currency: Currency::EUR,
			date: NaiveDate::from_ymd_opt(2024, 12, 1).unwrap(),
			category: "Utilities".to_string(),
			subcategory: "Gas".to_string(),
			description: "Monthly gas bill".to_string(),
			entity_id: 1,
		};

		let t3 = Transaction::Debit {
			value: 300.0,
			currency: Currency::EUR,
			date: NaiveDate::from_ymd_opt(2024, 12, 2).unwrap(),
			account_id: 42,
		};

		// Example data
		let items = vec![t1, t2, t3];

		let party: Party = Party {
			transactions: items,
			creation_date: NaiveDate::from_ymd_opt(2024, 12, 1).unwrap()
		};

		assert!(party.is_valid());
	}

	#[test]
	fn incorrect_party() {
		let t1 = Transaction::Expense {
			value: 102.0,
			currency: Currency::EUR,
			date: NaiveDate::from_ymd_opt(2024, 12, 1).unwrap(),
			category: "Utilities".to_string(),
			subcategory: "Electricity".to_string(),
			description: "Monthly electricity bill".to_string(),
			entity_id: 1,
		};

		let t2 = Transaction::Debit {
			value: 120.0,
			currency: Currency::EUR,
			date: NaiveDate::from_ymd_opt(2024, 12, 2).unwrap(),
			account_id: 42,
		};

		// Example data
		let items = vec![t1, t2];

		let party: Party = Party {
			transactions: items,
			creation_date: NaiveDate::from_ymd_opt(2024, 12, 1).unwrap(),
		};

		assert!(!party.is_valid());
	}

	#[test]
	fn multicurrency_party() {
		let t1 = Transaction::Earning {
			value: 120.0,
			currency: Currency::EUR,
			date: NaiveDate::from_ymd_opt(2024, 12, 1).unwrap(),
			category: "Salary".to_string(),
			subcategory: "Regular salary".to_string(),
			description: "Finally got the bread".to_string(),
			entity_id: 1,
		};

		let t2 = Transaction::Expense {
			value: 100.0,
			currency: Currency::SEK,
			date: NaiveDate::from_ymd_opt(2024, 12, 1).unwrap(),
			category: "Drugs".to_string(),
			subcategory: "Alcohol".to_string(),
			description: "Bought some beers to celebrate".to_string(),
			entity_id: 11,
		};

		let t3 = Transaction::Credit {
			value: 120.0,
			currency: Currency::EUR,
			date: NaiveDate::from_ymd_opt(2024, 12, 2).unwrap(),
			account_id: 2,
		};

		let t4 = Transaction::Debit {
			value: 100.0,
			currency: Currency::SEK,
			date: NaiveDate::from_ymd_opt(2024, 12, 2).unwrap(),
			account_id: 42,
		};

		// Example data
		let items = vec![t1, t2, t3, t4];

		let party: Party = Party {
			transactions: items,
			creation_date: NaiveDate::from_ymd_opt(2024, 12, 1).unwrap()
		};

		assert!(party.is_valid());
	}
}