use crate::shared::errors::ErrorResponse;

use super::{
    BeatmapsetListQuery, BeatmapsetListResponseV1, UaMapperListQuery, UaMapperListResponseV1,
    UaMapperProfileDtoV1, UaMapperSearchQuery,
};

#[utoipa::path(
    get,
    path = "/mappers",
    tag = "Mappers::List",
    summary = "List UA mappers",
    params(UaMapperListQuery),
    responses(
        (status = 200, description = "List UA mappers", body = UaMapperListResponseV1),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    operation_id = "mappers_list_v1"
)]
pub async fn list_mappers() {}

#[utoipa::path(
    get,
    path = "/mappers/search",
    tag = "Mappers::Search",
    summary = "Search UA mappers",
    params(UaMapperSearchQuery),
    responses(
        (status = 200, description = "Search UA mappers", body = UaMapperListResponseV1),
        (status = 400, description = "Invalid query", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    operation_id = "mappers_search_v1"
)]
pub async fn search_mappers() {}

#[utoipa::path(
    get,
    path = "/mappers/{user}",
    tag = "Mappers::Get",
    summary = "Get UA mapper by username",
    params(("user" = String, Path, description = "osu! username")),
    responses(
        (status = 200, description = "Mapper profile", body = UaMapperProfileDtoV1),
        (status = 404, description = "Not found", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    operation_id = "mappers_get_by_username_v1"
)]
pub async fn get_mapper() {}

#[utoipa::path(
    get,
    path = "/mappers/by-id/{osu_user_id}",
    tag = "Mappers::GetById",
    summary = "Get UA mapper by osu user id",
    params(("osu_user_id" = i64, Path, description = "osu! user id")),
    responses(
        (status = 200, description = "Mapper profile", body = UaMapperProfileDtoV1),
        (status = 404, description = "Not found", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    operation_id = "mappers_get_by_id_v1"
)]
pub async fn get_mapper_by_id() {}

#[utoipa::path(
    get,
    path = "/mappers/{user}/beatmapsets",
    tag = "Mappers::Beatmapsets",
    summary = "List cached beatmapsets for mapper (by username)",
    params(
        ("user" = String, Path, description = "osu! username"),
        BeatmapsetListQuery
    ),
    responses(
        (status = 200, description = "Mapper beatmapsets", body = BeatmapsetListResponseV1),
        (status = 404, description = "Not found", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    operation_id = "mappers_beatmapsets_list_by_username_v1"
)]
pub async fn list_mapper_beatmapsets() {}

#[utoipa::path(
    get,
    path = "/mappers/by-id/{osu_user_id}/beatmapsets",
    tag = "Mappers::BeatmapsetsById",
    summary = "List cached beatmapsets for mapper (by osu user id)",
    params(
        ("osu_user_id" = i64, Path, description = "osu! user id"),
        BeatmapsetListQuery
    ),
    responses(
        (status = 200, description = "Mapper beatmapsets", body = BeatmapsetListResponseV1),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    operation_id = "mappers_beatmapsets_list_by_id_v1"
)]
pub async fn list_mapper_beatmapsets_by_id() {}
