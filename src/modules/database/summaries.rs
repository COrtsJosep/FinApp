use crate::modules::currency_exchange::CurrencyExchange;
use crate::modules::database::DataBase;
use crate::modules::financial::Currency;
use chrono::{Local, NaiveDate};
use polars::prelude::*;
use std::str::FromStr;

impl DataBase {
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
                .collect()
                .expect("Failed to join funds")
        }
    }
}
