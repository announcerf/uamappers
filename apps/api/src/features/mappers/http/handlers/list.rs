use axum::{extract::Query, extract::State, Json};

use crate::{app::state::AppState, features::mappers::usecases, shared::errors::ApiError};

use super::super::dto::{UaMapperListQuery, UaMapperListResponse, UaMapperSearchQuery};
use super::super::pagination::DEFAULT_LIMIT;
use super::super::presenters::mapper_page_to_dto;

pub async fn list_mappers(
    State(state): State<AppState>,
    Query(query): Query<UaMapperListQuery>,
) -> Result<Json<UaMapperListResponse>, ApiError> {
    let cursor = usecases::CursorInput {
        limit: DEFAULT_LIMIT,
        after: query.cursor,
    };

    let result = usecases::list_mappers(&state.ua_mappers_repo, cursor)
        .await
        .map_err(|err| {
            tracing::error!(error = ?err, "failed to list mappers");
            ApiError::internal("mappers_list_failed", "Failed to list mappers")
        })?;

    Ok(Json(mapper_page_to_dto(result)))
}

pub async fn search_mappers(
    State(state): State<AppState>,
    Query(query): Query<UaMapperSearchQuery>,
) -> Result<Json<UaMapperListResponse>, ApiError> {
    if query.q.trim().is_empty() {
        return Err(
            ApiError::bad_request("query_required", "Search query is required")
                .with_field("q", "Query cannot be empty"),
        );
    }

    let cursor = usecases::CursorInput {
        limit: DEFAULT_LIMIT,
        after: query.cursor,
    };

    let result = usecases::search_mappers(&state.ua_mappers_repo, &query.q, cursor)
        .await
        .map_err(|err| {
            tracing::error!(error = ?err, query = %query.q, "failed to search mappers");
            ApiError::internal("mappers_search_failed", "Failed to search mappers")
        })?;

    Ok(Json(mapper_page_to_dto(result)))
}
