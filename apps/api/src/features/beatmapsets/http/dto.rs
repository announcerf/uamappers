use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BeatmapsetChartsPointDto {
    pub snapshot_week: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub playcount: i64,
    pub favourite_count: i64,
    pub rating: f32,
    pub avg_passcount: f32,
    pub avg_pass_rate: f32,
    pub min_pass_rate: f32,
    pub max_pass_rate: f32,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BeatmapsetChartsResponseDto {
    pub osu_beatmapset_id: i64,
    pub points: Vec<BeatmapsetChartsPointDto>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BeatmapsetDetailsDto {
    pub beatmapset: BeatmapsetHeaderDto,
    pub headline_stats: BeatmapsetHeadlineStatsDto,
    pub difficulty_overview: Vec<BeatmapDifficultyOverviewDto>,
    pub difficulty_details: Vec<BeatmapDifficultyDetailDto>,
    pub analytics: BeatmapsetAnalyticsDto,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BeatmapsetHeaderDto {
    pub osu_beatmapset_id: i64,
    pub title: String,
    pub title_unicode: Option<String>,
    pub artist: String,
    pub artist_unicode: Option<String>,
    pub creator_name: String,
    pub status: String,
    pub cover_url: String,
    pub card_url: String,
    pub submitted_date: Option<chrono::DateTime<chrono::Utc>>,
    pub ranked_date: Option<chrono::DateTime<chrono::Utc>>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub nsfw: bool,
    pub video: bool,
    pub storyboard: bool,
    pub spotlight: bool,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BeatmapsetHeadlineStatsDto {
    pub playcount: i64,
    pub favourite_count: i64,
    pub rating: f32,
    pub fav_play_ratio: f32,
    pub difficulty_count: i32,
    pub avg_pass_rate: f32,
    pub avg_stars: f32,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BeatmapDifficultyOverviewDto {
    pub osu_beatmap_id: i64,
    pub version: String,
    pub mode: String,
    pub stars: f32,
    pub ar: f32,
    pub cs: f32,
    pub od: f32,
    pub hp: f32,
    pub bpm: f32,
    pub seconds_total: i32,
    pub playcount: i64,
    pub passcount: i64,
    pub pass_rate: f32,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BeatmapDifficultyDetailDto {
    pub osu_beatmap_id: i64,
    pub version: String,
    pub mode: String,
    pub stars: f32,
    pub status: String,
    pub ar: f32,
    pub cs: f32,
    pub od: f32,
    pub hp: f32,
    pub bpm: f32,
    pub seconds_total: i32,
    pub seconds_drain: i32,
    pub max_combo: Option<i32>,
    pub playcount: i64,
    pub passcount: i64,
    pub pass_rate: f32,
    pub objects: i32,
    #[schema(value_type = Object)]
    pub owners: serde_json::Value,
    pub count_circles: i32,
    pub count_sliders: i32,
    pub count_spinners: i32,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BeatmapsetAnalyticsDto {
    pub charts: BeatmapsetChartsResponseDto,
}
