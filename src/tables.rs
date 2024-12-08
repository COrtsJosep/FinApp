pub mod tables {
    use chrono::NaiveDate;
    use polars::prelude::*;
    use crate::classes::financial::{Transaction};

    pub struct EarningsTable {
        pub data_frame: DataFrame
    }

    impl EarningsTable {
        pub fn new() -> EarningsTable {
            let mut data_frame = DataFrame::new(vec![
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
                    if self.data_frame.is_empty() {println!("oooo"); 0 }
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

    pub struct FundsTable {
        pub data_frame: DataFrame
    }

    pub struct DataBase {
        earnings_table: EarningsTable,
        expenses_table: ExpensesTable,
        funds_table: FundsTable
    }
}