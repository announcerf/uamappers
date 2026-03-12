use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UaMapperDto {
    pub osu_user_id: i64,
    pub username: String,
    pub country_code: String,
    pub first_seen_at: chrono::DateTime<chrono::Utc>,
    pub last_seen_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UaMapperListResponse {
    pub items: Vec<UaMapperDto>,
    pub next_cursor: Option<i64>,
    pub total: u64,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UaMapperListQuery {
    pub cursor: Option<i64>,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UaMapperSearchQuery {
    pub q: String,
    pub cursor: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UaMapperProfileDto {
    pub mapper: MapperDto,
    pub stats: Option<MapperStatsCurrentDto>,
    pub leaderboard_positions: Vec<MapperLeaderboardPositionDto>,
    pub charts: MapperChartsResponseDto,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MapperDto {
    pub bio: MapperBioDto,
    pub tracking: MapperTrackingDto,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MapperBioDto {
    pub osu_user_id: i64,
    pub username: String,
    pub country: String,
    pub country_code: String,
    pub avatar_url: String,
    #[schema(value_type = Object)]
    pub cover: serde_json::Value,
    #[schema(value_type = Object)]
    pub badges: serde_json::Value,
    #[schema(value_type = Object)]
    pub groups: serde_json::Value,
    pub primary_mode: String,
    pub is_bng: bool,
    pub is_nat: bool,
    pub is_gmt: bool,
    pub is_probation_bn: bool,
    pub is_full_bn: bool,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MapperTrackingDto {
    pub cached_at: chrono::DateTime<chrono::Utc>,
    pub first_seen_at: chrono::DateTime<chrono::Utc>,
    pub last_seen_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MapperKudosuDto {
    pub total: i32,
    pub available: i32,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MapperStatsCurrentDto {
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
    pub kudosu: MapperKudosuDto,
    pub has_ranked: bool,
    pub has_loved: bool,
    pub has_guest: bool,
    pub has_nominated: bool,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MapperLeaderboardPositionDto {
    pub leaderboard_key: String,
    pub current_rank: i32,
    pub previous_rank: Option<i32>,
    pub rank_delta: i32,
    pub measured_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MapperChartsPointDto {
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

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MapperChartsResponseDto {
    pub osu_user_id: i64,
    pub points: Vec<MapperChartsPointDto>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum UserBeatmapsetsKindDto {
    Graveyard,
    Guest,
    Loved,
    Nominated,
    Pending,
    Ranked,
}

impl UserBeatmapsetsKindDto {
    pub fn as_str(&self) -> &'static str {
        match self {
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
#[serde(rename_all = "camelCase")]
pub struct BeatmapsetDto {
    pub osu_beatmapset_id: i64,
    pub osu_last_updated: chrono::DateTime<chrono::Utc>,
    pub cached_at: chrono::DateTime<chrono::Utc>,
    #[schema(value_type = Object)]
    pub raw: serde_json::Value,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BeatmapsetListResponse {
    pub items: Vec<BeatmapsetDto>,
    pub limit: u64,
    pub offset: u64,
    pub total: u64,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BeatmapsetListQuery {
    pub kind: UserBeatmapsetsKindDto,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}
