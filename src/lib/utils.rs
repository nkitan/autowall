use std::time::SystemTime;
use chrono::{DateTime, Local};

pub fn get_current_datetime() -> DateTime<Local> {
    let datetime: DateTime<Local> = SystemTime::now().into();
    return datetime;
}