use axum::{
    Json,
    extract::{Path, State},
};

use crate::{app::state::AppState, features::mappers::usecases, shared::errors::ApiError};

use super::super::dto::UaMapperProfileDtoV1;
use super::common::mapper_profile_to_dto;

pub async fn get_mapper(
    State(state): State<AppState>,
    Path(user): Path<String>,
) -> Result<Json<UaMapperProfileDtoV1>, ApiError> {
    let profile = usecases::get_mapper_profile_by_username(
        &state.ua_mappers_repo,
        &state.osu_users_repo,
        &user,
    )
    .await
    .map_err(|err| {
        tracing::error!(error = ?err, user = %user, "failed to load mapper profile");
        ApiError::internal("mappers_get_failed", "Failed to load mapper profile")
    })?;

    let Some(profile) = profile else {
        return Err(ApiError::not_found("mapper_not_found", "Mapper not found"));
    };

    Ok(Json(mapper_profile_to_dto(profile)))
}

pub async fn get_mapper_by_id(
    State(state): State<AppState>,
    Path(osu_user_id): Path<i64>,
) -> Result<Json<UaMapperProfileDtoV1>, ApiError> {
    let profile = usecases::get_mapper_profile_by_id(
        &state.ua_mappers_repo,
        &state.osu_users_repo,
        osu_user_id,
    )
    .await
    .map_err(|err| {
        tracing::error!(error = ?err, osu_user_id, "failed to load mapper profile");
        ApiError::internal("mappers_get_failed", "Failed to load mapper profile")
    })?;

    let Some(profile) = profile else {
        return Err(ApiError::not_found("mapper_not_found", "Mapper not found"));
    };

    Ok(Json(mapper_profile_to_dto(profile)))
}
