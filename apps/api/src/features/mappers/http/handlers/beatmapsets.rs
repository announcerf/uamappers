use axum::{
    extract::{Path, Query, State},
    Json,
};

use crate::{app::state::AppState, shared::errors::ApiError};

use super::super::dto::{
    MapperBeatmapsetDtoV1, MapperBeatmapsetListQuery, MapperBeatmapsetListResponseV1,
};
use super::common::clamp_limit;

#[utoipa::path(
    get,
    path = "/mappers/{user}/beatmapsets",
    tag = "Mappers::Beatmapsets",
    params(
        ("user" = String, Path, description = "osu! username"),
        MapperBeatmapsetListQuery
    ),
    responses(
        (status = 200, description = "Mapper beatmapsets", body = MapperBeatmapsetListResponseV1),
        (status = 404, description = "Not found", body = crate::shared::errors::ErrorResponse),
        (status = 500, description = "Internal error", body = crate::shared::errors::ErrorResponse)
    ),
    operation_id = "mappers_beatmapsets_list_by_username_v1"
)]
pub async fn list_mapper_beatmapsets(
    State(state): State<AppState>,
    Path(user): Path<String>,
    Query(query): Query<MapperBeatmapsetListQuery>,
) -> Result<Json<MapperBeatmapsetListResponseV1>, ApiError> {
    let limit = clamp_limit(query.limit);
    let offset = query.offset.unwrap_or(0);

    let Some(mapper) = state
        .mappers_repo
        .get_by_username(&user)
        .await
        .map_err(|err| {
            tracing::error!(error = ?err, user = %user, "failed to resolve mapper");
            ApiError::internal("mappers_get_failed", "Failed to load mapper")
        })?
    else {
        return Err(ApiError::not_found("mapper_not_found", "Mapper not found"));
    };

    let (rows, total) = state
        .beatmapsets_repo
        .list_by_creator(mapper.osu_user_id, limit, offset)
        .await
        .map_err(|err| {
            tracing::error!(
                error = ?err,
                user = %user,
                osu_user_id = mapper.osu_user_id,
                "failed to list mapper beatmapsets"
            );
            ApiError::internal(
                "mappers_beatmapsets_list_failed",
                "Failed to list mapper beatmapsets",
            )
        })?;

    let items = rows
        .into_iter()
        .map(|row| MapperBeatmapsetDtoV1 {
            osu_beatmapset_id: row.osu_beatmapset_id,
            creator_osu_user_id: row.creator_osu_user_id,
            creator_username: row.creator_username,
            status: row.status,
            artist: row.artist,
            title: row.title,
            artist_unicode: row.artist_unicode,
            title_unicode: row.title_unicode,
            submitted_date: row.submitted_date,
            ranked_date: row.ranked_date,
            last_updated: row.last_updated,
            play_count: row.play_count,
            favourite_count: row.favourite_count,
        })
        .collect();

    Ok(Json(MapperBeatmapsetListResponseV1 {
        items,
        limit,
        offset,
        total,
    }))
}

#[utoipa::path(
    get,
    path = "/mappers/by-id/{osu_user_id}/beatmapsets",
    tag = "Mappers::BeatmapsetsById",
    params(
        ("osu_user_id" = i64, Path, description = "osu! user id"),
        MapperBeatmapsetListQuery
    ),
    responses(
        (status = 200, description = "Mapper beatmapsets", body = MapperBeatmapsetListResponseV1),
        (status = 500, description = "Internal error", body = crate::shared::errors::ErrorResponse)
    ),
    operation_id = "mappers_beatmapsets_list_by_id_v1"
)]
pub async fn list_mapper_beatmapsets_by_id(
    State(state): State<AppState>,
    Path(osu_user_id): Path<i64>,
    Query(query): Query<MapperBeatmapsetListQuery>,
) -> Result<Json<MapperBeatmapsetListResponseV1>, ApiError> {
    let limit = clamp_limit(query.limit);
    let offset = query.offset.unwrap_or(0);

    let (rows, total) = state
        .beatmapsets_repo
        .list_by_creator(osu_user_id, limit, offset)
        .await
        .map_err(|err| {
            tracing::error!(error = ?err, osu_user_id, "failed to list mapper beatmapsets");
            ApiError::internal(
                "mappers_beatmapsets_list_failed",
                "Failed to list mapper beatmapsets",
            )
        })?;

    let items = rows
        .into_iter()
        .map(|row| MapperBeatmapsetDtoV1 {
            osu_beatmapset_id: row.osu_beatmapset_id,
            creator_osu_user_id: row.creator_osu_user_id,
            creator_username: row.creator_username,
            status: row.status,
            artist: row.artist,
            title: row.title,
            artist_unicode: row.artist_unicode,
            title_unicode: row.title_unicode,
            submitted_date: row.submitted_date,
            ranked_date: row.ranked_date,
            last_updated: row.last_updated,
            play_count: row.play_count,
            favourite_count: row.favourite_count,
        })
        .collect();

    Ok(Json(MapperBeatmapsetListResponseV1 {
        items,
        limit,
        offset,
        total,
    }))
}
