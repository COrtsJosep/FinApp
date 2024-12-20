use chrono::{Local, NaiveDate};
use polars::prelude::*;
use super::financial::{Party, Transaction, Entity, Account};

pub struct IncomeTable {
    pub data_frame: DataFrame
}

impl IncomeTable {
    pub fn new() -> IncomeTable {
        let data_frame = DataFrame::new(vec![
            Column::from(Series::new(PlSmallStr::from("income_id"), Vec::<u32>::new())),
            Column::from(Series::new(PlSmallStr::from("value"), Vec::<f32>::new())),
            Column::from(Series::new(PlSmallStr::from("currency"), Vec::<String>::new())),
            Column::from(Series::new(PlSmallStr::from("date"), Vec::<NaiveDate>::new())),
            Column::from(Series::new(PlSmallStr::from("category"), Vec::<String>::new())),
            Column::from(Series::new(PlSmallStr::from("subcategory"), Vec::<String>::new())),
            Column::from(Series::new(PlSmallStr::from("description"), Vec::<String>::new())),
            Column::from(Series::new(PlSmallStr::from("entity_id"), Vec::<u32>::new())),
        ]).expect("Failed to initialize empty incomes table"); // considered unsafe. refactor?

        Self { data_frame }
    }

    pub fn load() -> IncomeTable {
        let data_frame = CsvReadOptions::default()
            .with_infer_schema_length(None)
            .with_has_header(true)
            .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
            .try_into_reader_with_file_path(Some("path/file.csv".into()))
            .expect("Failed to read incomes table") // considered unsafe. refactor?
            .finish()
            .expect("Failed to load incomes table");

        Self { data_frame }
    }

    fn get_last_income_id(&self) -> u32 {
        if self.data_frame.is_empty() { 0u32 }
        else {
            if let AnyValue::UInt32(id) = self.data_frame.column("income_id").expect("Failed to find income_id column").max_reduce().expect("Failed to generate id").value() { id + 1u32 }
            else {panic!("Failed to create an integer id")}
        }
    }

    pub fn add_record(&mut self, transaction: &Transaction) -> () {
        if let Transaction::Income {
            value,
            currency,
            date,
            category,
            subcategory,
            description,
            entity_id
        } = transaction {
            let income_id: u32 = self.get_last_income_id();

            let record = df!(
                "income_id" => [income_id],
                "value" => [*value],
                "currency" => [currency.to_string()],
                "date" => [*date],
                "category" => [category.to_string()],
                "subcategory" => [subcategory.to_string()],
                "description" => [description.to_string()],
                "entity_id" => [*entity_id],
            ).expect("Failed to create income record");

            self.data_frame = self.data_frame.vstack(&record).expect("Failed to insert income record")
        } else {
            panic!("Attempted to insert non-income into the incomes table");
        }

    }

    pub fn display(&self) {
        println!("{}", self.data_frame);
    }
}

pub struct ExpensesTable {
    pub data_frame: DataFrame
}

impl ExpensesTable {
    pub fn new() -> ExpensesTable {
        let data_frame = DataFrame::new(vec![
            Column::from(Series::new(PlSmallStr::from("expense_id"), Vec::<u32>::new())),
            Column::from(Series::new(PlSmallStr::from("value"), Vec::<f32>::new())),
            Column::from(Series::new(PlSmallStr::from("currency"), Vec::<String>::new())),
            Column::from(Series::new(PlSmallStr::from("date"), Vec::<NaiveDate>::new())),
            Column::from(Series::new(PlSmallStr::from("category"), Vec::<String>::new())),
            Column::from(Series::new(PlSmallStr::from("subcategory"), Vec::<String>::new())),
            Column::from(Series::new(PlSmallStr::from("description"), Vec::<String>::new())),
            Column::from(Series::new(PlSmallStr::from("entity_id"), Vec::<u32>::new())),
        ]).expect("Failed to initialize empty expenses table"); // considered unsafe. refactor?

        Self { data_frame }
    }

    pub fn load() -> ExpensesTable {
        let data_frame = CsvReadOptions::default()
            .with_infer_schema_length(None)
            .with_has_header(true)
            .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
            .try_into_reader_with_file_path(Some("path/file.csv".into()))
            .expect("Failed to read expenses table") // considered unsafe. refactor?
            .finish()
            .expect("Failed to load expenses table");

        Self { data_frame }
    }

    fn get_last_expense_id(&self) -> u32 {
        if self.data_frame.is_empty() { 0u32 }
        else {
            if let AnyValue::UInt32(id) = self.data_frame.column("expense_id").expect("Failed to find expense_id column").max_reduce().expect("Failed to generate id").value() { id + 1u32 }
            else {panic!("Failed to create an integer id")}
        }
    }

