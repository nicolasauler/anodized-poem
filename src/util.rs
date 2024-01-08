use chrono::{Datelike, Month, NaiveDate, Utc};

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
    let first_day_of_month = get_first_day_from_month(month);

    if first_day_of_month.month() == 12 {
        return first_day_of_month.with_day(31).unwrap();
    }
    first_day_of_month.with_month(month + 1).unwrap() - chrono::Duration::days(1)
}

pub fn get_last_month() -> Month {
    if Utc::now().month() == 1 {
        return Month::December;
    }
    Month::try_from(u8::try_from(Utc::now().month()).unwrap() - 1).unwrap()
}
