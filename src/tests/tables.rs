#[cfg(test)]
mod tests {
    use crate::modules::financial::*;
    use crate::modules::tables::*;
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

        let transaction = Transaction::Debit {
            value: 300.0,
            currency: Currency::EUR,
            date: NaiveDate::from_ymd_opt(2024, 12, 2).unwrap(),
            account_id: 0u32,
        };

        funds_table.add_record(&transaction);

        let binding = funds_table.data_frame.column("fund_movement_id").unwrap().max_reduce().unwrap();
        let actual_last_id = binding.value();
        let expected_last_id = AnyValue::UInt32(0u32);

        assert_eq!(actual_last_id, &expected_last_id)
    }

    #[test]
    fn correct_id_nonempty_funds_table_addition() {
        let mut funds_table: FundsTable = init_funds_table();
        let transaction = Transaction::Debit {
            value: 300.0,
            currency: Currency::EUR,
            date: NaiveDate::from_ymd_opt(2024, 12, 2).unwrap(),
            account_id: 0u32,
        };

        funds_table.add_record(&transaction);

        let binding = funds_table.data_frame.column("fund_movement_id").unwrap().max_reduce().unwrap();
        let actual_last_id = binding.value();
        let expected_last_id = AnyValue::UInt32(2u32);

        assert_eq!(actual_last_id, &expected_last_id)
    }

    #[test]
    fn correct_entity_table_init() {
        let entity_table: EntityTable = EntityTable::new();

        assert!(entity_table.data_frame.is_empty());
    }

    #[test]
    fn correct_id_empty_entity_table_init() {
        let mut entity_table: EntityTable = EntityTable::new();

        let entity = Entity::new(
            String::from("Aldi"),
            String::from("Germany"),
            EntityType::Firm,
            String::from("Supermarket"),
        );

        entity_table.add_record(&entity);

        let binding = entity_table.data_frame.column("entity_id").unwrap().max_reduce().unwrap();
        let actual_last_id = binding.value();
        let expected_last_id = AnyValue::UInt32(0u32);

        assert_eq!(actual_last_id, &expected_last_id)
    }

    #[test]
    fn correct_id_empty_account_table_init() {
        let mut account_table: AccountTable = AccountTable::new();

        let account = Account::new(
            String::from("Current account"),
            String::from("Credit Suisse"),
            Currency::CHF,
            AccountType::Deposit,
            1080.0f32,
        );

        account_table.add_record(&account);

        let binding = account_table.data_frame.column("account_id").unwrap().max_reduce().unwrap();
        let actual_last_id = binding.value();
        let expected_last_id = AnyValue::UInt32(0u32);

        assert_eq!(actual_last_id, &expected_last_id)
    }
}