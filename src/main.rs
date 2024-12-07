mod classes;

use classes::financial::*;
use chrono::prelude::*;

fn main() {
    println!("Hello, world!");

    let t1 = Transaction::Expense {
        value: 200.0,
        currency: Currency::EUR,
        date: NaiveDate::from_ymd(2024, 12, 1),
        category: "Utilities".to_string(),
        subcategory: "Electricity".to_string(),
        description: "Monthly electricity bill".to_string(),
        entity_id: 1,
    };

    let t2 = Transaction::Expense {
        value: 100.0,
        currency: Currency::EUR,
        date: NaiveDate::from_ymd(2024, 12, 1),
        category: "Utilities".to_string(),
        subcategory: "Gas".to_string(),
        description: "Monthly gas bill".to_string(),
        entity_id: 1,
    };

    let t3 = Transaction::Debit {
        value: 300.0,
        currency: Currency::EUR,
        date: NaiveDate::from_ymd(2024, 12, 2),
        account_id: 42,
    };

    // Example data
    let items = vec![t1, t2, t3];

    let party: Party = Party {
        transactions: items,
        id: 4
    };

    let validity = party.is_valid();
    println!("Is valid? {validity}");

}