    pub fn add_record(&mut self, transaction: &Transaction) -> () {
        if let Transaction::Expense {
            value,
            currency,
            date,
            category,
            subcategory,
            description,
            entity_id
        } = transaction {
            let expense_id: u32 = self.get_last_expense_id();

            let record = df!(
                "expense_id" => [expense_id],
                "value" => [*value],
                "currency" => [currency.to_string()],
                "date" => [*date],
                "category" => [category.to_string()],
                "subcategory" => [subcategory.to_string()],
                "description" => [description.to_string()],
                "entity_id" => [*entity_id],
            ).expect("Failed to create expense record");

            self.data_frame = self.data_frame.vstack(&record).expect("Failed to insert expense record")
        } else {
            panic!("Attempted to insert non-expense into the expenses table");
        }

    }

    pub fn display(&self) {
        println!("{}", self.data_frame);
    }
}

pub struct FundsTable {
    pub data_frame: DataFrame
}

impl FundsTable {
    pub fn new() -> FundsTable {
        let data_frame = DataFrame::new(vec![
            Column::from(Series::new(PlSmallStr::from("fund_movement_id"), Vec::<u32>::new())),
            Column::from(Series::new(PlSmallStr::from("fund_movement_type"), Vec::<String>::new())),
            Column::from(Series::new(PlSmallStr::from("value"), Vec::<f32>::new())),
            Column::from(Series::new(PlSmallStr::from("currency"), Vec::<String>::new())),
            Column::from(Series::new(PlSmallStr::from("date"), Vec::<NaiveDate>::new())),
            Column::from(Series::new(PlSmallStr::from("account_id"), Vec::<u32>::new())),
        ]).expect("Failed to initialize empty funds table"); // considered unsafe. refactor?

        Self { data_frame }
    }

    pub fn load() -> FundsTable {
        let data_frame = CsvReadOptions::default()
            .with_infer_schema_length(None)
            .with_has_header(true)
            .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
            .try_into_reader_with_file_path(Some("path/file.csv".into()))
            .expect("Failed to read funds table") // considered unsafe. refactor?
            .finish()
            .expect("Failed to load funds table");

        Self { data_frame }
    }

    fn get_last_fund_movement_id(&self) -> u32 {
        if self.data_frame.is_empty() { 0u32 }
        else {
            if let AnyValue::UInt32(id) = self.data_frame.column("fund_movement_id").expect("Failed to find fund_movement_id column").max_reduce().expect("Failed to generate id").value() { id + 1u32 }
            else {panic!("Failed to create an integer id")}
        }
    }

    pub fn add_record(&mut self, transaction: &Transaction) -> () {
        let fund_movement_id: u32 = self.get_last_fund_movement_id();

        if let Transaction::Credit {
            value,
            currency,
            date,
            account_id
        } = transaction {
            let record = df!(
                "fund_movement_id" => [fund_movement_id],
                "fund_movement_type" => ["Credit"], // very bad solution IMO
                "value" => [*value],
                "currency" => [currency.to_string()],
                "date" => [*date],
                "account_id" => [*account_id],
            ).expect("Failed to create credit record");

            self.data_frame = self.data_frame.vstack(&record).expect("Failed to insert credit record")
        } else if let Transaction::Debit {
            value,
            currency,
            date,
            account_id
        } = transaction {
            let record = df!(
                "fund_movement_id" => [fund_movement_id],
                "fund_movement_type" => ["Debit"], // awful solution IMO
                "value" => [-1.0 * (*value)],
                "currency" => [currency.to_string()],
                "date" => [*date],
                "account_id" => [*account_id],
            ).expect("Failed to create debit record");

            self.data_frame = self.data_frame.vstack(&record).expect("Failed to insert debit record")
        } else {
            panic!("Attempted to insert non-income into the incomes table");
        }
    }

    pub fn display(&self) {
        println!("{}", self.data_frame);
    }
}

pub struct PartyTable {
    pub data_frame: DataFrame
}

impl PartyTable {
    pub fn new() -> PartyTable {
        let data_frame = DataFrame::new(vec![
            Column::from(Series::new(PlSmallStr::from("party_id"), Vec::<u32>::new())),
            Column::from(Series::new(PlSmallStr::from("creation_date"), Vec::<NaiveDate>::new()))
        ]).expect("Failed to initialize empty party table"); // considered unsafe. refactor?

        Self { data_frame }
    }

    pub fn load() -> PartyTable {
        let data_frame = CsvReadOptions::default()
            .with_infer_schema_length(None)
            .with_has_header(true)
            .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
            .try_into_reader_with_file_path(Some("path/file.csv".into()))
            .expect("Failed to read party table") // considered unsafe. refactor?
            .finish()
            .expect("Failed to load party table");

        Self { data_frame }
    }

    pub fn get_last_party_id(&self) -> u32 {
        if self.data_frame.is_empty() { 0u32 }
        else {
            if let AnyValue::UInt32(id) = self.data_frame.column("party_id").expect("Failed to find party_id column").max_reduce().expect("Failed to generate id").value() { id + 1u32 }
            else {panic!("Failed to create an integer id")}
        }
    }

