use chrono::{DateTime, Utc};

use crate::features::leaderboards::http::dto::LeaderboardKeyDto;

#[derive(Debug)]
pub struct LeaderboardRow {
    pub rank: i32,
    pub previous_rank: Option<i32>,
    pub rank_delta: i32,
    pub osu_user_id: i64,
    pub username: String,
    pub avatar_url: String,
    pub country_code: String,
    pub main_metric_key: String,
    pub main_metric_value: f64,
    pub secondary_metrics: serde_json::Value,
    pub profile_url: String,
}

#[derive(Debug)]
pub struct LeaderboardPage {
    pub leaderboard: LeaderboardKeyDto,
    pub updated_at: DateTime<Utc>,
    pub next_cursor: Option<i32>,
    pub items: Vec<LeaderboardRow>,
}
