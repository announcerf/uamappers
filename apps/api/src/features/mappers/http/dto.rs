use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, ToSchema)]
pub struct MapperDtoV1 {
    pub osu_user_id: i64,
    pub username: String,
    pub country_code: String,
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
