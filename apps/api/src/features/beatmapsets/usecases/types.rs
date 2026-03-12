use crate::entities::{beatmap_profile, beatmapset_extra, beatmapset_profile};

#[derive(Debug)]
pub struct BeatmapsetCharts {
    pub osu_beatmapset_id: i64,
    pub points: Vec<BeatmapsetChartPoint>,
}

#[derive(Debug)]
pub struct BeatmapsetDetails {
    pub beatmapset: beatmapset_profile::Model,
    pub extra: Option<beatmapset_extra::Model>,
    pub beatmaps: Vec<beatmap_profile::Model>,
    pub charts: Vec<BeatmapsetChartPoint>,
}

#[derive(Debug)]
pub struct BeatmapsetChartPoint {
    pub snapshot_week: chrono::DateTime<chrono::Utc>,
    pub status: i16,
    pub playcount: i64,
    pub favourite_count: i64,
    pub rating: f32,
    pub avg_passcount: f32,
    pub avg_pass_rate: f32,
    pub min_pass_rate: f32,
    pub max_pass_rate: f32,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}
