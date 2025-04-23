use super::financial::{Account, AccountType, Currency, Entity, EntityType, Party, Transaction};
use chrono::{Local, NaiveDate};
use polars::prelude::*;
use std::fs::{create_dir, File};
use std::path::Path;
use std::str::FromStr;
use std::vec::IntoIter;

pub trait Table {
    /// Returns the name of the table
    fn name() -> String;

    /// Returns a reference to the dataframe of the table struct
    fn data_frame(&self) -> &DataFrame;

    /// Returns a mutable reference to the dataframe of the table struct
    fn mut_data_frame(&mut self) -> &mut DataFrame;

    /// Creates a table instance by consuming a dataframe
    fn create(data_frame: DataFrame) -> Box<Self>;

    /// Creates a table instance with zero rows
    fn new() -> Box<Self>;

    /// Creates a table instance by trying to load a csv in the right location
    fn try_load() -> Result<Box<Self>, String> {
        CsvReadOptions::default()
            .with_infer_schema_length(None)
            .with_has_header(true)
            .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
            .try_into_reader_with_file_path(Some(format!("data/{}_table.csv", Self::name()).into()))
            .map_err(|e| format!("Failed to read {} table: {}", Self::name(), e))?
            .finish()
            .map_err(|e| format!("Failed to load {} table: {}", Self::name(), e))
            .map(|data_frame| Self::create(data_frame))
    }

    /// Creates a table instance by trying to load the csv data and,
    /// if there is none, by creating an empty one
    fn init() -> Box<Self> {
        Self::try_load().unwrap_or_else(|_e| Self::new())
    }

    /// Saves the table data in the right location
    fn save(&mut self) -> () {
        if self.data_frame().is_empty() {
            return;
        }

        let file_name: String = format!("data/{}_table.csv", Self::name());
        let path: &Path = Path::new(&file_name);
        if !path.parent().expect("path does not have parent").exists() {
            let _ = create_dir(path.parent().expect("path does not have parents"));
        }

        let mut file = File::create(path)
            .expect(format!("Could not create file {}_table.csv", Self::name()).as_str());

        CsvWriter::new(&mut file)
            .include_header(true)
            .with_separator(b',')
            .finish(&mut self.mut_data_frame())
            .expect(format!("Failed to save {} table.", Self::name()).as_str());
    }

