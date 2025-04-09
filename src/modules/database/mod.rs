pub mod plotter;
pub mod summaries;
pub mod views;

use crate::modules::financial::*;
use crate::modules::tables::*;
use polars::prelude::*;
use std::io::Cursor;
use std::vec::IntoIter;

fn data_frame_to_csv_string(data_frame: &mut DataFrame) -> String {
    let mut buffer = Cursor::new(Vec::new());

    CsvWriter::new(&mut buffer)
        .include_header(true)
        .finish(data_frame)
        .unwrap();

    String::from_utf8(buffer.into_inner())
        .unwrap()
        .replace(".0,", ".00,")
}

fn capitalize_every_word(sentence: String) -> String {
    // Copied and addapted to my needs from thirtyseconds
    // https://docs.rs/thirtyseconds/latest/thirtyseconds/strings/fn.capitalize_every_word.html
    sentence
        .as_str()
        .split(' ')
        .map(|word| format!("{}{}", &word[..1].to_uppercase(), &word[1..]))
        .collect::<Vec<_>>()
        .join(" ")
}

pub struct DataBase {
    incomes_table: IncomeTable,
    expenses_table: ExpensesTable,
    funds_table: FundsTable,
    party_table: PartyTable,
    entity_table: EntityTable,
    account_table: AccountTable,
}

impl DataBase {
    pub(crate) fn new() -> DataBase {
        let incomes_table = *IncomeTable::new();
        let expenses_table = *ExpensesTable::new();
        let funds_table = *FundsTable::new();
        let party_table = *PartyTable::new();
        let entity_table = *EntityTable::new();
        let account_table = *AccountTable::new();

        DataBase {
            incomes_table,
            expenses_table,
            funds_table,
            party_table,
            entity_table,
            account_table,
        }
    }

    pub fn init() -> DataBase {
        let incomes_table = *IncomeTable::init();
        let expenses_table = *ExpensesTable::init();
        let funds_table = *FundsTable::init();
        let party_table = *PartyTable::init();
        let entity_table = *EntityTable::init();
        let account_table = *AccountTable::init();

        DataBase {
            incomes_table,
            expenses_table,
            funds_table,
            party_table,
            entity_table,
            account_table,
        }
    }

    pub fn save(&mut self) -> () {
        self.incomes_table.save();
        self.expenses_table.save();
        self.funds_table.save();
        self.party_table.save();
        self.entity_table.save();
        self.account_table.save();
    }

    pub fn insert_party(&mut self, party: &mut Party) -> () {
        let party_id: i64 = self.party_table.next_id();
        for transaction in party.iter() {
            self.insert_transaction(&transaction, party_id);
        }

        self.party_table.insert_party(party);
    }

    fn insert_transaction(&mut self, transaction: &Transaction, party_id: i64) -> () {
        match transaction {
            Transaction::Expense { .. } => self
                .expenses_table
                .insert_transaction(transaction, party_id),
            Transaction::Income { .. } => {
                self.incomes_table.insert_transaction(transaction, party_id)
            }
            Transaction::Credit { .. } | Transaction::Debit { .. } => {
                self.funds_table.insert_transaction(transaction, party_id)
            }
        }
    }

    /// Returns the number of records in each table, for testing purposes
    pub(crate) fn size(&self) -> DataFrame {
        let data_frame: DataFrame = df!(
            "table" => ["income", "expenses", "funds", "party", "entity", "account"],
            "records" => [
                self.incomes_table.data_frame.height() as i64,
                self.expenses_table.data_frame.height() as i64,
                self.funds_table.data_frame.height() as i64,
                self.party_table.data_frame.height() as i64,
                self.entity_table.data_frame.height() as i64,
                self.account_table.data_frame.height() as i64
            ]
        )
        .unwrap();

        data_frame
    }

    pub fn insert_entity(&mut self, entity: &Entity) -> i64 {
        self.entity_table.insert_entity(entity)
    }

    pub fn insert_account(&mut self, account: &Account) -> i64 {
        self.account_table.insert_account(account)
    }

    pub(crate) fn iter_entity_ids(&mut self) -> IntoIter<i64> {
        self.entity_table.iter()
    }

    pub(crate) fn entity(&self, entity_id: i64) -> Entity {
        self.entity_table.entity(entity_id)
    }

    pub(crate) fn iter_account_ids(&mut self) -> IntoIter<i64> {
        self.account_table.iter()
    }

    pub(crate) fn entity_countries(&self) -> Vec<String> {
        self.entity_table.countries()
    }

    pub(crate) fn account(&self, account_id: i64) -> Account {
        self.account_table.account(account_id)
    }

    pub(crate) fn account_countries(&self) -> Vec<String> {
        self.account_table.countries()
    }

    pub(crate) fn transaction_categories(&self, transaction_type: &TransactionType) -> Vec<String> {
        match transaction_type {
            TransactionType::Income => self.incomes_table.categories(),
            TransactionType::Expense => self.expenses_table.categories(),
            _ => Vec::new(), // rethink whether it's the correct thing to do
        }
    }

    pub(crate) fn transaction_subcategories(
        &self,
        transaction_type: &TransactionType,
        category: String,
    ) -> Vec<String> {
        match transaction_type {
            TransactionType::Income => self.incomes_table.subcategories(category),
            TransactionType::Expense => self.expenses_table.subcategories(category),
            _ => Vec::new(),
        }
    }

    pub(crate) fn entity_subtypes(&self) -> Vec<String> {
        self.entity_table.subtypes()
    }

    /// Deletes from the database all records from the party.
    pub(crate) fn delete_party(&mut self, party_id: i64) -> () {
        self.incomes_table.delete_party(party_id);
        self.expenses_table.delete_party(party_id);
        self.funds_table.delete_party(party_id);
        self.party_table.delete_party(party_id);
    }
}

impl Default for DataBase {
    fn default() -> Self {
        Self::init()
    }
}
