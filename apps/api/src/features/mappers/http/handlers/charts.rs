use axum::{
    extract::{Path, State},
    Json,
};

use crate::{app::state::AppState, features::mappers::usecases, shared::errors::ApiError};

use super::super::dto::MapperChartsResponseDto;
use super::super::presenters::mapper_charts_to_dto;

pub async fn get_mapper_charts(
    State(state): State<AppState>,
    Path(user): Path<String>,
) -> Result<Json<MapperChartsResponseDto>, ApiError> {
    let charts = usecases::get_mapper_charts_by_username(
        &state.ua_mappers_repo,
        &state.mapper_aggregate_snapshots_repo,
        &user,
    )
    .await
    .map_err(|err| {
        tracing::error!(error = ?err, user = %user, "failed to load mapper charts");
        ApiError::internal("mappers_charts_get_failed", "Failed to load mapper charts")
    })?;

    let Some(charts) = charts else {
        return Err(ApiError::not_found("mapper_not_found", "Mapper not found"));
    };

    Ok(Json(mapper_charts_to_dto(charts)))
}

pub async fn get_mapper_charts_by_id(
    State(state): State<AppState>,
    Path(osu_user_id): Path<i64>,
) -> Result<Json<MapperChartsResponseDto>, ApiError> {
    let charts = usecases::get_mapper_charts_by_id(
        &state.ua_mappers_repo,
        &state.mapper_aggregate_snapshots_repo,
        osu_user_id,
    )
    .await
    .map_err(|err| {
        tracing::error!(error = ?err, osu_user_id, "failed to load mapper charts");
        ApiError::internal("mappers_charts_get_failed", "Failed to load mapper charts")
    })?;

    let Some(charts) = charts else {
        return Err(ApiError::not_found("mapper_not_found", "Mapper not found"));
    };

    Ok(Json(mapper_charts_to_dto(charts)))
}