    /// Gets the ID of the last record of the table + 1. If the table is empty,
    /// returns 0
    fn next_id(&self) -> i64 {
        if self.data_frame().is_empty() {
            0i64
        } else {
            if let AnyValue::Int64(id) = self
                .data_frame()
                .column(format!("{}_id", Self::name()).as_str())
                .expect(format!("Failed to find {}_id column", Self::name()).as_str())
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

    /// Prints the table
    fn display(&self) {
        println!("{}", self.data_frame());
    }
}

pub struct IncomeTable {
    pub data_frame: DataFrame,
}

impl Table for IncomeTable {
    fn name() -> String {
        String::from("income")
    }

    fn data_frame(&self) -> &DataFrame {
        &self.data_frame
    }

    fn mut_data_frame(&mut self) -> &mut DataFrame {
        &mut self.data_frame
    }

    fn create(data_frame: DataFrame) -> Box<Self> {
        Box::new(IncomeTable { data_frame })
    }

    fn new() -> Box<Self> {
        let data_frame = DataFrame::new(vec![
            Column::from(Series::new(
                PlSmallStr::from(format!("{}_id", IncomeTable::name())),
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
            Column::from(Series::new(PlSmallStr::from("party_id"), Vec::<i64>::new())),
        ])
        .expect(format!("Failed to initialize empty {} table", IncomeTable::name()).as_str());

        IncomeTable::create(data_frame)
    }
}

impl IncomeTable {
    /// Adds income transaction to the table
    pub fn insert_transaction(&mut self, transaction: &Transaction, party_id: i64) -> () {
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
            let id: i64 = self.next_id();

            let record = df!(
                    format!("{}_id", IncomeTable::name()) => [id],
                    "value" => [*value],
                    "currency" => [currency.to_string()],
                    "date" => [*date],
                    "category" => [category.to_string()],
                    "subcategory" => [subcategory.to_string()],
                    "description" => [description.to_string()],
                    "entity_id" => [*entity_id],
                    "party_id" => [party_id]
            )
            .expect(format!("Failed to create {} record", IncomeTable::name()).as_str());

            self.data_frame = self
                .data_frame()
                .vstack(&record)
                .expect("Failed to insert income record")
        } else {
            panic!("Attempted to insert non-income into the income table");
        }
    }

    pub(crate) fn categories(&self) -> Vec<String> {
        self.data_frame()
            .column("category")
            .unwrap()
            .unique()
            .unwrap()
            .str()
            .unwrap()
            .into_no_null_iter()
            .map(|s| s.to_string())
            .collect()
    }

    pub(crate) fn subcategories(&self, category: String) -> Vec<String> {
        let mask = self
            .data_frame()
            .column("category")
            .unwrap()
            .str()
            .unwrap()
            .equal(category.as_str());

        self.data_frame()
            .filter(&mask)
            .unwrap()
            .column("subcategory")
            .unwrap()
            .unique()
            .unwrap()
            .str()
            .unwrap()
            .into_no_null_iter()
            .map(|s| s.to_string())
            .collect()
    }

    /// Deletes records corresponding to a party.
    pub(crate) fn delete_party(&mut self, party_id: i64) -> () {
        self.data_frame = self
            .data_frame
            .clone()
            .lazy()
            .filter(col("party_id").neq(lit(party_id)))
            .collect()
            .unwrap();
    }

    /// Returns iterator of income_ids that correspond to the given party_id
    pub(crate) fn iter_party(&self, party_id: i64) -> IntoIter<i64> {
        self.data_frame
            .clone()
            .lazy()
            .filter(col("party_id").eq(lit(party_id)))
            .collect()
            .unwrap()
            .column(format!("{}_id", IncomeTable::name()).as_str())
            .unwrap()
            .i64()
            .unwrap()
            .into_no_null_iter()
            .collect::<Vec<i64>>()
            .into_iter()
    }

    /// Returns entity given ID
    pub(crate) fn transaction(&self, id: i64) -> Transaction {
        let mask = self
            .data_frame
            .column(format!("{}_id", IncomeTable::name()).as_str())
            .unwrap()
            .i64()
            .unwrap()
            .equal(id);

        let record = self.data_frame.filter(&mask).unwrap().clone();
        let date = record
            .column("date")
            .unwrap()
            .date()
            .unwrap()
            .as_date_iter()
            .next()
            .unwrap()
            .unwrap()
            .clone();

        Transaction::Income {
            value: record
                .column("value")
                .unwrap()
                .f64()
                .unwrap()
                .get(0)
                .unwrap(),
            currency: Currency::from_str(
                record
                    .column("currency")
                    .unwrap()
                    .str()
                    .unwrap()
                    .get(0)
                    .unwrap(),
            )
            .unwrap(),
            date,
            category: record
                .column("category")
                .unwrap()
                .str()
                .unwrap()
                .get(0)
                .unwrap()
                .to_string(),
            subcategory: record
                .column("subcategory")
                .unwrap()
                .str()
                .unwrap()
                .get(0)
                .unwrap()
                .to_string(),
            description: record
                .column("description")
                .unwrap()
                .str()
                .unwrap()
                .get(0)
                .unwrap()
                .to_string(),
            entity_id: record
                .column("entity_id")
                .unwrap()
                .i64()
                .unwrap()
                .get(0)
                .unwrap(),
        }
    }
}

pub struct ExpensesTable {
    pub data_frame: DataFrame,
}

impl Table for ExpensesTable {
    fn name() -> String {
        String::from("expense")
    }

    fn data_frame(&self) -> &DataFrame {
        &self.data_frame
    }

    fn mut_data_frame(&mut self) -> &mut DataFrame {
        &mut self.data_frame
    }

    fn create(data_frame: DataFrame) -> Box<Self> {
        Box::new(ExpensesTable { data_frame })
    }

    fn new() -> Box<Self> {
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
            Column::from(Series::new(PlSmallStr::from("party_id"), Vec::<i64>::new())),
        ])
        .expect("Failed to initialize empty expenses table");

        ExpensesTable::create(data_frame)
    }
}

impl ExpensesTable {
    /// Adds expense transaction to the table
    pub fn insert_transaction(&mut self, transaction: &Transaction, party_id: i64) -> () {
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
            let id: i64 = self.next_id();

            let record = df!(
                format!("{}_id", ExpensesTable::name()) => [id],
                "value" => [*value],
                "currency" => [currency.to_string()],
                "date" => [*date],
                "category" => [category.to_string()],
                "subcategory" => [subcategory.to_string()],
                "description" => [description.to_string()],
                "entity_id" => [*entity_id],
                "party_id" => [party_id]
            )
            .expect(format!("Failed to create {} record", ExpensesTable::name()).as_str());

            self.data_frame = self
                .data_frame
                .vstack(&record)
                .expect(format!("Failed to insert {} record", ExpensesTable::name()).as_str())
        } else {
            panic!("Attempted to insert non-expense into the expenses table");
        }
    }

