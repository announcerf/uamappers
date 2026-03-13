use rosu_v2::model::beatmap::{BeatmapExtended, BeatmapsetExtended};
use rosu_v2::model::user::UserBeatmapsetsKind;
use uamappers_api::features::mappers::storage::codes::{
    genre_code, language_code, mode_code, status_code,
};
use uamappers_api::features::mappers::storage::{
    beatmap_profile_repo::NewBeatmapProfileRow, beatmapset_extra_repo::NewBeatmapsetExtraRow,
    beatmapset_profile_repo::NewBeatmapsetProfileRow,
    beatmapset_snapshot_weekly_repo::NewBeatmapsetSnapshotWeeklyRow,
};

use super::shared::{
    genre_to_str, language_to_str, mode_to_str, offset_to_utc, rank_status_to_str, to_json_array,
};
use crate::features::ingest::worker::jobs::mapper_enrich::snapshot::mapset_to_snapshot_row;

pub struct BeatmapsetsPersistPage {
    pub beatmapset_extras: Vec<NewBeatmapsetExtraRow>,
    pub beatmapset_profiles: Vec<NewBeatmapsetProfileRow>,
    pub beatmapset_snapshots: Vec<NewBeatmapsetSnapshotWeeklyRow>,
    pub beatmap_profiles: Vec<NewBeatmapProfileRow>,
    pub beatmap_ids_by_mapset: Vec<(i64, Vec<i64>)>,
    pub beatmapset_ids: Vec<i64>,
}

pub struct PersistedBeatmapset {
    pub mapset: BeatmapsetExtended,
    pub details_unavailable: bool,
}

pub fn kind_to_str(kind: UserBeatmapsetsKind) -> &'static str {
    match kind {
        UserBeatmapsetsKind::Graveyard => "graveyard",
        UserBeatmapsetsKind::Guest => "guest",
        UserBeatmapsetsKind::Loved => "loved",
        UserBeatmapsetsKind::Nominated => "nominated",
        UserBeatmapsetsKind::Pending => "pending",
        UserBeatmapsetsKind::Ranked => "ranked",
        _ => unreachable!("unsupported beatmapset kind"),
    }
}

pub fn build_page_payload(page: &[PersistedBeatmapset]) -> BeatmapsetsPersistPage {
    let mut beatmapset_extras = Vec::new();
    let mut beatmapset_profiles = Vec::new();
    let mut beatmapset_snapshots = Vec::new();
    let mut beatmap_profiles = Vec::new();
    let mut beatmap_ids_by_mapset = Vec::new();
    let mut beatmapset_ids = Vec::new();
    let cached_at = chrono::Utc::now();
    let weekly_snapshot =
        crate::features::ingest::worker::jobs::mapper_enrich::snapshot::snapshot_week(cached_at);

    for item in page {
        let mapset = &item.mapset;
        beatmapset_extras.push(mapset_to_extra_row(mapset, item.details_unavailable));
        beatmapset_ids.push(mapset.mapset_id as i64);
        beatmapset_profiles.push(mapset_to_profile_row(mapset, cached_at));
        beatmapset_snapshots.push(mapset_to_snapshot_row(mapset, weekly_snapshot));

        let nested_rows = maps_to_profile_rows(mapset, cached_at);
        let keep_ids = nested_rows.iter().map(|row| row.osu_beatmap_id).collect();
        beatmap_ids_by_mapset.push((mapset.mapset_id as i64, keep_ids));
        beatmap_profiles.extend(nested_rows);
    }

    BeatmapsetsPersistPage {
        beatmapset_extras,
        beatmapset_profiles,
        beatmapset_snapshots,
        beatmap_profiles,
        beatmap_ids_by_mapset,
        beatmapset_ids,
    }
}

pub fn mapset_to_extra_row(
    mapset: &BeatmapsetExtended,
    details_unavailable: bool,
) -> NewBeatmapsetExtraRow {
    NewBeatmapsetExtraRow {
        osu_beatmapset_id: mapset.mapset_id as i64,
        creator_id: mapset.creator_id as i64,
        creator_name: mapset.creator_name.to_string(),
        anime_cover: None,
        details_unavailable,
    }
}

pub fn mapset_to_profile_row(
    mapset: &BeatmapsetExtended,
    cached_at: chrono::DateTime<chrono::Utc>,
) -> NewBeatmapsetProfileRow {
    NewBeatmapsetProfileRow {
        osu_beatmapset_id: mapset.mapset_id as i64,
        artist: mapset.artist.clone(),
        artist_unicode: mapset.artist_unicode.clone(),
        title: mapset.title.clone(),
        title_unicode: mapset.title_unicode.clone(),
        source: mapset.source.clone(),
        tags: mapset.tags.clone(),
        genre: mapset
            .genre
            .map(genre_to_str)
            .map(genre_code)
            .unwrap_or_else(|| genre_code("unspecified")),
        language: mapset
            .language
            .map(language_to_str)
            .map(language_code)
            .unwrap_or_else(|| language_code("unspecified")),
        status: status_code(rank_status_to_str(mapset.status)),
        submitted_date: mapset.submitted_date.map(offset_to_utc),
        ranked_date: mapset.ranked_date.map(offset_to_utc),
        last_updated: offset_to_utc(mapset.last_updated),
        nsfw: mapset.nsfw,
        video: mapset.video,
        storyboard: mapset.storyboard,
        spotlight: mapset.spotlight,
        playcount: mapset.playcount as i64,
        favourite_count: mapset.favourite_count as i64,
        rating: mapset.rating,
        cover_url: mapset.covers.cover.clone(),
        card_url: mapset.covers.card.clone(),
        bpm: mapset.bpm,
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
        version: map.version.clone(),
        mode: mode_code(mode_to_str(map.mode)),
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
        status: status_code(rank_status_to_str(map.status)),
        last_updated: offset_to_utc(map.last_updated),
        cached_at,
    }
}
