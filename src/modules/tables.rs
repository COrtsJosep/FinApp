use super::financial::{Account, Entity, EntityType, Party, Transaction};
use chrono::{Local, NaiveDate};
use polars::prelude::*;
use std::fs::File;
use std::str::FromStr;

pub struct IncomeTable {
    pub data_frame: DataFrame,
}

impl IncomeTable {
    pub fn new() -> IncomeTable {
        let data_frame = DataFrame::new(vec![
            Column::from(Series::new(
                PlSmallStr::from("income_id"),
                Vec::<i64>::new(),
            )),
            Column::from(Series::new(PlSmallStr::from("value"), Vec::<f64>::new())),
            Column::from(Series::new(
                PlSmallStr::from("currency"),
                Vec::<String>::new(),
            )),
            Column::from(Series::new(
                PlSmallStr::from("date"),
                Vec::<NaiveDate>::new(),
            )),
            Column::from(Series::new(
                PlSmallStr::from("category"),
                Vec::<String>::new(),
            )),
            Column::from(Series::new(
                PlSmallStr::from("subcategory"),
                Vec::<String>::new(),
            )),
            Column::from(Series::new(
                PlSmallStr::from("description"),
                Vec::<String>::new(),
            )),
            Column::from(Series::new(
                PlSmallStr::from("entity_id"),
                Vec::<i64>::new(),
            )),
        ])
        .expect("Failed to initialize empty income table"); // considered unsafe. refactor?

        Self { data_frame }
    }

    pub fn try_load() -> Result<IncomeTable, String> {
        CsvReadOptions::default()
            .with_infer_schema_length(None)
            .with_has_header(true)
            .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
            .try_into_reader_with_file_path(Some("data/income_table.csv".into()))
            .map_err(|e| format!("Failed to read income table: {}", e))?
            .finish()
            .map_err(|e| format!("Failed to load income table: {}", e))
            .map(|data_frame| Self { data_frame })
    }

    pub fn save(&mut self) -> () {
        if self.data_frame.is_empty() {
            return;
        }

        let mut file =
            File::create("data/income_table.csv").expect("Could not create file income_table.csv");
        CsvWriter::new(&mut file)
            .include_header(true)
            .with_separator(b',')
            .finish(&mut self.data_frame)
            .expect("Failed to save income table.");
    }

    pub fn init() -> IncomeTable {
        IncomeTable::try_load().unwrap_or_else(|e| IncomeTable::new())
    }

    fn get_last_income_id(&self) -> i64 {
        if self.data_frame.is_empty() {
            0i64
        } else {
            if let AnyValue::Int64(id) = self
                .data_frame
                .column("income_id")
                .expect("Failed to find income_id column")
                .max_reduce()
                .expect("Failed to generate id")
                .value()
            {
                id + 1i64
            } else {
                panic!("Failed to create an integer id")
            }
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
            entity_id,
        } = transaction
        {
            let income_id: i64 = self.get_last_income_id();

            let record = df!(
                "income_id" => [income_id],
                "value" => [*value],
                "currency" => [currency.to_string()],
                "date" => [*date],
                "category" => [category.to_string()],
                "subcategory" => [subcategory.to_string()],
                "description" => [description.to_string()],
                "entity_id" => [*entity_id],
            )
            .expect("Failed to create income record");

            self.data_frame = self
                .data_frame
                .vstack(&record)
                .expect("Failed to insert income record")
        } else {
            panic!("Attempted to insert non-income into the income table");
        }
    }

    pub fn display(&self) {
        println!("{}", self.data_frame);
    }
}

pub struct ExpensesTable {
    pub data_frame: DataFrame,
}

