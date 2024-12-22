#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use crate::modules::financial::*;
    use crate::tests::test_helpers::init_party;

    #[test]
    fn correct_party() {
        let party: Party = init_party();

        assert!(party.is_valid());
    }

    #[test]
    fn incorrect_party() {
        let t1 = Transaction::Expense {
            value: 102.0,
            currency: Currency::EUR,
            date: NaiveDate::from_ymd_opt(2024, 12, 1).unwrap(),
            category: "Utilities".to_string(),
            subcategory: "Electricity".to_string(),
            description: "Monthly electricity bill".to_string(),
            entity_id: 1,
        };

        let t2 = Transaction::Debit {
            value: 120.0,
            currency: Currency::EUR,
            date: NaiveDate::from_ymd_opt(2024, 12, 2).unwrap(),
            account_id: 42,
        };

        // Example data
        let items = vec![t1, t2];

        let party: Party = Party::new(items);

        assert!(!party.is_valid());
    }

    #[test]
    fn multicurrency_party() {
        let party: Party = init_party();

        assert!(party.is_valid());
    }
}