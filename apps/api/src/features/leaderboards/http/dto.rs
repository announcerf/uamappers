use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum LeaderboardKeyDto {
    Followers,
    Ranked,
    GuestDiff,
    Plays,
    Kudosu,
    Nominations,
}

impl LeaderboardKeyDto {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "followers" => Some(Self::Followers),
            "ranked" => Some(Self::Ranked),
            "guestDiff" => Some(Self::GuestDiff),
            "plays" => Some(Self::Plays),
            "kudosu" => Some(Self::Kudosu),
            "nominations" => Some(Self::Nominations),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Followers => "followers",
            Self::Ranked => "ranked",
            Self::GuestDiff => "guest_diff",
            Self::Plays => "plays",
            Self::Kudosu => "kudosu",
            Self::Nominations => "nominations",
        }
    }
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardQueryDto {
    pub cursor: Option<i32>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardRowDto {
    pub rank: i32,
    pub previous_rank: Option<i32>,
    pub rank_delta: i32,
    pub osu_user_id: i64,
    pub username: String,
    pub avatar_url: String,
    pub country_code: String,
    pub main_metric_key: String,
    pub main_metric_value: f64,
    #[schema(value_type = Object)]
    pub secondary_metrics: serde_json::Value,
    pub profile_url: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LeaderboardResponseDto {
    pub leaderboard: LeaderboardKeyDto,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub next_cursor: Option<i32>,
    pub items: Vec<LeaderboardRowDto>,
}
