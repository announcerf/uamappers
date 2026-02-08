use rosu_v2::model::beatmap::BeatmapsetExtended;

use uamappers_api::features::mappers::storage::beatmapset_repo::NewBeatmapsetRow;

use super::raw::strip_top_level_id;

pub fn mapset_to_row(mapset: &BeatmapsetExtended) -> NewBeatmapsetRow {
    let last_updated = offset_to_utc(mapset.last_updated);
    let raw = serde_json::to_value(mapset)
        .map(strip_top_level_id)
        .unwrap_or(serde_json::Value::Null);

    NewBeatmapsetRow {
        osu_beatmapset_id: mapset.mapset_id as i64,
        last_updated,
        raw,
    }
}

fn offset_to_utc(dt: time::OffsetDateTime) -> chrono::DateTime<chrono::Utc> {
    use chrono::{TimeZone, Utc};
    let secs = dt.unix_timestamp();
    let nanos: u32 = dt.nanosecond();

    Utc.timestamp_opt(secs, nanos)
        .single()
        .unwrap_or_else(|| Utc.timestamp_opt(secs, 0).single().unwrap())
}
