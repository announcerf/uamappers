use rosu_v2::model::beatmap::RankStatus;
use rosu_v2::model::GameMode;
use sea_orm::JsonValue;
use serde::Serialize;

pub(super) fn to_json_array<T: Serialize>(value: Option<&T>) -> JsonValue {
    match value {
        Some(value) => serde_json::to_value(value).unwrap_or_else(|_| JsonValue::Array(Vec::new())),
        None => JsonValue::Array(Vec::new()),
    }
}

pub(super) fn offset_to_utc(dt: time::OffsetDateTime) -> chrono::DateTime<chrono::Utc> {
    use chrono::{TimeZone, Utc};

    let secs = dt.unix_timestamp();
    let nanos = dt.nanosecond();

    Utc.timestamp_opt(secs, nanos)
        .single()
        .unwrap_or_else(|| Utc.timestamp_opt(secs, 0).single().unwrap())
}

pub(super) fn mode_to_str(mode: GameMode) -> &'static str {
    match mode {
        GameMode::Osu => "osu",
        GameMode::Taiko => "taiko",
        GameMode::Catch => "catch",
        GameMode::Mania => "mania",
    }
}

pub(super) fn rank_status_to_str(status: RankStatus) -> &'static str {
    match status {
        RankStatus::Graveyard => "graveyard",
        RankStatus::WIP => "wip",
        RankStatus::Pending => "pending",
        RankStatus::Ranked => "ranked",
        RankStatus::Approved => "approved",
        RankStatus::Qualified => "qualified",
        RankStatus::Loved => "loved",
    }
}