    pub(crate) fn categories(&self) -> Vec<String> {
        self.data_frame()
            .column("category")
            .unwrap()
            .unique()
            .unwrap()
            .str()
            .unwrap()
            .sort(false)
            .into_no_null_iter()
            .map(|s| s.to_string())
            .collect()
    }

    pub(crate) fn subcategories(&self, category: String) -> Vec<String> {
        let mask = self
            .data_frame()
            .column("category")
            .unwrap()
            .str()
            .unwrap()
            .equal(category.as_str());

        self.data_frame()
            .filter(&mask)
            .unwrap()
            .column("subcategory")
            .unwrap()
            .unique()
            .unwrap()
            .str()
            .unwrap()
            .sort(false)
            .into_no_null_iter()
            .map(|s| s.to_string())
            .collect()
    }

    // Deletes records corresponding to a party.
    pub(crate) fn delete_party(&mut self, party_id: i64) -> () {
        self.data_frame = self
            .data_frame
            .clone()
            .lazy()
            .filter(col("party_id").neq(lit(party_id)))
            .collect()
            .unwrap();
    }

    /// Returns iterator of expenses_ids that correspond to the given party_id
    pub(crate) fn iter_party(&self, party_id: i64) -> IntoIter<i64> {
        self.data_frame
            .clone()
            .lazy()
            .filter(col("party_id").eq(lit(party_id)))
            .collect()
            .unwrap()
            .column(format!("{}_id", ExpensesTable::name()).as_str())
            .unwrap()
            .i64()
            .unwrap()
            .into_no_null_iter()
            .collect::<Vec<i64>>()
            .into_iter()
    }

    /// Returns entity given ID
    pub(crate) fn transaction(&self, id: i64) -> Transaction {
        let mask = self
            .data_frame
            .column(format!("{}_id", ExpensesTable::name()).as_str())
            .unwrap()
            .i64()
            .unwrap()
            .equal(id);

        let record = self.data_frame.filter(&mask).unwrap().clone();
        let date = record
            .column("date")
            .unwrap()
            .date()
            .unwrap()
            .as_date_iter()
            .next()
            .unwrap()
            .unwrap()
            .clone();

        Transaction::Expense {
            value: record
                .column("value")
                .unwrap()
                .f64()
                .unwrap()
                .get(0)
                .unwrap(),
            currency: Currency::from_str(
                record
                    .column("currency")
                    .unwrap()
                    .str()
                    .unwrap()
                    .get(0)
                    .unwrap(),
            )
            .unwrap(),
            date,
            category: record
                .column("category")
                .unwrap()
                .str()
                .unwrap()
                .get(0)
                .unwrap()
                .to_string(),
            subcategory: record
                .column("subcategory")
                .unwrap()
                .str()
                .unwrap()
                .get(0)
                .unwrap()
                .to_string(),
            description: record
                .column("description")
                .unwrap()
                .str()
                .unwrap()
                .get(0)
                .unwrap()
                .to_string(),
            entity_id: record
                .column("entity_id")
                .unwrap()
                .i64()
                .unwrap()
                .get(0)
                .unwrap(),
        }
    }
}

pub struct FundsTable {
    pub data_frame: DataFrame,
}

impl Table for FundsTable {
    fn name() -> String {
        String::from("fund_movement")
    }

    fn data_frame(&self) -> &DataFrame {
        &self.data_frame
    }

    fn mut_data_frame(&mut self) -> &mut DataFrame {
        &mut self.data_frame
    }

    fn create(data_frame: DataFrame) -> Box<Self> {
        Box::new(FundsTable { data_frame })
    }

