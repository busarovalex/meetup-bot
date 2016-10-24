use chrono::offset::utc::UTC;
use chrono::TimeZone;

pub type DateTime = ::chrono::datetime::DateTime<UTC>;

pub fn datetime(secs: i64) -> DateTime {
    UTC.timestamp(secs, 0)
}

pub fn now() -> DateTime {
    UTC::now()
}

pub fn format_remaining_time(meetup_time: DateTime) -> (Option<String>, String) {
    let now = now();
    let duration = meetup_time - now;

    if duration.num_hours() > 0 {
        let format = if duration.num_days() > 0 {
            "%Y-%m-%d %H:%M:%S"
        } else {
            "%H:%M:%S"
        };
        return (Some(format!("{}", meetup_time.format(format))), format!("{} часов {} минут", duration.num_hours(), duration.num_minutes() % 60));
    }
    (None, format!("{} минут {} секунд {} миллисекунд", duration.num_minutes(), duration.num_seconds() % 60, duration.num_milliseconds() % 1000))
}
