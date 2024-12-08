pub mod tables {
    use chrono::NaiveDate;
    use polars::prelude::*;
    use crate::classes::financial::{Currency, Transaction};

    pub struct EarningsTable {
        pub data_frame: DataFrame
    }

    impl EarningsTable {
        fn new() -> EarningsTable {
            let mut data_frame = DataFrame::new(vec![
                Column::from(Series::new(PlSmallStr::from("income_id"), Vec::<u32>::new())),
                Column::from(Series::new(PlSmallStr::from("value"), Vec::<f32>::new())),
                Column::from(Series::new(PlSmallStr::from("currency"), Vec::<Currency>::new())),
                Column::from(Series::new(PlSmallStr::from("date"), Vec::<NaiveDate>::new())),
                Column::from(Series::new(PlSmallStr::from("category"), Vec::<String>::new())),
                Column::from(Series::new(PlSmallStr::from("subcategory"), Vec::<String>::new())),
                Column::from(Series::new(PlSmallStr::from("description"), Vec::<String>::new())),
                Column::from(Series::new(PlSmallStr::from("entity_id"), Vec::<u32>::new())),
            ]).expect("Failed to initialize empty earnings table"); // considered unsafe. refactor?

            Self { data_frame }
        }

        fn load() -> EarningsTable {
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

        fn add_record(&mut self, transaction: &Transaction::Earning) {
            let record = df![
                "income_id" => [self.data_frame.max("income_id") + 1],
                "value" => [transaction.value],
                "currency" => [transaction.currency.to_string()],
                "date" => [transaction.date],
                "category" => [transaction.category],
                "subcategory" => [transaction.subcategory],
                "description" => [transaction.description],
                "entity_id" => [transaction.entity_id]
            ]?;
            self.data_frame = self.data_frame.vstack(&record)?;
            Ok(())
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