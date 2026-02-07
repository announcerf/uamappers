use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, ToSchema)]
pub struct MapperDtoV1 {
    pub osu_user_id: i64,
    pub username: String,
    pub country_code: String,
    pub kudosu_available: Option<i32>,
    pub kudosu_total: Option<i32>,
    pub count_graveyard: i32,
    pub count_pending: i32,
    pub count_wip: i32,
    pub count_loved: i32,
    pub count_ranked: i32,
    pub count_approved: i32,
    pub count_total: i32,
    pub is_bn: bool,
    pub nominated_count: Option<i32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MapperListResponseV1 {
    pub items: Vec<MapperDtoV1>,
    pub limit: u64,
    pub offset: u64,
    pub total: u64,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct MapperListQuery {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct MapperSearchQuery {
    pub q: String,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MapperBeatmapsetDtoV1 {
    pub osu_beatmapset_id: i64,
    pub creator_osu_user_id: i64,
    pub creator_username: String,
    pub status: String,
    pub artist: String,
    pub title: String,
    pub artist_unicode: Option<String>,
    pub title_unicode: Option<String>,
    pub submitted_date: Option<chrono::DateTime<chrono::Utc>>,
    pub ranked_date: Option<chrono::DateTime<chrono::Utc>>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub play_count: i32,
    pub favourite_count: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MapperBeatmapsetListResponseV1 {
    pub items: Vec<MapperBeatmapsetDtoV1>,
    pub limit: u64,
    pub offset: u64,
    pub total: u64,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct MapperBeatmapsetListQuery {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}
