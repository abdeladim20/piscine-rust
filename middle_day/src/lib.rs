use chrono::{Datelike, NaiveDate};

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    if is_leap_year(year) {
        return None;
    }
    let result = NaiveDate::from_yo_opt(year as i32, 183);
    match result {
        Some(date) => Some(date.weekday()),
        None => None,
    }
}

pub fn is_leap_year(year: u32) -> bool {
    return year % 4 == 0 && (year % 100 != 0 || (year % 100 == 0 && year % 400 == 0));
}