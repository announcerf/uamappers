use rosu_v2::model::beatmap::{Genre, Language, RankStatus};
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

pub(super) fn genre_to_str(genre: Genre) -> &'static str {
    match genre {
        Genre::Any => "any",
        Genre::Unspecified => "unspecified",
        Genre::VideoGame => "video_game",
        Genre::Anime => "anime",
        Genre::Rock => "rock",
        Genre::Pop => "pop",
        Genre::Other => "other",
        Genre::Novelty => "novelty",
        Genre::HipHop => "hip_hop",
        Genre::Electronic => "electronic",
        Genre::Metal => "metal",
        Genre::Classical => "classical",
        Genre::Folk => "folk",
        Genre::Jazz => "jazz",
    }
}

pub(super) fn language_to_str(language: Language) -> &'static str {
    match language {
        Language::Any => "any",
        Language::Other => "other",
        Language::English => "english",
        Language::Japanese => "japanese",
        Language::Chinese => "chinese",
        Language::Instrumental => "instrumental",
        Language::Korean => "korean",
        Language::French => "french",
        Language::German => "german",
        Language::Swedish => "swedish",
        Language::Spanish => "spanish",
        Language::Italian => "italian",
        Language::Russian => "russian",
        Language::Polish => "polish",
        Language::Unspecified => "unspecified",
    }
}
