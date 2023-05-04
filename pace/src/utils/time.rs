use chrono::NaiveDateTime;

pub fn to_datetime(time: i64) -> NaiveDateTime {
    return NaiveDateTime::from_timestamp_millis(time).unwrap();
}

pub fn date_str_to_seconds(date: &str) -> i64 {
    let dt = NaiveDateTime::parse_from_str(date, "%d.%m.%Y %H:%M:%S").unwrap();
    return dt.timestamp();
}
