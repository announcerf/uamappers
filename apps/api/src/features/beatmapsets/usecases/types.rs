use crate::entities::{beatmap_profile, beatmapset_profile, beatmapset_snapshot_weekly};

#[derive(Debug)]
pub struct BeatmapsetCharts {
    pub osu_beatmapset_id: i64,
    pub points: Vec<beatmapset_snapshot_weekly::Model>,
}

#[derive(Debug)]
pub struct BeatmapsetDetails {
    pub beatmapset: beatmapset_profile::Model,
    pub beatmaps: Vec<beatmap_profile::Model>,
    pub charts: Vec<beatmapset_snapshot_weekly::Model>,
}
