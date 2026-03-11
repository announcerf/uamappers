use std::collections::HashMap;

use chrono::{DateTime, Utc};
use uamappers_api::entities::{leaderboard_position_current, mapper_stats_current};
use uamappers_api::features::mappers::storage::leaderboard_position_current_repo::NewLeaderboardPositionCurrentRow;

pub const LEADERBOARD_KEYS: [&str; 6] = [
    "followers",
    "ranked",
    "guest_diff",
    "plays",
    "kudosu",
    "nominations",
];

pub fn build_leaderboard_rows(
    leaderboard_key: &str,
    measured_at: DateTime<Utc>,
    previous: &HashMap<i64, leaderboard_position_current::Model>,
    stats: &[mapper_stats_current::Model],
) -> Vec<NewLeaderboardPositionCurrentRow> {
    let mut rows = Vec::with_capacity(stats.len());

    for (index, stat) in stats.iter().enumerate() {
        let current_rank = index as i32 + 1;
        let previous_rank = previous.get(&stat.osu_user_id).map(|row| row.current_rank);
        let rank_delta = previous_rank.map(|prev| prev - current_rank).unwrap_or(0);

        rows.push(NewLeaderboardPositionCurrentRow {
            leaderboard_key: leaderboard_key.to_string(),
            osu_user_id: stat.osu_user_id,
            current_rank,
            previous_rank,
            rank_delta,
            measured_at,
        });
    }

    rows
}