impl ExpensesTable {
    pub fn new() -> ExpensesTable {
        let data_frame = DataFrame::new(vec![
            Column::from(Series::new(
                PlSmallStr::from("expense_id"),
                Vec::<i64>::new(),
            )),
            Column::from(Series::new(PlSmallStr::from("value"), Vec::<f64>::new())),
            Column::from(Series::new(
                PlSmallStr::from("currency"),
                Vec::<String>::new(),
            )),
            Column::from(Series::new(
                PlSmallStr::from("date"),
                Vec::<NaiveDate>::new(),
            )),
            Column::from(Series::new(
                PlSmallStr::from("category"),
                Vec::<String>::new(),
            )),
            Column::from(Series::new(
                PlSmallStr::from("subcategory"),
                Vec::<String>::new(),
            )),
            Column::from(Series::new(
                PlSmallStr::from("description"),
                Vec::<String>::new(),
            )),
            Column::from(Series::new(
                PlSmallStr::from("entity_id"),
                Vec::<i64>::new(),
            )),
        ])
        .expect("Failed to initialize empty expenses table"); // considered unsafe. refactor?

        Self { data_frame }
    }

    pub fn try_load() -> Result<ExpensesTable, String> {
        CsvReadOptions::default()
            .with_infer_schema_length(None)
            .with_has_header(true)
            .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
            .try_into_reader_with_file_path(Some("data/expenses_table.csv".into()))
            .map_err(|e| format!("Failed to read expenses table: {}", e))?
            .finish()
            .map_err(|e| format!("Failed to load expenses table: {}", e))
            .map(|data_frame| Self { data_frame })
    }

    pub fn save(&mut self) -> () {
        if self.data_frame.is_empty() {
            return;
        }

        let mut file = File::create("data/expenses_table.csv")
            .expect("Could not create file expenses_table.csv");
        CsvWriter::new(&mut file)
            .include_header(true)
            .with_separator(b',')
            .finish(&mut self.data_frame)
            .expect("Failed to save expenses table.");
    }

    pub fn init() -> ExpensesTable {
        ExpensesTable::try_load().unwrap_or_else(|e| ExpensesTable::new())
    }

    fn get_last_expense_id(&self) -> i64 {
        if self.data_frame.is_empty() {
            0i64
        } else {
            if let AnyValue::Int64(id) = self
                .data_frame
                .column("expense_id")
                .expect("Failed to find expense_id column")
                .max_reduce()
                .expect("Failed to generate id")
                .value()
            {
                id + 1i64
            } else {
                panic!("Failed to create an integer id")
            }
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
            entity_id,
        } = transaction
        {
            let expense_id: i64 = self.get_last_expense_id();

            let record = df!(
                "expense_id" => [expense_id],
                "value" => [*value],
                "currency" => [currency.to_string()],
                "date" => [*date],
                "category" => [category.to_string()],
                "subcategory" => [subcategory.to_string()],
                "description" => [description.to_string()],
                "entity_id" => [*entity_id],
            )
            .expect("Failed to create expense record");

            self.data_frame = self
                .data_frame
                .vstack(&record)
                .expect("Failed to insert expense record")
        } else {
            panic!("Attempted to insert non-expense into the expenses table");
        }
    }

    pub fn display(&self) {
        println!("{}", self.data_frame);
    }
}

pub struct FundsTable {
    pub data_frame: DataFrame,
}

impl FundsTable {
    pub fn new() -> FundsTable {
        let data_frame = DataFrame::new(vec![
            Column::from(Series::new(
                PlSmallStr::from("fund_movement_id"),
                Vec::<i64>::new(),
            )),
            Column::from(Series::new(
                PlSmallStr::from("fund_movement_type"),
                Vec::<String>::new(),
            )),
            Column::from(Series::new(PlSmallStr::from("value"), Vec::<f64>::new())),
            Column::from(Series::new(
                PlSmallStr::from("currency"),
                Vec::<String>::new(),
            )),
            Column::from(Series::new(
                PlSmallStr::from("date"),
                Vec::<NaiveDate>::new(),
            )),
            Column::from(Series::new(
                PlSmallStr::from("account_id"),
                Vec::<i64>::new(),
            )),
        ])
        .expect("Failed to initialize empty funds table"); // considered unsafe. refactor?

        Self { data_frame }
    }

