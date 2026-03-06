use axum::{
    Json,
    extract::{Path, Query, State},
};

use crate::{app::state::AppState, features::mappers::usecases, shared::errors::ApiError};

use super::super::dto::{BeatmapsetListQuery, BeatmapsetListResponseV1};
use super::common::{beatmapset_page_to_dto, clamp_limit};

pub async fn list_mapper_beatmapsets(
    State(state): State<AppState>,
    Path(user): Path<String>,
    Query(query): Query<BeatmapsetListQuery>,
) -> Result<Json<BeatmapsetListResponseV1>, ApiError> {
    let page = usecases::PageInput {
        limit: clamp_limit(query.limit),
        offset: query.offset.unwrap_or(0),
    };

    let result = usecases::list_mapper_beatmapsets_by_username(
        &state.ua_mappers_repo,
        &state.osu_user_beatmapsets_repo,
        &user,
        query.kind.as_str(),
        page,
    )
    .await
    .map_err(|err| {
        tracing::error!(error = ?err, user = %user, "failed to resolve mapper");
        ApiError::internal("mappers_get_failed", "Failed to load mapper")
    })?;

    let Some(result) = result else {
        return Err(ApiError::not_found("mapper_not_found", "Mapper not found"));
    };

    Ok(Json(beatmapset_page_to_dto(result)))
}

pub async fn list_mapper_beatmapsets_by_id(
    State(state): State<AppState>,
    Path(osu_user_id): Path<i64>,
    Query(query): Query<BeatmapsetListQuery>,
) -> Result<Json<BeatmapsetListResponseV1>, ApiError> {
    let page = usecases::PageInput {
        limit: clamp_limit(query.limit),
        offset: query.offset.unwrap_or(0),
    };

    let result = usecases::list_mapper_beatmapsets_by_id(
        &state.osu_user_beatmapsets_repo,
        osu_user_id,
        query.kind.as_str(),
        page,
    )
    .await
    .map_err(|err| {
        tracing::error!(error = ?err, osu_user_id, "failed to list mapper beatmapsets");
        ApiError::internal(
            "mappers_beatmapsets_list_failed",
            "Failed to list mapper beatmapsets",
        )
    })?;

    Ok(Json(beatmapset_page_to_dto(result)))
}
