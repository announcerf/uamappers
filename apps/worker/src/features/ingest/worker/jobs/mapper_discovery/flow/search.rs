fn offset_to_utc(dt: time::OffsetDateTime) -> chrono::DateTime<chrono::Utc> {
    use chrono::{TimeZone, Utc};

    let secs = dt.unix_timestamp();
    let nanos = dt.nanosecond();

    Utc.timestamp_opt(secs, nanos)
        .single()
        .unwrap_or_else(|| Utc.timestamp_opt(secs, 0).single().unwrap())
}

pub(super) fn encode_search_cursor(
    result: &rosu_v2::prelude::BeatmapsetSearchResult,
) -> Option<String> {
    let value = serde_json::to_value(result).ok()?;
    let cursor = value.get("cursor_string")?.as_str()?;
    if cursor.is_empty() {
        return None;
    }
    Some(format!("cursor:{}", cursor))
}

pub(super) fn page_is_before_or_equal_cutoff(
    result: &rosu_v2::prelude::BeatmapsetSearchResult,
    cutoff: chrono::DateTime<chrono::Utc>,
) -> bool {
    let mut min: Option<chrono::DateTime<chrono::Utc>> = None;

    for mapset in &result.mapsets {
        let updated_at = offset_to_utc(mapset.last_updated);
        min = Some(match min {
            Some(current) => current.min(updated_at),
            None => updated_at,
        });
    }

    min.is_some_and(|value| value <= cutoff)
}
