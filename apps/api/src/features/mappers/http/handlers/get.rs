use axum::{
    extract::{Path, State},
    Json,
};

use crate::{app::state::AppState, shared::errors::ApiError};

use super::super::dto::MapperDtoV1;
use super::common::mapper_to_dto;

#[utoipa::path(
    get,
    path = "/mappers/{user}",
    tag = "Mappers::Get",
    params(("user" = String, Path, description = "osu! username")),
    responses(
        (status = 200, description = "Mapper", body = MapperDtoV1),
        (status = 404, description = "Not found", body = crate::shared::errors::ErrorResponse),
        (status = 500, description = "Internal error", body = crate::shared::errors::ErrorResponse)
    ),
    operation_id = "mappers_get_by_username_v1"
)]
pub async fn get_mapper(
    State(state): State<AppState>,
    Path(user): Path<String>,
) -> Result<Json<MapperDtoV1>, ApiError> {
    let mapper = state
        .mappers_repo
        .get_by_username(&user)
        .await
        .map_err(|err| {
            tracing::error!(error = ?err, user = %user, "failed to load mapper");
            ApiError::internal("mappers_get_failed", "Failed to load mapper")
        })?;

    match mapper {
        Some(model) => Ok(Json(mapper_to_dto(model))),
        None => Err(ApiError::not_found("mapper_not_found", "Mapper not found")),
    }
}

#[utoipa::path(
    get,
    path = "/mappers/by-id/{osu_user_id}",
    tag = "Mappers::GetById",
    params(("osu_user_id" = i64, Path, description = "osu! user id")),
    responses(
        (status = 200, description = "Mapper", body = MapperDtoV1),
        (status = 404, description = "Not found", body = crate::shared::errors::ErrorResponse),
        (status = 500, description = "Internal error", body = crate::shared::errors::ErrorResponse)
    ),
    operation_id = "mappers_get_by_id_v1"
)]
pub async fn get_mapper_by_id(
    State(state): State<AppState>,
    Path(osu_user_id): Path<i64>,
) -> Result<Json<MapperDtoV1>, ApiError> {
    let mapper = state
        .mappers_repo
        .get_by_osu_user_id(osu_user_id)
        .await
        .map_err(|err| {
            tracing::error!(error = ?err, osu_user_id, "failed to load mapper");
            ApiError::internal("mappers_get_failed", "Failed to load mapper")
        })?;

    match mapper {
        Some(model) => Ok(Json(mapper_to_dto(model))),
        None => Err(ApiError::not_found("mapper_not_found", "Mapper not found")),
    }
}
