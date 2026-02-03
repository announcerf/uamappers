use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::{app::state::AppState, entities::mapper, shared::errors::ApiError};

use super::dto::{MapperDtoV1, MapperListQuery, MapperListResponseV1, MapperSearchQuery};

const DEFAULT_LIMIT: u64 = 50;
const MAX_LIMIT: u64 = 200;

fn clamp_limit(limit: Option<u64>) -> u64 {
    let limit = limit.unwrap_or(DEFAULT_LIMIT);
    limit.clamp(1, MAX_LIMIT)
}

fn to_dto(model: mapper::Model) -> MapperDtoV1 {
    MapperDtoV1 {
        osu_user_id: model.osu_user_id,
        username: model.username,
        country_code: model.country_code,
        count_graveyard: model.count_graveyard,
        count_pending: model.count_pending,
        count_wip: model.count_wip,
        count_loved: model.count_loved,
        count_ranked: model.count_ranked,
        count_approved: model.count_approved,
        count_total: model.count_total,
        is_bn: model.is_bn,
        nominated_count: model.nominated_count,
    }
}

#[utoipa::path(
    get,
    path = "/mappers",
    tag = "Mappers::List",
    params(MapperListQuery),
    responses(
        (status = 200, description = "List UA mappers", body = MapperListResponseV1),
        (status = 500, description = "Internal error", body = crate::shared::errors::ErrorResponse)
    ),
    operation_id = "mappers_list_v1"
)]
pub async fn list_mappers(
    State(state): State<AppState>,
    Query(query): Query<MapperListQuery>,
) -> Result<Json<MapperListResponseV1>, ApiError> {
    let limit = clamp_limit(query.limit);
    let offset = query.offset.unwrap_or(0);

    let (rows, total) = state
        .mappers_repo
        .list_ua(limit, offset)
        .await
        .map_err(|err| {
            tracing::error!(error = ?err, "failed to list mappers");
            ApiError::internal("mappers_list_failed", "Failed to list mappers")
        })?;

    let items = rows.into_iter().map(to_dto).collect();

    Ok(Json(MapperListResponseV1 {
        items,
        limit,
        offset,
        total,
    }))
}

#[utoipa::path(
    get,
    path = "/mappers/{osu_user_id}",
    tag = "Mappers::Get",
    params(("osu_user_id" = i64, Path, description = "osu! user id")),
    responses(
        (status = 200, description = "Mapper", body = MapperDtoV1),
        (status = 404, description = "Not found", body = crate::shared::errors::ErrorResponse),
        (status = 500, description = "Internal error", body = crate::shared::errors::ErrorResponse)
    ),
    operation_id = "mappers_get_v1"
)]
pub async fn get_mapper(
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
        Some(model) => Ok(Json(to_dto(model))),
        None => Err(ApiError::not_found("mapper_not_found", "Mapper not found")),
    }
}

#[utoipa::path(
    get,
    path = "/mappers/search",
    tag = "Mappers::Search",
    params(MapperSearchQuery),
    responses(
        (status = 200, description = "Search UA mappers", body = MapperListResponseV1),
        (status = 400, description = "Invalid query", body = crate::shared::errors::ErrorResponse),
        (status = 500, description = "Internal error", body = crate::shared::errors::ErrorResponse)
    ),
    operation_id = "mappers_search_v1"
)]
pub async fn search_mappers(
    State(state): State<AppState>,
    Query(query): Query<MapperSearchQuery>,
) -> Result<Json<MapperListResponseV1>, ApiError> {
    if query.q.trim().is_empty() {
        return Err(
            ApiError::bad_request("query_required", "Search query is required")
                .with_field("q", "Query cannot be empty"),
        );
    }

    let limit = clamp_limit(query.limit);
    let offset = query.offset.unwrap_or(0);

    let (rows, total) = state
        .mappers_repo
        .search_ua(&query.q, limit, offset)
        .await
        .map_err(|err| {
            tracing::error!(error = ?err, query = %query.q, "failed to search mappers");
            ApiError::internal("mappers_search_failed", "Failed to search mappers")
        })?;

    let items = rows.into_iter().map(to_dto).collect();

    Ok(Json(MapperListResponseV1 {
        items,
        limit,
        offset,
        total,
    }))
}
