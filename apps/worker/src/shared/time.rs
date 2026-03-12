use std::time::Duration;

use chrono::{DateTime, Utc};
use chrono_tz::Europe::Kyiv;

pub fn format_duration(duration: Duration) -> String {
    let total_ms: u64 = duration.as_millis().try_into().unwrap_or(u64::MAX);

    let total_seconds = total_ms / 1000;
    let ms = total_ms % 1000;

    let seconds = total_seconds % 60;
    let minutes = (total_seconds / 60) % 60;
    let hours = total_seconds / 3600;

    match (hours, minutes, seconds) {
        (0, 0, s) => format!("{s}s {ms}ms"),
        (0, m, s) => format!("{m}m {s}s"),
        (h, m, s) => format!("{h}h {m}m {s}s"),
    }
}

pub fn format_kyiv(datetime: DateTime<Utc>) -> String {
    datetime
        .with_timezone(&Kyiv)
        .format("%Y-%m-%d %H:%M:%S %Z")
        .to_string()
}
