pub mod tables {
    use chrono::NaiveDate;
    use polars::prelude::*;
    use crate::classes::financial::{Transaction};

    pub struct EarningsTable {
        pub data_frame: DataFrame
    }

    impl EarningsTable {
        pub fn new() -> EarningsTable {
            let data_frame = DataFrame::new(vec![
                Column::from(Series::new(PlSmallStr::from("income_id"), Vec::<u32>::new())),
                Column::from(Series::new(PlSmallStr::from("value"), Vec::<f32>::new())),
                Column::from(Series::new(PlSmallStr::from("currency"), Vec::<String>::new())),
                Column::from(Series::new(PlSmallStr::from("date"), Vec::<NaiveDate>::new())),
                Column::from(Series::new(PlSmallStr::from("category"), Vec::<String>::new())),
                Column::from(Series::new(PlSmallStr::from("subcategory"), Vec::<String>::new())),
                Column::from(Series::new(PlSmallStr::from("description"), Vec::<String>::new())),
                Column::from(Series::new(PlSmallStr::from("entity_id"), Vec::<u32>::new())),
            ]).expect("Failed to initialize empty earnings table"); // considered unsafe. refactor?

            Self { data_frame }
        }

        pub fn load() -> EarningsTable {
            let data_frame = CsvReadOptions::default()
                .with_infer_schema_length(None)
                .with_has_header(true)
                .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
                .try_into_reader_with_file_path(Some("path/file.csv".into()))
                .expect("Failed to read earnings table") // considered unsafe. refactor?
                .finish()
                .expect("Failed to load earnings table");

            Self { data_frame }
        }

        pub fn add_record(&mut self, transaction: &Transaction) -> () {
            if let Transaction::Earning {
                value,
                currency,
                date,
                category,
                subcategory,
                description,
                entity_id
            } = transaction {
                let income_id: u32 = {
                    if self.data_frame.is_empty() { 0 }
                    else {
                        if let AnyValue::UInt32(id) = self.data_frame.column("income_id").expect("Failed to find income_id column").max_reduce().expect("Failed to generate id").value() { id + 1 }
                        else {panic!("Failed to create an integer id")}
                    }
                };

                let record = df!(
                    "income_id" => [income_id],
                    "value" => [*value],
                    "currency" => [currency.to_string()],
                    "date" => [*date],
                    "category" => [category.to_string()],
                    "subcategory" => [subcategory.to_string()],
                    "description" => [description.to_string()],
                    "entity_id" => [*entity_id],
                ).expect("Failed to create earning record");

                self.data_frame = self.data_frame.vstack(&record).expect("Failed to insert earning record")
            } else {
                panic!("Attempted to insert non-earning into the earnings table");
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
                let expense_id: u32 = {
                    if self.data_frame.is_empty() { 0 }
                    else {
                        if let AnyValue::UInt32(id) = self.data_frame.column("expense_id").expect("Failed to find expense_id column").max_reduce().expect("Failed to generate id").value() { id + 1 }
                        else {panic!("Failed to create an integer id")}
                    }
                };

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

        pub fn add_record(&mut self, transaction: &Transaction) -> () {
            let fund_movement_id: u32 = {
                if self.data_frame.is_empty() { 0u32 }
                else {
                    if let AnyValue::UInt32(id) = self.data_frame.column("fund_movement_id").expect("Failed to find fund_movement_id column").max_reduce().expect("Failed to generate id").value() { id + 1u32 }
                    else {panic!("Failed to create an integer id")}
                }
            };

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
                panic!("Attempted to insert non-earning into the earnings table");
            }
        }

        pub fn display(&self) {
            println!("{}", self.data_frame);
        }
    }

    pub struct DataBase {
        earnings_table: EarningsTable,
        expenses_table: ExpensesTable,
        funds_table: FundsTable
    }
}

#[cfg(test)]
mod tests {
    use crate::classes::financial::*;
    use crate::tables::tables::*;
    use chrono::prelude::*;
    use polars::prelude::*;

    fn init_funds_table() -> FundsTable {
        let data_frame: DataFrame = df!(
            "fund_movement_id" => [0u32, 1u32],
            "fund_movement_type" => ["Credit", "Debit"],
            "value" => [1309.23f32, -89.0f32],
            "currency" => [Currency::EUR.to_string(), Currency::EUR.to_string()],
            "date" => [
                NaiveDate::from_ymd_opt(1997, 1, 10).unwrap(),
                NaiveDate::from_ymd_opt(1985, 2, 15).unwrap()
            ],
            "account_id" => [0u32, 0u32]
        ).unwrap();

        FundsTable { data_frame }
    }

    #[test]
    fn correct_funds_table_init() {
        let funds_table: FundsTable = FundsTable::new();

        assert!(funds_table.data_frame.is_empty());
    }

    #[test]
    fn correct_id_empty_funds_table_init() {
        let mut funds_table: FundsTable = FundsTable::new();

        let record = Transaction::Debit {
            value: 300.0,
            currency: Currency::EUR,
            date: NaiveDate::from_ymd_opt(2024, 12, 2).unwrap(),
            account_id: 0u32,
        };

        funds_table.add_record(&record);

        let binding = funds_table.data_frame.column("fund_movement_id").unwrap().max_reduce().unwrap();
        let actual_last_id = binding.value();
        let expected_last_id = AnyValue::UInt32(0u32);

        assert_eq!(actual_last_id, &expected_last_id)
    }

    #[test]
    fn correct_id_nonempty_funds_table_addition() {
        let mut funds_table: FundsTable = init_funds_table();
        let record = Transaction::Debit {
            value: 300.0,
            currency: Currency::EUR,
            date: NaiveDate::from_ymd_opt(2024, 12, 2).unwrap(),
            account_id: 0u32,
        };

        funds_table.add_record(&record);

        let binding = funds_table.data_frame.column("fund_movement_id").unwrap().max_reduce().unwrap();
        let actual_last_id = binding.value();
        let expected_last_id = AnyValue::UInt32(2u32);

        assert_eq!(actual_last_id, &expected_last_id)
    }




}

