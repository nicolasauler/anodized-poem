use chrono::{Datelike, NaiveDate, Utc};

pub fn get_first_day_from_month(month: u32) -> NaiveDate {
    Utc::now()
        .with_day(1)
        .unwrap()
        .naive_utc()
        .date()
        .with_month(month)
        .unwrap()
}

pub fn get_last_day_from_month(month: u32) -> NaiveDate {
    Utc::now()
        .with_day(1)
        .unwrap()
        .naive_utc()
        .date()
        .with_day(1)
        .unwrap()
        .with_month(month + 1)
        .unwrap()
        - chrono::Duration::days(1)
}
