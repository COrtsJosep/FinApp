use chrono::NaiveDate;
use crate::modules::financial::{Currency, Party, Transaction};

pub(crate) fn init_party() -> Party {
    let t1 = Transaction::Income {
        value: 120.0,
        currency: Currency::EUR,
        date: NaiveDate::from_ymd_opt(2024, 12, 1).unwrap(),
        category: "Salary".to_string(),
        subcategory: "Regular salary".to_string(),
        description: "Finally got the bread".to_string(),
        entity_id: 0,
    };

    let t2 = Transaction::Expense {
        value: 100.0,
        currency: Currency::SEK,
        date: NaiveDate::from_ymd_opt(2024, 12, 1).unwrap(),
        category: "Drugs".to_string(),
        subcategory: "Alcohol".to_string(),
        description: "Bought some beers to celebrate".to_string(),
        entity_id: 1,
    };

    let t3 = Transaction::Credit {
        value: 120.0,
        currency: Currency::EUR,
        date: NaiveDate::from_ymd_opt(2024, 12, 2).unwrap(),
        account_id: 0,
    };

    let t4 = Transaction::Debit {
        value: 100.0,
        currency: Currency::SEK,
        date: NaiveDate::from_ymd_opt(2024, 12, 2).unwrap(),
        account_id: 1,
    };

    // Example data
    let items = vec![t1, t2, t3, t4];

    Party::new(items)
}