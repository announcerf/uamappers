use sea_orm::JsonValue;

use crate::entities::{
    beatmapset, leaderboard_position_current, mapper_aggregate_snapshot_weekly, mapper_stats_current,
    ua_mapper,
};

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
pub struct CursorPage<T> {
    pub items: Vec<T>,
    pub next_cursor: Option<i64>,
    pub total: u64,
}

#[derive(Debug)]
pub struct MapperProfile {
    pub mapper: ua_mapper::Model,
    pub mapper_fingerprint: Option<JsonValue>,
    pub mapper_stats: Option<mapper_stats_current::Model>,
    pub leaderboard_positions: Vec<leaderboard_position_current::Model>,
    pub charts: Vec<mapper_aggregate_snapshot_weekly::Model>,
}

#[derive(Debug)]
pub struct MapperCharts {
    pub osu_user_id: i64,
    pub points: Vec<mapper_aggregate_snapshot_weekly::Model>,
}

pub type MapperPage = CursorPage<ua_mapper::Model>;
pub type BeatmapsetPage = Page<beatmapset::Model>;
