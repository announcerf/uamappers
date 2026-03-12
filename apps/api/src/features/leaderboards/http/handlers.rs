use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::{app::state::AppState, shared::errors::ApiError};

use super::dto::{
    LeaderboardKeyDto, LeaderboardQueryDto, LeaderboardResponseDto, LeaderboardRowDto,
};

pub async fn get_leaderboard(
    State(state): State<AppState>,
    Path(leaderboard): Path<String>,
    Query(query): Query<LeaderboardQueryDto>,
) -> Result<Json<LeaderboardResponseDto>, ApiError> {
    let Some(leaderboard) = LeaderboardKeyDto::parse(&leaderboard) else {
        return Err(ApiError::bad_request(
            "leaderboard_invalid",
            "Unsupported leaderboard",
        ));
    };

    let page = crate::features::leaderboards::usecases::get_leaderboard(
        &state.leaderboard_positions_repo,
        &state.ua_mappers_repo,
        &state.osu_users_repo,
        &state.mapper_stats_repo,
        leaderboard,
        query.cursor,
    )
    .await
    .map_err(|err| {
        tracing::error!(error = ?err, leaderboard = leaderboard.as_str(), "failed to load leaderboard");
        ApiError::internal("leaderboard_get_failed", "Failed to load leaderboard")
    })?;

    Ok(Json(LeaderboardResponseDto {
        leaderboard: page.leaderboard,
        updated_at: page.updated_at,
        next_cursor: page.next_cursor,
        items: page
            .items
            .into_iter()
            .map(|row| LeaderboardRowDto {
                rank: row.rank,
                previous_rank: row.previous_rank,
                rank_delta: row.rank_delta,
                osu_user_id: row.osu_user_id,
                username: row.username,
                avatar_url: row.avatar_url,
                country_code: row.country_code,
                main_metric_key: row.main_metric_key,
                main_metric_value: row.main_metric_value,
                secondary_metrics: row.secondary_metrics,
                profile_url: row.profile_url,
            })
            .collect(),
    }))
}