    fn new() -> Box<Self> {
        let data_frame = DataFrame::new(vec![
            Column::from(Series::new(
                PlSmallStr::from(format!("{}_id", FundsTable::name())),
                Vec::<i64>::new(),
            )),
            Column::from(Series::new(
                PlSmallStr::from(format!("{}_type", FundsTable::name())),
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
            Column::from(Series::new(PlSmallStr::from("party_id"), Vec::<i64>::new())),
        ])
        .expect(format!("Failed to initialize empty {} table", FundsTable::name()).as_str());

        FundsTable::create(data_frame)
    }
}

impl FundsTable {
    /// Adds funds transaction to the table
    pub fn insert_transaction(&mut self, transaction: &Transaction, party_id: i64) -> () {
        let id: i64 = self.next_id();

        if let Transaction::Credit {
            value,
            currency,
            date,
            account_id,
        } = transaction
        {
            let record = df!(
                format!("{}_id", FundsTable::name()) => [id],
                format!("{}_type", FundsTable::name()) => ["Credit"], // very bad solution IMO
                "value" => [*value],
                "currency" => [currency.to_string()],
                "date" => [*date],
                "account_id" => [*account_id],
                "party_id" => [party_id]
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
                format!("{}_id", FundsTable::name()) => [id],
                format!("{}_type", FundsTable::name()) => ["Debit"], // awful solution IMO
                "value" => [-1.0 * (*value)],
                "currency" => [currency.to_string()],
                "date" => [*date],
                "account_id" => [*account_id],
                "party_id" => [party_id]
            )
            .expect("Failed to create debit record");

            self.data_frame = self
                .data_frame
                .vstack(&record)
                .expect("Failed to insert debit record")
        } else {
            panic!(
                "{}",
                format!(
                    "Attempted to insert non-{} into the {} table",
                    FundsTable::name(),
                    FundsTable::name()
                )
                .as_str()
            );
        }
    }

    // Deletes records corresponding to a party.
    pub(crate) fn delete_party(&mut self, party_id: i64) -> () {
        self.data_frame = self
            .data_frame
            .clone()
            .lazy()
            .filter(col("party_id").neq(lit(party_id)))
            .collect()
            .unwrap();
    }

    /// Returns iterator of funds_ids that correspond to the given party_id
    pub(crate) fn iter_party(&self, party_id: i64) -> IntoIter<i64> {
        self.data_frame
            .clone()
            .lazy()
            .filter(col("party_id").eq(lit(party_id)))
            .collect()
            .unwrap()
            .column(format!("{}_id", FundsTable::name()).as_str())
            .unwrap()
            .i64()
            .unwrap()
            .into_no_null_iter()
            .collect::<Vec<i64>>()
            .into_iter()
    }

    /// Returns entity given ID
    pub(crate) fn transaction(&self, id: i64) -> Transaction {
        let mask = self
            .data_frame
            .column(format!("{}_id", FundsTable::name()).as_str())
            .unwrap()
            .i64()
            .unwrap()
            .equal(id);

        let record = self.data_frame.filter(&mask).unwrap().clone();
        let date = record
            .column("date")
            .unwrap()
            .date()
            .unwrap()
            .as_date_iter()
            .next()
            .unwrap()
            .unwrap()
            .clone();
        let transaction_type = record
            .column(format!("{}_type", FundsTable::name()).as_str())
            .unwrap()
            .str()
            .unwrap()
            .get(0)
            .unwrap()
            .to_string();
        let value = record
            .column("value")
            .unwrap()
            .f64()
            .unwrap()
            .get(0)
            .unwrap();
        let currency = Currency::from_str(
            record
                .column("currency")
                .unwrap()
                .str()
                .unwrap()
                .get(0)
                .unwrap(),
        )
        .unwrap();
        let account_id = record
            .column("account_id")
            .unwrap()
            .i64()
            .unwrap()
            .get(0)
            .unwrap();

        if transaction_type == String::from("Credit") {
            Transaction::Credit {
                value,
                currency,
                date,
                account_id,
            }
        } else {
            // then it is debit
            Transaction::Debit {
                value: -1.0 * value,
                currency,
                date,
                account_id,
            }
        }
    }
}

pub struct PartyTable {
    pub data_frame: DataFrame,
}

impl Table for PartyTable {
    fn name() -> String {
        String::from("party")
    }

    fn data_frame(&self) -> &DataFrame {
        &self.data_frame
    }

    fn mut_data_frame(&mut self) -> &mut DataFrame {
        &mut self.data_frame
    }

    fn create(data_frame: DataFrame) -> Box<Self> {
        Box::new(PartyTable { data_frame })
    }

    fn new() -> Box<Self> {
        let data_frame = DataFrame::new(vec![
            Column::from(Series::new(
                PlSmallStr::from(format!("{}_id", PartyTable::name())),
                Vec::<i64>::new(),
            )),
            Column::from(Series::new(
                PlSmallStr::from("creation_date"),
                Vec::<NaiveDate>::new(),
            )),
        ])
        .expect(format!("Failed to initialize empty {} table", PartyTable::name()).as_str());

        PartyTable::create(data_frame)
    }
}

impl PartyTable {
    /// Adds party record to the table
    pub fn insert_party(&mut self, party: &Party) -> () {
        let id: i64 = self.next_id();

        let record = df!(
            format!("{}_id", PartyTable::name()) => [id],
            "creation_date" => [party.creation_date]
        )
        .expect(format!("Failed to create {} record", PartyTable::name()).as_str());

        self.data_frame = self
            .data_frame
            .vstack(&record)
            .expect(format!("Failed to insert {} record", PartyTable::name()).as_str())
    }

    // Deletes records corresponding to a party.
    pub(crate) fn delete_party(&mut self, party_id: i64) -> () {
        self.data_frame = self
            .data_frame
            .clone()
            .lazy()
            .filter(col("party_id").neq(lit(party_id)))
            .collect()
            .unwrap();
    }
}

pub struct EntityTable {
    pub data_frame: DataFrame,
}

impl Table for EntityTable {
    fn name() -> String {
        String::from("entity")
    }

    fn data_frame(&self) -> &DataFrame {
        &self.data_frame
    }

    fn mut_data_frame(&mut self) -> &mut DataFrame {
        &mut self.data_frame
    }

    fn create(data_frame: DataFrame) -> Box<Self> {
        Box::new(EntityTable { data_frame })
    }

    fn new() -> Box<Self> {
        let data_frame: DataFrame = df!(
            format!("{}_id", EntityTable::name()) => [0i64],
            "name" => ["Unknown"],
            "country" => ["Unknown"],
            format!("{}_type", EntityTable::name()) => [EntityType::default().to_string()],
            format!("{}_subtype", EntityTable::name()) => [""],
            "creation_date" => [Local::now().date_naive()])
        .expect(format!("Failed to initialize empty {} table", EntityTable::name()).as_str());

        EntityTable::create(data_frame)
    }
}

impl EntityTable {
    /// Iterator over IDs
    pub(crate) fn iter(&self) -> IntoIter<i64> {
        self.data_frame
            .sort(["name"], Default::default())
            .unwrap()
            .column(format!("{}_id", EntityTable::name()).as_str())
            .unwrap()
            .i64()
            .unwrap()
            .into_no_null_iter()
            .collect::<Vec<i64>>()
            .into_iter()
    }

    /// Adds entity to the table
    pub fn insert_entity(&mut self, entity: &Entity) -> i64 {
        let id: i64 = self.next_id();

        let record = df!(
            format!("{}_id", EntityTable::name()) => [id],
            "name" => [entity.name()],
            "country" => [entity.country()],
            format!("{}_type", EntityTable::name()) => [entity.entity_type().to_string()],
            format!("{}_subtype", EntityTable::name()) => [entity.entity_subtype()],
            "creation_date" => [Local::now().date_naive()]
        )
        .expect(format!("Failed to create {} record", EntityTable::name()).as_str());

        self.data_frame = self
            .data_frame
            .vstack(&record)
            .expect(format!("Failed to insert {} record", EntityTable::name()).as_str());

        id
    }

    /// Returns entity given ID
    pub(crate) fn entity(&self, id: i64) -> Entity {
        let mask = self
            .data_frame
            .column(format!("{}_id", EntityTable::name()).as_str())
            .unwrap()
            .i64()
            .unwrap()
            .equal(id);

        let record = self.data_frame.filter(&mask).unwrap();

        Entity::new(
            record
                .column("name")
                .unwrap()
                .str()
                .unwrap()
                .get(0)
                .unwrap()
                .to_string(),
            record
                .column("country")
                .unwrap()
                .str()
                .unwrap()
                .get(0)
                .unwrap()
                .to_string(),
            EntityType::from_str(
                record
                    .column(format!("{}_type", EntityTable::name()).as_str())
                    .unwrap()
                    .str()
                    .unwrap()
                    .get(0)
                    .unwrap(),
            )
            .unwrap(),
            record
                .column(format!("{}_subtype", EntityTable::name()).as_str())
                .unwrap()
                .str()
                .unwrap()
                .get(0)
                .unwrap()
                .to_string(),
        )
    }

    /// Returns list of unique countries
    pub(crate) fn countries(&self) -> Vec<String> {
        self.data_frame()
            .column("country")
            .unwrap()
            .unique()
            .unwrap()
            .str()
            .unwrap()
            .sort(false)
            .into_no_null_iter()
            .map(|s| s.to_string())
            .collect()
    }

    pub(crate) fn subtypes(&self) -> Vec<String> {
        // filter type?
        self.data_frame()
            .column(format!("{}_subtype", EntityTable::name()).as_str())
            .unwrap()
            .unique()
            .unwrap()
            .str()
            .unwrap()
            .sort(false)
            .into_no_null_iter()
            .map(|s| s.to_string())
            .collect()
    }
}
pub struct AccountTable {
    pub data_frame: DataFrame,
}

impl Table for AccountTable {
    fn name() -> String {
        String::from("account")
    }

    fn data_frame(&self) -> &DataFrame {
        &self.data_frame
    }

    fn mut_data_frame(&mut self) -> &mut DataFrame {
        &mut self.data_frame
    }

    fn create(data_frame: DataFrame) -> Box<Self> {
        Box::new(AccountTable { data_frame })
    }

    fn new() -> Box<Self> {
        let data_frame: DataFrame = df!(
            format!("{}_id", AccountTable::name()) => [0i64],
            "name" => ["Unknown"],
            "country" => ["Unknown"],
            "currency" => [Currency::default().to_string()],
            format!("{}_type", AccountTable::name()) => [AccountType::default().to_string()],
            "initial_balance" => [0.0f64],
            "creation_date" => [Local::now().date_naive()])
        .expect(format!("Failed to initialize empty {} table", AccountTable::name()).as_str());

        AccountTable::create(data_frame)
    }
}

impl AccountTable {
    /// Iterator over IDs
    pub(crate) fn iter(&self) -> IntoIter<i64> {
        self.data_frame
            .sort(["name"], Default::default())
            .unwrap()
            .column(format!("{}_id", AccountTable::name()).as_str())
            .unwrap()
            .i64()
            .unwrap()
            .into_no_null_iter()
            .collect::<Vec<i64>>()
            .into_iter()
    }

    /// Adds account record to the table
    pub fn insert_account(&mut self, account: &Account) -> i64 {
        let id: i64 = self.next_id();

        let record = df!(
            format!("{}_id", AccountTable::name()) => [id],
            "name" => [account.name()],
            "country" => [account.country()],
            "currency" => [account.currency().to_string()],
            format!("{}_type", AccountTable::name()) => [account.account_type().to_string()],
            "initial_balance" => [account.initial_balance()],
            "creation_date" => [Local::now().date_naive()]
        )
        .expect(format!("Failed to create {} record", AccountTable::name()).as_str());

        self.data_frame = self
            .data_frame
            .vstack(&record)
            .expect(format!("Failed to insert {} record", AccountTable::name()).as_str());

        id
    }

    /// Retrieves account from the table, given ID
    pub(crate) fn account(&self, id: i64) -> Account {
        let mask = self
            .data_frame
            .column(format!("{}_id", AccountTable::name()).as_str())
            .unwrap()
            .i64()
            .unwrap()
            .equal(id);

        let record = self.data_frame.filter(&mask).unwrap();

        Account::new(
            record
                .column("name")
                .unwrap()
                .str()
                .unwrap()
                .get(0)
                .unwrap()
                .to_string(),
            record
                .column("country")
                .unwrap()
                .str()
                .unwrap()
                .get(0)
                .unwrap()
                .to_string(),
            Currency::from_str(
                record
                    .column("currency")
                    .unwrap()
                    .str()
                    .unwrap()
                    .get(0)
                    .unwrap(),
            )
            .unwrap(),
            AccountType::from_str(
                record
                    .column(format!("{}_type", AccountTable::name()).as_str())
                    .unwrap()
                    .str()
                    .unwrap()
                    .get(0)
                    .unwrap(),
            )
            .unwrap(),
            record
                .column("initial_balance")
                .unwrap()
                .f64()
                .unwrap()
                .get(0)
                .unwrap(),
        )
    }

    pub(crate) fn countries(&self) -> Vec<String> {
        self.data_frame()
            .column("country")
            .unwrap()
            .unique()
            .unwrap()
            .str()
            .unwrap()
            .sort(false)
            .into_no_null_iter()
            .map(|s| s.to_string())
            .collect()
    }
}