    pub fn try_load() -> Result<FundsTable, String> {
        CsvReadOptions::default()
            .with_infer_schema_length(None)
            .with_has_header(true)
            .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
            .try_into_reader_with_file_path(Some("data/funds_table.csv".into()))
            .map_err(|e| format!("Failed to read funds table: {}", e))?
            .finish()
            .map_err(|e| format!("Failed to load funds table: {}", e))
            .map(|data_frame| Self { data_frame })
    }

    pub fn save(&mut self) -> () {
        if self.data_frame.is_empty() {
            return;
        }

        let mut file =
            File::create("data/funds_table.csv").expect("Could not create file funds_table.csv");
        CsvWriter::new(&mut file)
            .include_header(true)
            .with_separator(b',')
            .finish(&mut self.data_frame)
            .expect("Failed to save funds table.");
    }

    pub fn init() -> FundsTable {
        FundsTable::try_load().unwrap_or_else(|e| FundsTable::new())
    }

    fn get_last_fund_movement_id(&self) -> i64 {
        if self.data_frame.is_empty() {
            0i64
        } else {
            if let AnyValue::Int64(id) = self
                .data_frame
                .column("fund_movement_id")
                .expect("Failed to find fund_movement_id column")
                .max_reduce()
                .expect("Failed to generate id")
                .value()
            {
                id + 1i64
            } else {
                panic!("Failed to create an integer id")
            }
        }
    }

    pub fn add_record(&mut self, transaction: &Transaction) -> () {
        let fund_movement_id: i64 = self.get_last_fund_movement_id();

        if let Transaction::Credit {
            value,
            currency,
            date,
            account_id,
        } = transaction
        {
            let record = df!(
                "fund_movement_id" => [fund_movement_id],
                "fund_movement_type" => ["Credit"], // very bad solution IMO
                "value" => [*value],
                "currency" => [currency.to_string()],
                "date" => [*date],
                "account_id" => [*account_id],
            )
            .expect("Failed to create credit record");

            self.data_frame = self
                .data_frame
                .vstack(&record)
                .expect("Failed to insert credit record")
        } else if let Transaction::Debit {
            value,
            currency,
            date,
            account_id,
        } = transaction
        {
            let record = df!(
                "fund_movement_id" => [fund_movement_id],
                "fund_movement_type" => ["Debit"], // awful solution IMO
                "value" => [-1.0 * (*value)],
                "currency" => [currency.to_string()],
                "date" => [*date],
                "account_id" => [*account_id],
            )
            .expect("Failed to create debit record");

            self.data_frame = self
                .data_frame
                .vstack(&record)
                .expect("Failed to insert debit record")
        } else {
            panic!("Attempted to insert non-fund into the fund table");
        }
    }

    pub fn display(&self) {
        println!("{}", self.data_frame);
    }
}

pub struct PartyTable {
    pub data_frame: DataFrame,
}

impl PartyTable {
    pub fn new() -> PartyTable {
        let data_frame = DataFrame::new(vec![
            Column::from(Series::new(PlSmallStr::from("party_id"), Vec::<i64>::new())),
            Column::from(Series::new(
                PlSmallStr::from("creation_date"),
                Vec::<NaiveDate>::new(),
            )),
        ])
        .expect("Failed to initialize empty party table"); // considered unsafe. refactor?

        Self { data_frame }
    }

    pub fn try_load() -> Result<PartyTable, String> {
        CsvReadOptions::default()
            .with_infer_schema_length(None)
            .with_has_header(true)
            .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
            .try_into_reader_with_file_path(Some("data/party_table.csv".into()))
            .map_err(|e| format!("Failed to read party table: {}", e))?
            .finish()
            .map_err(|e| format!("Failed to load party table: {}", e))
            .map(|data_frame| Self { data_frame })
    }

    pub fn save(&mut self) -> () {
        if self.data_frame.is_empty() {
            return;
        }

        let mut file =
            File::create("data/party_table.csv").expect("Could not create file party_table.csv");
        CsvWriter::new(&mut file)
            .include_header(true)
            .with_separator(b',')
            .finish(&mut self.data_frame)
            .expect("Failed to save party table.");
    }

    pub fn init() -> PartyTable {
        PartyTable::try_load().unwrap_or_else(|e| PartyTable::new())
    }

