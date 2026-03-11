use crate::shared::errors::ErrorResponse;

use super::{LeaderboardKeyDto, LeaderboardQueryDto, LeaderboardResponseDto};

#[utoipa::path(
    get,
    path = "/leaderboards/{leaderboard}",
    tag = "Leaderboards::Get",
    summary = "Get leaderboard rows",
    params(
        ("leaderboard" = LeaderboardKeyDto, Path, description = "Leaderboard key"),
        LeaderboardQueryDto
    ),
    responses(
        (status = 200, description = "Leaderboard rows", body = LeaderboardResponseDto),
        (status = 400, description = "Invalid leaderboard", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    operation_id = "leaderboards_get"
)]
pub async fn get_leaderboard() {}
