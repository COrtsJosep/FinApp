use polars::prelude::*;
use crate::modules::tables::*;
use crate::modules::financial::*;

pub struct DataBase {
    incomes_table: IncomeTable,
    expenses_table: ExpensesTable,
    funds_table: FundsTable,
    party_table: PartyTable,
    entity_table: EntityTable,
    account_table: AccountTable
}

impl DataBase {
    pub(crate) fn new() -> DataBase {
        let incomes_table = IncomeTable::new();
        let expenses_table = ExpensesTable::new();
        let funds_table = FundsTable::new();
        let party_table = PartyTable::new();
        let entity_table = EntityTable::new();
        let account_table = AccountTable::new();

        DataBase {
            incomes_table,
            expenses_table,
            funds_table,
            party_table,
            entity_table,
            account_table
        }
    }
    pub(crate) fn insert_party(&mut self, party: &mut Party) -> () { // does party need to be mutable
        for transaction in party.iter() {
            self.insert_transaction(&transaction);
        }

        self.party_table.add_record(party);
    }

    fn insert_transaction(&mut self, transaction: &Transaction) -> () {
        match transaction {
            Transaction::Expense { .. } => self.expenses_table.add_record(transaction),
            Transaction::Income { .. } => self.incomes_table.add_record(transaction),
            Transaction::Credit { .. } | Transaction::Debit { .. } => self.funds_table.add_record(transaction)
        }
    }

    pub(crate) fn size(&self) -> DataFrame {
        let data_frame: DataFrame = df!(
            "table" => ["income", "expenses", "funds", "party", "entity", "account"],
            "records" => [
                self.incomes_table.data_frame.height() as u32,
                self.expenses_table.data_frame.height() as u32,
                self.funds_table.data_frame.height() as u32,
                self.party_table.data_frame.height() as u32,
                self.entity_table.data_frame.height() as u32,
                self.account_table.data_frame.height() as u32
            ]
        ).unwrap();

        data_frame
    }

    pub(crate) fn insert_entity(&mut self, entity: &Entity) -> () {
        self.entity_table.add_record(entity);
    }

    pub(crate) fn insert_account(&mut self, account: &Account) -> () {
        self.account_table.add_record(account);
    }
}
