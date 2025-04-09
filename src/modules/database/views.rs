use crate::modules::database::{capitalize_every_word, data_frame_to_csv_string, DataBase};
use polars::prelude::*;

impl DataBase {
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
}
