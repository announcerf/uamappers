use axum::{extract::Query, extract::State, Json};

use crate::{app::state::AppState, shared::errors::ApiError};

use super::super::dto::{UaMapperListQuery, UaMapperListResponseV1, UaMapperSearchQuery};
use super::common::{clamp_limit, ua_mapper_to_dto};

#[utoipa::path(
    get,
    path = "/mappers",
    tag = "Mappers::List",
    summary = "List UA mappers",
    params(UaMapperListQuery),
    responses(
        (status = 200, description = "List UA mappers", body = UaMapperListResponseV1),
        (status = 500, description = "Internal error", body = crate::shared::errors::ErrorResponse)
    ),
    operation_id = "mappers_list_v1"
)]
pub async fn list_mappers(
    State(state): State<AppState>,
    Query(query): Query<UaMapperListQuery>,
) -> Result<Json<UaMapperListResponseV1>, ApiError> {
    let limit = clamp_limit(query.limit);
    let offset = query.offset.unwrap_or(0);

    let (rows, total) = state
        .ua_mappers_repo
        .list(limit, offset)
        .await
        .map_err(|err| {
            tracing::error!(error = ?err, "failed to list mappers");
            ApiError::internal("mappers_list_failed", "Failed to list mappers")
        })?;

    let items = rows.into_iter().map(ua_mapper_to_dto).collect();

    Ok(Json(UaMapperListResponseV1 {
        items,
        limit,
        offset,
        total,
    }))
}

#[utoipa::path(
    get,
    path = "/mappers/search",
    tag = "Mappers::Search",
    summary = "Search UA mappers",
    params(UaMapperSearchQuery),
    responses(
        (status = 200, description = "Search UA mappers", body = UaMapperListResponseV1),
        (status = 400, description = "Invalid query", body = crate::shared::errors::ErrorResponse),
        (status = 500, description = "Internal error", body = crate::shared::errors::ErrorResponse)
    ),
    operation_id = "mappers_search_v1"
)]
pub async fn search_mappers(
    State(state): State<AppState>,
    Query(query): Query<UaMapperSearchQuery>,
) -> Result<Json<UaMapperListResponseV1>, ApiError> {
    if query.q.trim().is_empty() {
        return Err(
            ApiError::bad_request("query_required", "Search query is required")
                .with_field("q", "Query cannot be empty"),
        );
    }

    let limit = clamp_limit(query.limit);
    let offset = query.offset.unwrap_or(0);

    let (rows, total) = state
        .ua_mappers_repo
        .search(&query.q, limit, offset)
        .await
        .map_err(|err| {
            tracing::error!(error = ?err, query = %query.q, "failed to search mappers");
            ApiError::internal("mappers_search_failed", "Failed to search mappers")
        })?;

    let items = rows.into_iter().map(ua_mapper_to_dto).collect();

    Ok(Json(UaMapperListResponseV1 {
        items,
        limit,
        offset,
        total,
    }))
}
