use chrono::NaiveDateTime;

pub fn to_datetime(time: i64) -> NaiveDateTime {
    return NaiveDateTime::from_timestamp_millis(time).unwrap();
}