    pub fn get_last_party_id(&self) -> i64 {
        if self.data_frame.is_empty() {
            0i64
        } else {
            if let AnyValue::Int64(id) = self
                .data_frame
                .column("party_id")
                .expect("Failed to find party_id column")
                .max_reduce()
                .expect("Failed to generate id")
                .value()
            {
                id + 1i64
            } else {
                panic!("Failed to create an integer id")
            }
        }
    }

    pub fn add_record(&mut self, party: &Party) -> () {
        let party_id: i64 = self.get_last_party_id();

        let record = df!(
            "party_id" => [party_id],
            "creation_date" => [party.creation_date]
        )
        .expect("Failed to create party record");

        self.data_frame = self
            .data_frame
            .vstack(&record)
            .expect("Failed to insert party record")
    }

    pub fn display(&self) {
        println!("{}", self.data_frame);
    }
}

pub struct EntityTable {
    pub data_frame: DataFrame,
}

impl EntityTable {
    pub fn new() -> EntityTable {
        let data_frame = DataFrame::new(vec![
            Column::from(Series::new(
                PlSmallStr::from("entity_id"),
                Vec::<i64>::new(),
            )),
            Column::from(Series::new(PlSmallStr::from("name"), Vec::<String>::new())),
            Column::from(Series::new(
                PlSmallStr::from("country"),
                Vec::<String>::new(),
            )),
            Column::from(Series::new(
                PlSmallStr::from("entity_type"),
                Vec::<String>::new(),
            )),
            Column::from(Series::new(
                PlSmallStr::from("entity_subtype"),
                Vec::<String>::new(),
            )),
            Column::from(Series::new(
                PlSmallStr::from("creation_date"),
                Vec::<NaiveDate>::new(),
            )),
        ])
        .expect("Failed to initialize empty party table"); // considered unsafe. refactor?

        Self { data_frame }
    }

    pub fn try_load() -> Result<EntityTable, String> {
        CsvReadOptions::default()
            .with_infer_schema_length(None)
            .with_has_header(true)
            .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
            .try_into_reader_with_file_path(Some("data/entity_table.csv".into()))
            .map_err(|e| format!("Failed to read entity table: {}", e))?
            .finish()
            .map_err(|e| format!("Failed to load entity table: {}", e))
            .map(|data_frame| Self { data_frame })
    }

    pub fn save(&mut self) -> () {
        if self.data_frame.is_empty() {
            return;
        }

        let mut file =
            File::create("data/entity_table.csv").expect("Could not create file entity_table.csv");
        CsvWriter::new(&mut file)
            .include_header(true)
            .with_separator(b',')
            .finish(&mut self.data_frame)
            .expect("Failed to save entity table.");
    }

    pub fn init() -> EntityTable {
        EntityTable::try_load().unwrap_or_else(|e| EntityTable::new())
    }

    fn get_last_entity_id(&self) -> i64 {
        if self.data_frame.is_empty() {
            0i64
        } else {
            if let AnyValue::Int64(id) = self
                .data_frame
                .column("entity_id")
                .expect("Failed to find entity_id column")
                .max_reduce()
                .expect("Failed to generate id")
                .value()
            {
                id + 1i64
            } else {
                panic!("Failed to create an integer id")
            }
        }
    }

    pub fn add_record(&mut self, entity: &Entity) -> () {
        let entity_id: i64 = self.get_last_entity_id();

        let record = df!(
            "entity_id" => [entity_id],
            "name" => [entity.get_name()],
            "country" => [entity.get_country()],
            "entity_type" => [entity.get_entity_type().to_string()],
            "entity_subtype" => [entity.get_entity_subtype()],
            "creation_date" => [Local::now().date_naive()]
        )
        .expect("Failed to create entity record");

        self.data_frame = self
            .data_frame
            .vstack(&record)
            .expect("Failed to insert entity record")
    }

