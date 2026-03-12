use axum::{
    extract::{Path, State},
    Json,
};

use crate::{app::state::AppState, features::beatmapsets::usecases, shared::errors::ApiError};

use super::dto::{BeatmapsetChartsResponseDto, BeatmapsetDetailsDto};
use super::presenters::{beatmapset_charts_to_dto, beatmapset_details_to_dto};

pub async fn get_beatmapset_charts(
    State(state): State<AppState>,
    Path(osu_beatmapset_id): Path<i64>,
) -> Result<Json<BeatmapsetChartsResponseDto>, ApiError> {
    let charts = usecases::get_beatmapset_charts(
        &state.beatmapset_profiles_repo,
        &state.beatmapset_snapshots_repo,
        osu_beatmapset_id,
    )
    .await
    .map_err(|err| {
        tracing::error!(error = ?err, osu_beatmapset_id, "failed to load beatmapset charts");
        ApiError::internal(
            "beatmapset_charts_get_failed",
            "Failed to load beatmapset charts",
        )
    })?;

    let Some(charts) = charts else {
        return Err(ApiError::not_found(
            "beatmapset_not_found",
            "Beatmapset not found",
        ));
    };

    Ok(Json(beatmapset_charts_to_dto(charts)))
}

pub async fn get_beatmapset_details(
    State(state): State<AppState>,
    Path(osu_beatmapset_id): Path<i64>,
) -> Result<Json<BeatmapsetDetailsDto>, ApiError> {
    let details = usecases::get_beatmapset_details(
        &state.beatmapset_extras_repo,
        &state.beatmapset_profiles_repo,
        &state.beatmap_profiles_repo,
        &state.beatmapset_snapshots_repo,
        osu_beatmapset_id,
    )
    .await
    .map_err(|err| {
        tracing::error!(error = ?err, osu_beatmapset_id, "failed to load beatmapset details");
        ApiError::internal("beatmapset_get_failed", "Failed to load beatmapset")
    })?;

    let Some(details) = details else {
        return Err(ApiError::not_found(
            "beatmapset_not_found",
            "Beatmapset not found",
        ));
    };

    Ok(Json(beatmapset_details_to_dto(details)))
}
