use crate::modules::financial::Currency;
use chrono::{Local, NaiveDate};
use polars::prelude::*;
use reqwest;
use std::collections::HashMap;
use std::fs::File;
use std::io::Cursor;
use std::string::String;
use strum::IntoEnumIterator;

const BASE_CURRENCY: Currency = Currency::EUR;

pub enum Extremum {
    MIN,
    MAX,
}

pub struct CurrencyExchange {
    hash_map: HashMap<String, DataFrame>,
}

impl CurrencyExchange {
    /// Downloads the exchange rate table from the ECB, optionally from a starting date.
    pub(crate) fn download(currency: &Currency, from_date: Option<NaiveDate>) -> DataFrame {
        let start_period_string: String = match from_date {
            Some(date) => format!("&startPeriod={}", date.to_string()),
            None => String::default(),
        };

        let url: String = format!("https://data-api.ecb.europa.eu/service/data/EXR/D.{}.{}.SP00.A?format=csvdata&detail=dataonly{}",
            currency, BASE_CURRENCY, start_period_string
        );

        let response = reqwest::blocking::get(url).expect("Could not make request");
        let csv_data = response.bytes().expect("Could not read response bytes");

        let cursor = Cursor::new(csv_data);
        let data_frame: DataFrame = CsvReadOptions::default()
            .with_infer_schema_length(None)
            .with_has_header(true)
            .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
            .into_reader_with_file_handle(cursor)
            .finish().expect("CSV download error")
            .lazy()
            .select([
                col("TIME_PERIOD").alias("date"),
                (lit(1.0) / col("OBS_VALUE")).alias("value")
            ])
            .collect()
            .expect("Failed to transform and rename");

        data_frame.to_owned()
    }

    /// Tries to read the exchange rate table from the expected path. If it's there, it is loaded, 
    /// and if not up-to-date, it is enhanced with fresh data from the ECB.
    fn load(currency: &Currency) -> Result<DataFrame, String> {
        // could be refactored
        let key: String = CurrencyExchange::key(currency, &BASE_CURRENCY);

        let data_frame: Result<DataFrame, String> = CsvReadOptions::default()
            .with_infer_schema_length(None)
            .with_has_header(true)
            .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
            .try_into_reader_with_file_path(Some(format!("data/exchange_rate_{}.csv", key).into()))
            .map_err(|e| format!("Failed to read {} table: {}", key, e))?
            .finish()
            .map_err(|e| format!("Failed to load {} table: {}", key, e));
        
        if let Err(data_frame) = data_frame {
            return Err(data_frame);
        } 
        
        let mut data_frame = data_frame?;

        let extremum: Extremum = Extremum::MAX;
        let max_date: NaiveDate = Self::extreme_date(&data_frame, &extremum);
        if max_date < Local::now().date_naive() {
            let new_records: DataFrame = Self::download(currency, Option::from(max_date));
            data_frame = data_frame.vstack(&new_records).expect("Could not append new data")
        }

        Ok(data_frame)
    }

    /// Adds any missing days and fills them with a forward rolling strategy.
    fn expand(data_frame: &DataFrame) -> DataFrame {
        data_frame
            .clone()
            .upsample::<[String; 0]>([], "date", Duration::parse("1d")).expect("Failed to expand date")
            .fill_null(FillNullStrategy::Forward(None)).expect("Failed to fill null values")
    }
    
    #[cfg(test)]
    pub(crate) fn test_expand(data_frame: &DataFrame) -> DataFrame {
        Self::expand(&data_frame)
    }

    /// Returns the most recent date of the dataframe's column "date"
    fn extreme_date(data_frame: &DataFrame, extrema: &Extremum) -> NaiveDate {
        let iterator = data_frame
            .column("date").expect("Could not find date column")
            .date().expect("Could not convert date column to date (what the hell does that mean?)")
            .as_date_iter();

        match extrema {
            Extremum::MIN => {
                iterator.min().expect("Min could not be performed").expect("Column has no min")
            },
            Extremum::MAX => {
                iterator.max().expect("Max could not be performed").expect("Column has no max")
            }
        }
    }

    #[cfg(test)] // public only for testing
    pub(crate) fn test_extreme_date(data_frame: &DataFrame, extremum: &Extremum) -> NaiveDate {
        Self::extreme_date(data_frame, extremum)
    }

    #[cfg(test)]
    pub(crate) fn new(hash_map: HashMap<String, DataFrame>) -> CurrencyExchange{
        CurrencyExchange{ hash_map }
    }

    /// Initializes the currency exchange module
    pub(crate) fn init() -> CurrencyExchange {
        let mut hash_map: HashMap<String, DataFrame> = HashMap::new();

        for currency in Currency::iter() {
            if currency == BASE_CURRENCY { continue; }
            
            let key: String = CurrencyExchange::key(&currency, &BASE_CURRENCY);
            let mut data_frame: DataFrame = match Self::load(&currency) {
                Ok(data_frame) => data_frame,
                Err(_err) => Self::download(&currency, None),
            };

            let extremum: Extremum = Extremum::MAX;
            if Self::extreme_date(&data_frame, &extremum) < Local::now().date_naive() {
                let extra_row: DataFrame = df!(
                    "date" => [Local::now().date_naive()],
                    "value" => [None::<f64>]
                )
                .unwrap();

                data_frame = data_frame.vstack(&extra_row).unwrap();
            }

            data_frame = Self::expand(&mut data_frame);
            hash_map.insert(key, data_frame);
        }
        
        CurrencyExchange { hash_map }
    }
    
    /// Saves the currency exchange tables.
    pub(crate) fn save(&mut self) -> () {
        for (key, data_frame) in self.hash_map.iter_mut() {
            if data_frame.is_empty() {
                return;
            }

            let mut file =
                File::create(format!("data/exchange_rate_{}.csv", key))
                    .expect(format!("Could not create file {}_table.csv", key).as_str());

            CsvWriter::new(&mut file)
                .include_header(true)
                .with_separator(b',')
                .finish(data_frame)
                .expect(format!("Failed to save {} table.", key).as_str());
        }
    }
    
    /// Creates a key for the currency exchange HashMap (just the concatenation of the currency
    /// names)
    fn key(currency_from: &Currency, currency_to: &Currency) -> String {
        format!("{}{}", currency_from, currency_to)
    }

    /// Returns the historic exchange rate between two currencies at a given date
    pub(crate) fn exchange(
        &self,
        currency_from: &Currency,
        currency_to: &Currency,
        date: NaiveDate,
    ) -> f64 {
        if currency_to == currency_from { return 1.0; }

        let key: String = CurrencyExchange::key(currency_from, currency_to);
        let inverse_key: String = CurrencyExchange::key(currency_to, currency_from);
        if self.hash_map.contains_key(&key) {
            let data_frame = self.hash_map.get(&key).unwrap();
            assert!(Self::extreme_date(data_frame, &Extremum::MIN) <= date, "Tried to get an exchange rate too far away in the past");
            assert!(date <= Self::extreme_date(data_frame, &Extremum::MAX), "Tried to get an exchange rate too near the present");

            data_frame
                .clone()
                .lazy()
                .filter(col("date").eq(lit(date)))
                .collect().expect("Failed to filter date")
                .column("value").expect("Failed to find column 'value'")
                .f64().expect("Failed to convert value to float")
                .get(0).expect("Failed to find observation")

        } else if self.hash_map.contains_key(&inverse_key) {
            1.0 / self.exchange(currency_to, currency_from, date)
        } else {
            self.exchange(currency_from, &BASE_CURRENCY, date) * self.exchange(&BASE_CURRENCY, currency_to, date)
        }
    }
}
