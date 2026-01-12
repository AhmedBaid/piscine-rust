use chrono::{Datelike, NaiveDate};
pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
        return None;
    }
    let specefioed_date = NaiveDate::from_yo_opt(year as i32, 183).unwrap();
    Some(specefioed_date.weekday())
}
