use axum::{Json, extract::Query, extract::State};

use crate::{app::state::AppState, features::mappers::usecases, shared::errors::ApiError};

use super::super::dto::{UaMapperListQuery, UaMapperListResponseV1, UaMapperSearchQuery};
use super::common::{clamp_limit, mapper_page_to_dto};

pub async fn list_mappers(
    State(state): State<AppState>,
    Query(query): Query<UaMapperListQuery>,
) -> Result<Json<UaMapperListResponseV1>, ApiError> {
    let page = usecases::PageInput {
        limit: clamp_limit(query.limit),
        offset: query.offset.unwrap_or(0),
    };

    let result = usecases::list_mappers(&state.ua_mappers_repo, page)
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
) -> Result<Json<UaMapperListResponseV1>, ApiError> {
    if query.q.trim().is_empty() {
        return Err(
            ApiError::bad_request("query_required", "Search query is required")
                .with_field("q", "Query cannot be empty"),
        );
    }

    let page = usecases::PageInput {
        limit: clamp_limit(query.limit),
        offset: query.offset.unwrap_or(0),
    };

    let result = usecases::search_mappers(&state.ua_mappers_repo, &query.q, page)
        .await
        .map_err(|err| {
            tracing::error!(error = ?err, query = %query.q, "failed to search mappers");
            ApiError::internal("mappers_search_failed", "Failed to search mappers")
        })?;

    Ok(Json(mapper_page_to_dto(result)))
}
