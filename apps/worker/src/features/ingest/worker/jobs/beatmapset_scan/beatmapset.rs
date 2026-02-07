use rosu_v2::model::beatmap::BeatmapsetExtended;
use rosu_v2::prelude::RankStatus;

use uamappers_api::features::mappers::storage::beatmapset_repo::NewBeatmapsetRow;

pub fn mapset_to_row(mapset: &BeatmapsetExtended) -> NewBeatmapsetRow {
    let submitted_date = mapset.submitted_date.map(offset_to_utc);
    let ranked_date = mapset.ranked_date.map(offset_to_utc);
    let last_updated = offset_to_utc(mapset.last_updated);

    NewBeatmapsetRow {
        osu_beatmapset_id: mapset.mapset_id as i64,
        creator_osu_user_id: mapset.creator_id as i64,
        creator_username: mapset.creator_name.to_string(),
        status: rank_status_to_string(mapset.status),
        artist: mapset.artist.to_string(),
        title: mapset.title.to_string(),
        artist_unicode: mapset.artist_unicode.clone(),
        title_unicode: mapset.title_unicode.clone(),
        submitted_date,
        ranked_date,
        last_updated,
        play_count: mapset.playcount as i32,
        favourite_count: mapset.favourite_count as i32,
        raw: serde_json::to_value(mapset).unwrap_or(serde_json::Value::Null),
    }
}

fn rank_status_to_string(status: RankStatus) -> String {
    match status {
        RankStatus::Graveyard => "graveyard".to_string(),
        RankStatus::Pending => "pending".to_string(),
        RankStatus::WIP => "wip".to_string(),
        RankStatus::Loved => "loved".to_string(),
        RankStatus::Ranked => "ranked".to_string(),
        RankStatus::Approved => "approved".to_string(),
        RankStatus::Qualified => "qualified".to_string(),
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
