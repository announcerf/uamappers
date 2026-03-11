use rosu_v2::model::beatmap::{BeatmapExtended, BeatmapsetExtended, Genre, Language, RankStatus};
use rosu_v2::model::user::UserExtended;
use rosu_v2::model::GameMode;
use sea_orm::JsonValue;
use serde::Serialize;
use uamappers_api::features::mappers::storage::{
    beatmap_profile_repo::NewBeatmapProfileRow, beatmapset_profile_repo::NewBeatmapsetProfileRow,
    mapper_profile_repo::NewMapperProfileRow,
};

pub fn user_to_mapper_profile_row(
    user: &UserExtended,
    cached_at: chrono::DateTime<chrono::Utc>,
) -> NewMapperProfileRow {
    NewMapperProfileRow {
        osu_user_id: user.user_id as i64,
        username: user.username.to_string(),
        avatar_url: user.avatar_url.clone(),
        country: user.country.clone(),
        country_code: user.country_code.to_string(),
        cover_url: user.cover.url.clone(),
        primary_mode: mode_to_str(user.mode).to_string(),
        join_date: offset_to_utc(user.join_date),
        last_visit: user.last_visit.map(offset_to_utc),
        mapping_followers: user.mapping_follower_count.unwrap_or(0) as i32,
        kudosu_available: user.kudosu.available,
        kudosu_total: user.kudosu.total,
        badges_json: to_json_array(user.badges.as_ref()),
        groups_json: to_json_array(user.groups.as_ref()),
        is_bng: user.is_bng.unwrap_or(false),
        is_nat: user.is_nat.unwrap_or(false),
        is_gmt: user.is_gmt.unwrap_or(false),
        is_limited_bn: user.is_limited_bn.unwrap_or(false),
        is_full_bn: user.is_full_bn.unwrap_or(false),
        cached_at,
    }
}

pub fn mapset_to_profile_row(
    mapset: &BeatmapsetExtended,
    cached_at: chrono::DateTime<chrono::Utc>,
) -> NewBeatmapsetProfileRow {
    let hype_current = mapset.hype.map(|h| h.current as i32).unwrap_or_default();
    let hype_required = mapset.hype.map(|h| h.required as i32).unwrap_or_default();
    let difficulty_count = mapset
        .maps
        .as_ref()
        .map(|maps| maps.len() as i32)
        .unwrap_or(0);

    NewBeatmapsetProfileRow {
        osu_beatmapset_id: mapset.mapset_id as i64,
        creator_id: mapset.creator_id as i64,
        creator_name: mapset.creator_name.to_string(),
        artist: mapset.artist.clone(),
        artist_unicode: mapset.artist_unicode.clone(),
        title: mapset.title.clone(),
        title_unicode: mapset.title_unicode.clone(),
        source: mapset.source.clone(),
        tags: mapset.tags.clone(),
        genre: mapset.genre.map(genre_to_str).map(str::to_string),
        language: mapset.language.map(language_to_str).map(str::to_string),
        status: rank_status_to_str(mapset.status).to_string(),
        submitted_date: mapset.submitted_date.map(offset_to_utc),
        ranked_date: mapset.ranked_date.map(offset_to_utc),
        last_updated: offset_to_utc(mapset.last_updated),
        discussion_enabled: mapset.discussion_enabled,
        discussion_locked: mapset.discussion_locked,
        can_be_hyped: mapset.can_be_hyped,
        is_scoreable: mapset.is_scoreable,
        download_disabled: mapset.availability.download_disabled,
        nsfw: mapset.nsfw,
        video: mapset.video,
        storyboard: mapset.storyboard,
        spotlight: mapset.spotlight,
        playcount: mapset.playcount as i64,
        favourite_count: mapset.favourite_count as i64,
        rating: mapset.rating,
        hype_current,
        hype_required,
        nominations_current: mapset.nominations_summary.current as i32,
        cover_url: mapset.covers.cover.clone(),
        card_url: mapset.covers.card.clone(),
        preview_url: mapset.preview_url.clone(),
        bpm: mapset.bpm,
        difficulty_count,
        cached_at,
    }
}

pub fn maps_to_profile_rows(
    mapset: &BeatmapsetExtended,
    cached_at: chrono::DateTime<chrono::Utc>,
) -> Vec<NewBeatmapProfileRow> {
    let Some(maps) = mapset.maps.as_ref() else {
        return Vec::new();
    };

    maps.iter()
        .map(|map| map_to_profile_row(map, cached_at))
        .collect()
}

fn map_to_profile_row(
    map: &BeatmapExtended,
    cached_at: chrono::DateTime<chrono::Utc>,
) -> NewBeatmapProfileRow {
    NewBeatmapProfileRow {
        osu_beatmap_id: map.map_id as i64,
        osu_beatmapset_id: map.mapset_id as i64,
        creator_id: map.creator_id as i64,
        version: map.version.clone(),
        mode: mode_to_str(map.mode).to_string(),
        stars: map.stars,
        ar: map.ar,
        cs: map.cs,
        od: map.od,
        hp: map.hp,
        bpm: map.bpm,
        seconds_total: map.seconds_total as i32,
        seconds_drain: map.seconds_drain as i32,
        max_combo: map.max_combo.map(|value| value as i32),
        playcount: map.playcount as i64,
        passcount: map.passcount as i64,
        count_circles: map.count_circles as i32,
        count_sliders: map.count_sliders as i32,
        count_spinners: map.count_spinners as i32,
        owners_json: to_json_array(map.owners.as_ref()),
        status: rank_status_to_str(map.status).to_string(),
        is_scoreable: map.is_scoreable,
        last_updated: offset_to_utc(map.last_updated),
        cached_at,
    }
}

fn to_json_array<T: Serialize>(value: Option<&T>) -> JsonValue {
    match value {
        Some(value) => serde_json::to_value(value).unwrap_or_else(|_| JsonValue::Array(Vec::new())),
        None => JsonValue::Array(Vec::new()),
    }
}

fn offset_to_utc(dt: time::OffsetDateTime) -> chrono::DateTime<chrono::Utc> {
    use chrono::{TimeZone, Utc};

    let secs = dt.unix_timestamp();
    let nanos = dt.nanosecond();

    Utc.timestamp_opt(secs, nanos)
        .single()
        .unwrap_or_else(|| Utc.timestamp_opt(secs, 0).single().unwrap())
}

fn mode_to_str(mode: GameMode) -> &'static str {
    match mode {
        GameMode::Osu => "osu",
        GameMode::Taiko => "taiko",
        GameMode::Catch => "catch",
        GameMode::Mania => "mania",
    }
}

fn rank_status_to_str(status: RankStatus) -> &'static str {
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

fn genre_to_str(genre: Genre) -> &'static str {
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

fn language_to_str(language: Language) -> &'static str {
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
