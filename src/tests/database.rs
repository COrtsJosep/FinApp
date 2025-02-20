#[cfg(test)]
mod tests {
    use crate::modules::database::summaries::*;
    use crate::modules::database::DataBase;
    use crate::modules::financial::*;
    use crate::tests::test_helpers::init_party;
    use chrono::NaiveDate;
    use polars::df;

    #[test]
    fn correct_insert_party() {
        let mut data_base: DataBase = DataBase::new();
        let mut party: Party = init_party();

        data_base.insert_party(&mut party);

        let expected_result = df!(
                "table" => ["income", "expenses", "funds", "party", "entity", "account"],
                "records" => [1, 1, 2, 1, 1, 1]
        )
        .unwrap();

        let actual_result = data_base.size();

        assert!(actual_result.equals(&expected_result));
    }

    #[test]
    fn correct_insert_entity_account() {
        let mut data_base: DataBase = DataBase::new();
        let entity = Entity::new(
            String::from("Aldi"),
            String::from("Germany"),
            EntityType::Firm,
            String::from("Supermarket"),
        );

        let account = Account::new(
            String::from("Current account"),
            String::from("Credit Suisse"),
            Currency::CHF,
            AccountType::Deposit,
            1080.0f64,
        );

        data_base.insert_entity(&entity);
        data_base.insert_account(&account);

        let expected_result = df!(
                "table" => ["income", "expenses", "funds", "party", "entity", "account"],
                "records" => [0, 0, 0, 0, 2, 2]
        )
        .unwrap();

        let actual_result = data_base.size();

        assert!(actual_result.equals(&expected_result));
    }

    #[test]
    fn last_date_february() {
        let date: NaiveDate = NaiveDate::from_ymd_opt(2025, 2, 4).unwrap();
        let expected_date: NaiveDate = NaiveDate::from_ymd_opt(2025, 2, 28).unwrap();
        let actual_date: NaiveDate = test_last_day_of_month(date);

        assert_eq!(expected_date, actual_date);
    }

    fn last_date_last() {
        let date: NaiveDate = NaiveDate::from_ymd_opt(2025, 1, 31).unwrap();
        let expected_date: NaiveDate = NaiveDate::from_ymd_opt(2025, 1, 31).unwrap();
        let actual_date: NaiveDate = test_last_day_of_month(date);

        assert_eq!(expected_date, actual_date);
    }
}
