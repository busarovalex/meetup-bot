use chrono::offset::utc::UTC;
use chrono::TimeZone;

pub type DateTime = ::chrono::datetime::DateTime<UTC>;

pub fn datetime(secs: i64) -> DateTime {
    UTC.timestamp(secs, 0)
}

pub fn now() -> DateTime {
    UTC::now()
}

pub fn sub_formatted(from: DateTime, to: DateTime) -> String {
    let duration = to - from;
    format!("{}", duration)
}
