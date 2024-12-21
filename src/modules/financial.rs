use chrono::prelude::*;
use std::collections::HashMap;
use std::fmt::Display;
use std::slice::Iter;

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

	pub(crate) fn iter(&mut self) -> Iter<'_, Transaction> {
		self.transactions.iter()
	}
}

/// Basic entity of the accounting system. Incomes and expenses reflect what event provoked
/// the movement, credit and debit record what funds were used.
pub enum Transaction {
	Income {value: f32,
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
			Transaction::Income {..} => 1.0,
			Transaction::Expense {..} => -1.0,
			Transaction::Credit  {..} => -1.0,
			Transaction::Debit {..} => 1.0,
		}
	}

	/// Sign getter.
	fn get_sign(&self) -> f32 {
		match self {
			Transaction::Income { value, .. }
			| Transaction::Expense { value, .. }
			| Transaction::Credit { value, .. }
			| Transaction::Debit { value, .. } => *value,
		}
	}

	/// Currency getter.
	fn get_currency(&self) -> &Currency {
		match self {
			Transaction::Income { currency, .. }
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
			Transaction::Income { .. } => "Income".to_string(),
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
pub struct Entity {
	name: String,
	country: String,
	entity_type: EntityType,
	entity_subtype: String //supermarket, pharmacy, ... (?)
}

impl Entity {
	pub(crate) fn get_name(&self) -> String { self.name.to_string() }
	pub(crate) fn get_country(&self) -> String { self.country.to_string() }
	pub (crate) fn get_entity_type(&self) -> &EntityType { &self.entity_type }
	pub (crate) fn get_entity_subtype(&self) -> String { self.entity_subtype.to_string() }

	pub fn new(name: String,
			   country: String,
			   entity_type: EntityType,
			   entity_subtype: String) -> Self {
		Self {
			name,
			country,
			entity_type,
			entity_subtype,
		}
	}
}

/// Account where funds are stored.
pub struct Account {
	name: String,
	country: String,
	currency: Currency,
	account_type: AccountType,
	initial_balance: f32
}

impl Account {
	pub(crate) fn get_name(&self) -> String { self.name.to_string() }
	pub(crate) fn get_country(&self) -> String { self.country.to_string() }
	pub(crate) fn get_currency(&self) -> &Currency { &self.currency }
	pub (crate) fn get_account_type(&self) -> &AccountType { &self.account_type }
	pub (crate) fn get_initial_balance(&self) -> f32 { self.initial_balance }

	pub fn new(name: String,
			   country: String,
			   currency: Currency,
			   account_type: AccountType,
			   initial_balance: f32) -> Self {
		Self {
			name,
			country,
			currency,
			account_type,
			initial_balance,
		}
	}
}

pub enum EntityType {
	Firm,
	Human,
	State,
	NGO
}

/// Conversion to string
impl Display for EntityType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let str = match self {
			EntityType::Firm { .. } => "Firm".to_string(),
			EntityType::Human { .. } => "Human".to_string(),
			EntityType::State { .. } => "State".to_string(),
			EntityType::NGO { .. } => "NGO".to_string()
		};
		write!(f, "{}", str)
	}
}

pub enum AccountType {
	Deposit,
	Investment,
	Cash
}

/// Conversion to string
impl Display for AccountType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let str = match self {
			AccountType::Deposit { .. } => "Deposit".to_string(),
			AccountType::Investment { .. } => "Investment".to_string(),
			AccountType::Cash { .. } => "Cash".to_string()
		};
		write!(f, "{}", str)
	}
}