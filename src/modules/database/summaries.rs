use crate::modules::currency_exchange::CurrencyExchange;
use crate::modules::database::DataBase;
use crate::modules::financial::Currency;
use chrono::{Local, NaiveDate};
use polars::prelude::*;
use std::io::Cursor;
use std::str::FromStr;

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

impl DataBase {
    /// Calculates the sum of all the incomes earned between date_from to date_to, both included,
    /// in the currency currency_to.
    fn total_income(
        &self,
        date_from: NaiveDate,
        date_to: NaiveDate,
        currency_to: &Currency,
    ) -> f64 {
        let currency_exchange: CurrencyExchange = CurrencyExchange::init();

        let income_table: DataFrame = self
            .incomes_table
            .data_frame
            .clone()
            .lazy()
            .filter(col("date").is_between(lit(date_from), lit(date_to), ClosedInterval::Both))
            .collect()
            .unwrap();

        let mut exchange_rates = Vec::new();
        let currency_iterator = income_table
            .column("currency")
            .unwrap()
            .str()
            .unwrap()
            .into_iter();
        for currency in currency_iterator {
            let currency_from =
                Currency::from_str(currency.unwrap()).expect("Failed to find currency");
            let exchange_rate: f64 =
                currency_exchange.exchange_currency(&currency_from, currency_to, date_to);
            exchange_rates.push(exchange_rate);
        }

        let exchange_rates: Series = Series::new("exchange_rate".into(), exchange_rates);

        income_table
            .lazy()
            .with_column(exchange_rates.lit())
            .with_column((col("exchange_rate") * col("value")).alias(currency_to.to_string()))
            .collect()
            .unwrap()
            .column(currency_to.to_string().as_str())
            .unwrap()
            .f64()
            .unwrap()
            .sum()
            .unwrap()
    }

    pub(crate) fn current_fund_stand(&self, currency_to: Option<&Currency>) -> DataFrame {
        let currency_exchange: CurrencyExchange = CurrencyExchange::init();

        let initial_balances: DataFrame = self.account_table.data_frame.clone();

        let funds_table: DataFrame = self
            .funds_table
            .data_frame
            .clone()
            .lazy()
            .group_by(["account_id", "currency"])
            .agg([col("value").sum()])
            .collect()
            .expect("Failed to aggregate account values");

        let summary = initial_balances
            .lazy()
            .join(
                funds_table.clone().lazy(),
                [col("account_id"), col("currency")],
                [col("account_id"), col("currency")],
                JoinArgs::new(JoinType::Left),
            )
            .with_column(col("value").fill_null(0.0))
            .with_column((col("initial_balance") + col("value")).alias("total_value"))
            .collect()
            .expect("Failed to join funds");

        if let Some(currency_to) = currency_to {
            let mut exchange_rates = Vec::new();
            let currency_iterator = summary
                .column("currency")
                .unwrap()
                .str()
                .unwrap()
                .into_iter();
            for currency in currency_iterator {
                let currency_from =
                    Currency::from_str(currency.unwrap()).expect("Failed to find currency");
                let exchange_rate: f64 = currency_exchange.exchange_currency(
                    &currency_from,
                    currency_to,
                    Local::now().date_naive(),
                );
                exchange_rates.push(exchange_rate);
            }

            let exchange_rates: Series = Series::new("exchange_rate".into(), exchange_rates);

            summary
                .lazy()
                .with_column(exchange_rates.lit())
                .with_column(
                    (col("exchange_rate") * col("total_value")).alias(currency_to.to_string()),
                )
                .sort(
                    [currency_to.to_string()],
                    SortMultipleOptions::default().with_order_descending(true),
                )
                .select([
                    col("name"),
                    col("country"),
                    col("account_type"),
                    col(currency_to.to_string()),
                ])
                .filter(col(currency_to.to_string()).gt_eq(lit(0.01)))
                .select([all().name().map(|name| {
                    Ok(PlSmallStr::from_string(
                        name.replace("_", " ").to_uppercase(),
                    ))
                })])
                .collect()
                .unwrap()
        } else {
            summary
                .clone()
                .lazy()
                .sort(
                    ["currency", "total_value"],
                    SortMultipleOptions::default().with_order_descending_multi([false, true]),
                )
                .select([
                    col("name"),
                    col("country"),
                    col("currency"),
                    col("account_type"),
                    col("total_value"),
                ])
                .filter(col("total_value").gt_eq(lit(0.01)))
                .select([all().name().map(|name| {
                    Ok(PlSmallStr::from_string(
                        name.replace("_", " ").to_uppercase(),
                    ))
                })])
                .collect()
                .unwrap()
        }
    }

    /// Generates a summary table of all expenses between date_from to date_to, expressed in the currency_to
    pub(crate) fn expenses_summary(
        &self,
        date_from: NaiveDate,
        date_to: NaiveDate,
        currency_to: &Currency,
    ) -> String {
        let currency_exchange: CurrencyExchange = CurrencyExchange::init();
        let total_income: f64 = self.total_income(date_from, date_to, currency_to);

        let expenses_table: DataFrame = self
            .expenses_table
            .data_frame
            .clone()
            .lazy()
            .filter(col("date").is_between(lit(date_from), lit(date_to), ClosedInterval::Both))
            .collect()
            .unwrap();

        let mut exchange_rates = Vec::new();
        let currency_iterator = expenses_table
            .column("currency")
            .unwrap()
            .str()
            .unwrap()
            .into_iter();
        for currency in currency_iterator {
            let currency_from =
                Currency::from_str(currency.unwrap()).expect("Failed to find currency");
            let exchange_rate: f64 =
                currency_exchange.exchange_currency(&currency_from, currency_to, date_to);
            exchange_rates.push(exchange_rate);
        }

        let exchange_rates: Series = Series::new("exchange_rate".into(), exchange_rates);

        let mut summary: DataFrame = expenses_table
            .lazy()
            .with_column(exchange_rates.lit())
            .with_column((col("exchange_rate") * col("value")).alias(currency_to.to_string()))
            .group_by([col("category"), col("subcategory")])
            .agg([col(currency_to.to_string()).sum()])
            .with_columns([
                col(currency_to.to_string()).round(2),
                (col(currency_to.to_string()) * lit(100) / col(currency_to.to_string()).sum())
                    .round(2)
                    .alias("%_total_expenses"),
                (col(currency_to.to_string()) * lit(100) / lit(total_income))
                    .round(2)
                    .alias("%_total_income"),
            ])
            .sort(
                ["category", "subcategory"],
                SortMultipleOptions::default().with_order_descending_multi([false, false]),
            )
            .select([all().name().map(|name| {
                Ok(PlSmallStr::from_string(capitalize_every_word(
                    name.replace("_", " "),
                )))
            })])
            .collect()
            .unwrap();

        let total_expenses: f64 = summary
            .column(currency_to.to_string().as_str())
            .unwrap()
            .f64()
            .unwrap()
            .sum()
            .unwrap();

        let last_row: DataFrame = df!(
        "Category" => ["Total"],
        "Subcategory" => ["Total"],
        currency_to.to_string().as_str() => [(100.0 * total_expenses).round() / 100.0],
        "% Total Expenses" => [100.0],
        "% Total Income" => [(100.0 * 100.0 * total_expenses / total_income).round() / 100.0]
        )
        .unwrap();

        summary = summary.vstack(&last_row).unwrap();

        data_frame_to_csv_string(&mut summary)
    }
}