    pub fn add_record(&mut self, party: &Party) -> () {
        let party_id: u32 = self.get_last_party_id();

        let record = df!(
            "party_id" => [party_id],
            "creation_date" => [party.creation_date]
        ).expect("Failed to create party record");

        self.data_frame = self.data_frame.vstack(&record).expect("Failed to insert party record")
    }

    pub fn display(&self) {
        println!("{}", self.data_frame);
    }
}

pub struct EntityTable {
    pub data_frame: DataFrame
}

impl EntityTable {
    pub fn new() -> EntityTable {
        let data_frame = DataFrame::new(vec![
            Column::from(Series::new(PlSmallStr::from("entity_id"), Vec::<u32>::new())),
            Column::from(Series::new(PlSmallStr::from("name"), Vec::<String>::new())),
            Column::from(Series::new(PlSmallStr::from("country"), Vec::<String>::new())),
            Column::from(Series::new(PlSmallStr::from("entity_type"), Vec::<String>::new())),
            Column::from(Series::new(PlSmallStr::from("entity_subtype"), Vec::<String>::new())),
            Column::from(Series::new(PlSmallStr::from("creation_date"), Vec::<NaiveDate>::new()))
        ]).expect("Failed to initialize empty party table"); // considered unsafe. refactor?

        Self { data_frame }
    }

    pub fn load() -> EntityTable {
        let data_frame = CsvReadOptions::default()
            .with_infer_schema_length(None)
            .with_has_header(true)
            .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
            .try_into_reader_with_file_path(Some("path/file.csv".into()))
            .expect("Failed to read entity table") // considered unsafe. refactor?
            .finish()
            .expect("Failed to load entity table");

        Self { data_frame }
    }

    fn get_last_entity_id(&self) -> u32 {
        if self.data_frame.is_empty() { 0u32 }
        else {
            if let AnyValue::UInt32(id) = self.data_frame.column("entity_id").expect("Failed to find entity_id column").max_reduce().expect("Failed to generate id").value() { id + 1u32 }
            else {panic!("Failed to create an integer id")}
        }
    }

    pub fn add_record(&mut self, entity: &Entity) -> () {
        let entity_id: u32 = self.get_last_entity_id();

        let record = df!(
            "entity_id" => [entity_id],
            "name" => [entity.get_name()],
            "country" => [entity.get_country()],
            "entity_type" => [entity.get_entity_type().to_string()],
            "entity_subtype" => [entity.get_entity_subtype()],
            "creation_date" => [Local::now().date_naive()]
        ).expect("Failed to create entity record");

        self.data_frame = self.data_frame.vstack(&record).expect("Failed to insert entity record")
    }

    pub fn display(&self) {
        println!("{}", self.data_frame);
    }
}
pub struct AccountTable {
    pub data_frame: DataFrame
}

impl AccountTable {
    pub fn new() -> AccountTable {
        let data_frame = DataFrame::new(vec![
            Column::from(Series::new(PlSmallStr::from("account_id"), Vec::<u32>::new())),
            Column::from(Series::new(PlSmallStr::from("name"), Vec::<String>::new())),
            Column::from(Series::new(PlSmallStr::from("country"), Vec::<String>::new())),
            Column::from(Series::new(PlSmallStr::from("currency"), Vec::<String>::new())),
            Column::from(Series::new(PlSmallStr::from("account_type"), Vec::<String>::new())),
            Column::from(Series::new(PlSmallStr::from("initial_balance"), Vec::<f32>::new())),
            Column::from(Series::new(PlSmallStr::from("creation_date"), Vec::<NaiveDate>::new()))
        ]).expect("Failed to initialize empty party table"); // considered unsafe. refactor?

        Self { data_frame }
    }

    pub fn load() -> AccountTable {
        let data_frame = CsvReadOptions::default()
            .with_infer_schema_length(None)
            .with_has_header(true)
            .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
            .try_into_reader_with_file_path(Some("path/file.csv".into()))
            .expect("Failed to read account table") // considered unsafe. refactor?
            .finish()
            .expect("Failed to load account table");

        Self { data_frame }
    }

    fn get_last_account_id(&self) -> u32 {
        if self.data_frame.is_empty() { 0u32 }
        else {
            if let AnyValue::UInt32(id) = self.data_frame.column("account_id").expect("Failed to find account_id column").max_reduce().expect("Failed to generate id").value() { id + 1u32 }
            else {panic!("Failed to create an integer id")}
        }
    }

    pub fn add_record(&mut self, account: &Account) -> () {
        let account_id: u32 = self.get_last_account_id();

        let record = df!(
            "account_id" => [account_id],
            "name" => [account.get_name()],
            "country" => [account.get_country()],
            "currency" => [account.get_currency().to_string()],
            "account_type" => [account.get_account_type().to_string()],
            "initial_balance" => [account.get_initial_balance()],
            "creation_date" => [Local::now().date_naive()]
        ).expect("Failed to create entity record");

        self.data_frame = self.data_frame.vstack(&record).expect("Failed to insert account record")
    }

    pub fn display(&self) {
        println!("{}", self.data_frame);
    }
}

