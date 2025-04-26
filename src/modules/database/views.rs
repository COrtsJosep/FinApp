use crate::modules::database::{capitalize_every_word, data_frame_to_csv_string, DataBase};
use polars::prelude::*;

impl DataBase {
    /// Returns a csv in String format with the last n transactions.
    pub(crate) fn last_transactions(&self, n: usize) -> String {
        let incomes_table: DataFrame = self
            .incomes_table
            .data_frame
            .clone()
            .lazy()
            .select([all().exclude(["income_id"])])
            .with_column(lit("Income").alias("type"))
            .collect()
            .unwrap();
        let expenses_table: DataFrame = self
            .expenses_table
            .data_frame
            .clone()
            .lazy()
            .select([all().exclude(["expense_id"])])
            .with_column(lit("Expense").alias("type"))
            .collect()
            .unwrap();

        let entities_table: DataFrame = self
            .entity_table
            .data_frame
            .clone()
            .lazy()
            .select([col("entity_id"), col("name")])
            .rename(["name"], ["entity_name"], true)
            .collect()
            .unwrap();

        let transactions_table: DataFrame = incomes_table
            .vstack(&expenses_table)
            .unwrap()
            .inner_join(&entities_table, ["entity_id"], ["entity_id"])
            .unwrap()
            .select([
                "type",
                "date",
                "value",
                "currency",
                "entity_name",
                "category",
                "subcategory",
                "description",
                "party_id",
            ])
            .unwrap()
            .lazy()
            .sort(
                ["date", "party_id"],
                SortMultipleOptions::default().with_order_descending_multi([true, true]),
            )
            .select([all().name().map(|name| {
                Ok(PlSmallStr::from_string(capitalize_every_word(
                    name.replace("_", " "),
                )))
            })])
            .collect()
            .unwrap();

        data_frame_to_csv_string(&mut transactions_table.head(Some(n)))
    }

    /// Returns a csv in String format with the last n fund movements.
    pub(crate) fn last_fund_movements(&self, n: usize) -> String {
        let funds_table: DataFrame = self
            .funds_table
            .data_frame
            .clone()
            .lazy()
            .select([all().exclude(["fund_movement_id"])])
            .collect()
            .unwrap();

        let accounts_table: DataFrame = self
            .account_table
            .data_frame
            .clone()
            .lazy()
            .select([col("account_id"), col("name")])
            .rename(["name"], ["account_name"], true)
            .collect()
            .unwrap();

        let mut last_fund_movements = funds_table
            .inner_join(&accounts_table, ["account_id"], ["account_id"])
            .unwrap()
            .select([
                "fund_movement_type",
                "date",
                "value",
                "currency",
                "account_name",
                "party_id",
            ])
            .unwrap()
            .lazy()
            .sort(
                ["date", "party_id"],
                SortMultipleOptions::default().with_order_descending_multi([true, true]),
            )
            .select([all().name().map(|name| {
                Ok(PlSmallStr::from_string(capitalize_every_word(
                    name.replace("_", " "),
                )))
            })])
            .collect()
            .unwrap()
            .head(Some(n));

        data_frame_to_csv_string(&mut last_fund_movements)
    }
}