    pub fn display(&self) {
        println!("{}", self.data_frame);
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (i64, &str)> {
        let id_col = self.data_frame.column("entity_id").unwrap().i64().unwrap();
        let name_col = self.data_frame.column("name").unwrap().str().unwrap();

        id_col.into_no_null_iter().zip(name_col.into_no_null_iter())
    }

    pub(crate) fn get_entity(&self, entity_id: i64) -> Entity {
        let mask = self.data_frame
            .column("entity_id")
            .unwrap()
            .i64()
            .unwrap()
            .equal(entity_id);

        let record = self.data_frame.filter(&mask).unwrap();

        Entity::new(
            record.column("name").unwrap().str().unwrap().get(0).unwrap().to_string(),
            record.column("country").unwrap().str().unwrap().get(0).unwrap().to_string(),
            EntityType::from_str(
                record.column("entity_type").unwrap().str().unwrap().get(0).unwrap()
            ).unwrap(),
            record.column("entity_subtype").unwrap().str().unwrap().get(0).unwrap().to_string()
        )
    }

}
pub struct AccountTable {
    pub data_frame: DataFrame,
}

impl AccountTable {
    pub fn new() -> AccountTable {
        let data_frame = DataFrame::new(vec![
            Column::from(Series::new(
                PlSmallStr::from("account_id"),
                Vec::<i64>::new(),
            )),
            Column::from(Series::new(PlSmallStr::from("name"), Vec::<String>::new())),
            Column::from(Series::new(
                PlSmallStr::from("country"),
                Vec::<String>::new(),
            )),
            Column::from(Series::new(
                PlSmallStr::from("currency"),
                Vec::<String>::new(),
            )),
            Column::from(Series::new(
                PlSmallStr::from("account_type"),
                Vec::<String>::new(),
            )),
            Column::from(Series::new(
                PlSmallStr::from("initial_balance"),
                Vec::<f64>::new(),
            )),
            Column::from(Series::new(
                PlSmallStr::from("creation_date"),
                Vec::<NaiveDate>::new(),
            )),
        ])
        .expect("Failed to initialize empty party table"); // considered unsafe. refactor?

        Self { data_frame }
    }

    pub fn try_load() -> Result<AccountTable, String> {
        CsvReadOptions::default()
            .with_infer_schema_length(None)
            .with_has_header(true)
            .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
            .try_into_reader_with_file_path(Some("data/account_table.csv".into()))
            .map_err(|e| format!("Failed to read account table: {}", e))?
            .finish()
            .map_err(|e| format!("Failed to load account table: {}", e))
            .map(|data_frame| Self { data_frame })
    }

    pub fn save(&mut self) -> () {
        if self.data_frame.is_empty() {
            return;
        }

        let mut file = File::create("data/account_table.csv")
            .expect("Could not create file account_table.csv");
        CsvWriter::new(&mut file)
            .include_header(true)
            .with_separator(b',')
            .finish(&mut self.data_frame)
            .expect("Failed to save account table.");
    }

    pub fn init() -> AccountTable {
        AccountTable::try_load().unwrap_or_else(|e| AccountTable::new())
    }

    fn get_last_account_id(&self) -> i64 {
        if self.data_frame.is_empty() {
            0i64
        } else {
            if let AnyValue::Int64(id) = self
                .data_frame
                .column("account_id")
                .expect("Failed to find account_id column")
                .max_reduce()
                .expect("Failed to generate id")
                .value()
            {
                id + 1i64
            } else {
                panic!("Failed to create an integer id")
            }
        }
    }

    pub fn add_record(&mut self, account: &Account) -> () {
        let account_id: i64 = self.get_last_account_id();

        let record = df!(
            "account_id" => [account_id],
            "name" => [account.get_name()],
            "country" => [account.get_country()],
            "currency" => [account.get_currency().to_string()],
            "account_type" => [account.get_account_type().to_string()],
            "initial_balance" => [account.get_initial_balance()],
            "creation_date" => [Local::now().date_naive()]
        )
        .expect("Failed to create entity record");

        self.data_frame = self
            .data_frame
            .vstack(&record)
            .expect("Failed to insert account record")
    }

    pub fn display(&self) {
        println!("{}", self.data_frame);
    }
}
