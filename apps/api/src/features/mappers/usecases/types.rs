use chrono::{DateTime, Utc};
use sea_orm::JsonValue;

use crate::entities::{beatmapset, ua_mapper};

#[derive(Clone, Copy, Debug)]
pub struct PageInput {
    pub limit: u64,
    pub offset: u64,
}

#[derive(Debug)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub limit: u64,
    pub offset: u64,
    pub total: u64,
}

#[derive(Debug)]
pub struct MapperProfile {
    pub mapper: ua_mapper::Model,
    pub user_fetched_at: Option<DateTime<Utc>>,
    pub user_raw: Option<JsonValue>,
}

pub type MapperPage = Page<ua_mapper::Model>;
pub type BeatmapsetPage = Page<beatmapset::Model>;
