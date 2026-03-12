use sea_orm::JsonValue;

use crate::entities::{beatmapset_extra, beatmapset_profile, ua_mapper};

#[derive(Clone, Copy, Debug)]
pub struct PageInput {
    pub limit: u64,
    pub offset: u64,
}

#[derive(Clone, Copy, Debug)]
pub struct CursorInput {
    pub limit: u64,
    pub after: Option<i64>,
}

#[derive(Debug)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub limit: u64,
    pub offset: u64,
    pub total: u64,
}

#[derive(Debug)]
pub struct BeatmapsetListItem {
    pub profile: beatmapset_profile::Model,
    pub extra: Option<beatmapset_extra::Model>,
}

#[derive(Debug)]
pub struct CursorPage<T> {
    pub items: Vec<T>,
    pub next_cursor: Option<i64>,
    pub total: u64,
}

#[derive(Debug)]
pub struct MapperProfile {
    pub mapper: ua_mapper::Model,
    pub mapper_fingerprint: Option<JsonValue>,
    pub mapper_stats: Option<MapperCurrentStats>,
    pub leaderboard_positions: Vec<MapperLeaderboardPosition>,
    pub charts: Vec<MapperChartPoint>,
}

#[derive(Debug)]
pub struct MapperCharts {
    pub osu_user_id: i64,
    pub points: Vec<MapperChartPoint>,
}

#[derive(Debug)]
pub struct MapperCurrentStats {
    pub total_mapsets: i32,
    pub ranked_mapsets: i32,
    pub loved_mapsets: i32,
    pub guest_mapsets: i32,
    pub nominated_mapsets: i32,
    pub graveyard_mapsets: i32,
    pub pending_mapsets: i32,
    pub total_playcount: i64,
    pub avg_rating: f32,
    pub weighted_rating: f32,
    pub avg_stars: f32,
    pub min_stars: f32,
    pub max_stars: f32,
    pub avg_bpm: f32,
    pub avg_length_seconds: f32,
    pub avg_ar: f32,
    pub avg_cs: f32,
    pub avg_od: f32,
    pub avg_hp: f32,
    pub first_submitted_date: Option<chrono::DateTime<chrono::Utc>>,
    pub first_ranked_date: Option<chrono::DateTime<chrono::Utc>>,
    pub last_mapset_updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub main_mode: String,
    pub mapping_followers: i32,
    pub kudosu: MapperKudosu,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct MapperKudosu {
    pub total: i32,
    pub available: i32,
}

#[derive(Debug)]
pub struct MapperLeaderboardPosition {
    pub leaderboard_key: String,
    pub current_rank: i32,
    pub previous_rank: Option<i32>,
    pub rank_delta: i32,
    pub measured_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct MapperChartPoint {
    pub snapshot_week: chrono::DateTime<chrono::Utc>,
    pub total_mapsets: i32,
    pub ranked_mapsets: i32,
    pub loved_mapsets: i32,
    pub guest_mapsets: i32,
    pub nominated_mapsets: i32,
    pub total_playcount: i64,
    pub avg_rating: f32,
    pub avg_stars: f32,
    pub avg_bpm: f32,
    pub avg_length_seconds: f32,
    pub main_mode: String,
}

pub type MapperPage = CursorPage<ua_mapper::Model>;
pub type BeatmapsetPage = Page<BeatmapsetListItem>;
