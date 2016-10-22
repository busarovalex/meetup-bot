use chrono::offset::utc::UTC;
use chrono::TimeZone;

pub type DateTime = ::chrono::datetime::DateTime<UTC>;

pub fn datetime(secs: i64) -> DateTime {
    UTC.timestamp(secs, 0)
}
