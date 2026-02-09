use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, ToSchema)]
pub struct UaMapperDtoV1 {
    pub osu_user_id: i64,
    pub username: String,
    pub country_code: String,
    pub first_seen_at: chrono::DateTime<chrono::Utc>,
    pub last_seen_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UaMapperListResponseV1 {
    pub items: Vec<UaMapperDtoV1>,
    pub limit: u64,
    pub offset: u64,
    pub total: u64,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct UaMapperListQuery {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct UaMapperSearchQuery {
    pub q: String,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UaMapperProfileDtoV1 {
    pub mapper: UaMapperDtoV1,
    #[schema(value_type = Object)]
    pub user: Option<serde_json::Value>,
    pub user_fetched_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum UserBeatmapsetsKindDtoV1 {
    Favourite,
    Graveyard,
    Guest,
    Loved,
    Nominated,
    Pending,
    Ranked,
}

impl UserBeatmapsetsKindDtoV1 {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Favourite => "favourite",
            Self::Graveyard => "graveyard",
            Self::Guest => "guest",
            Self::Loved => "loved",
            Self::Nominated => "nominated",
            Self::Pending => "pending",
            Self::Ranked => "ranked",
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BeatmapsetDtoV1 {
    pub osu_beatmapset_id: i64,
    pub osu_last_updated: chrono::DateTime<chrono::Utc>,
    pub cached_at: chrono::DateTime<chrono::Utc>,
    #[schema(value_type = Object)]
    pub raw: serde_json::Value,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BeatmapsetListResponseV1 {
    pub items: Vec<BeatmapsetDtoV1>,
    pub limit: u64,
    pub offset: u64,
    pub total: u64,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct BeatmapsetListQuery {
    pub kind: UserBeatmapsetsKindDtoV1,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}
