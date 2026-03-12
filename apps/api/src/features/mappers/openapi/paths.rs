use crate::shared::errors::ErrorResponse;

use super::{
    BeatmapsetListQuery, BeatmapsetListResponse, MapperChartsResponseDto, UaMapperListQuery,
    UaMapperListResponse, UaMapperProfileDto, UaMapperSearchQuery,
};

#[utoipa::path(
    get,
    path = "/mappers",
    tag = "Mappers::List",
    summary = "List UA mappers",
    params(UaMapperListQuery),
    responses(
        (status = 200, description = "List UA mappers", body = UaMapperListResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    operation_id = "mappers_list"
)]
pub async fn list_mappers() {}

#[utoipa::path(
    get,
    path = "/mappers/search",
    tag = "Mappers::Search",
    summary = "Search UA mappers",
    params(UaMapperSearchQuery),
    responses(
        (status = 200, description = "Search UA mappers", body = UaMapperListResponse),
        (status = 400, description = "Invalid query", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    operation_id = "mappers_search"
)]
pub async fn search_mappers() {}

#[utoipa::path(
    get,
    path = "/mappers/{user}",
    tag = "Mappers::Get",
    summary = "Get UA mapper by username",
    params(("user" = String, Path, description = "osu! username")),
    responses(
        (status = 200, description = "Mapper profile", body = UaMapperProfileDto),
        (status = 404, description = "Not found", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    operation_id = "mappers_get_by_username"
)]
pub async fn get_mapper() {}

#[utoipa::path(
    get,
    path = "/mappers/by-id/{osu_user_id}",
    tag = "Mappers::GetById",
    summary = "Get UA mapper by osu user id",
    params(("osu_user_id" = i64, Path, description = "osu! user id")),
    responses(
        (status = 200, description = "Mapper profile", body = UaMapperProfileDto),
        (status = 404, description = "Not found", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    operation_id = "mappers_get_by_id"
)]
pub async fn get_mapper_by_id() {}

#[utoipa::path(
    get,
    path = "/mappers/{user}/charts",
    tag = "Mappers::Charts",
    summary = "Get mapper charts by username",
    params(("user" = String, Path, description = "osu! username")),
    responses(
        (status = 200, description = "Mapper charts", body = MapperChartsResponseDto),
        (status = 404, description = "Not found", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    operation_id = "mappers_charts_get_by_username"
)]
pub async fn get_mapper_charts() {}

#[utoipa::path(
    get,
    path = "/mappers/by-id/{osu_user_id}/charts",
    tag = "Mappers::ChartsById",
    summary = "Get mapper charts by osu user id",
    params(("osu_user_id" = i64, Path, description = "osu! user id")),
    responses(
        (status = 200, description = "Mapper charts", body = MapperChartsResponseDto),
        (status = 404, description = "Not found", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    operation_id = "mappers_charts_get_by_id"
)]
pub async fn get_mapper_charts_by_id() {}

#[utoipa::path(
    get,
    path = "/mappers/{user}/beatmapsets",
    tag = "Mappers::Beatmapsets",
    summary = "List mapper beatmapsets (by username)",
    params(
        ("user" = String, Path, description = "osu! username"),
        BeatmapsetListQuery
    ),
    responses(
        (status = 200, description = "Mapper beatmapsets", body = BeatmapsetListResponse),
        (status = 404, description = "Not found", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    operation_id = "mappers_beatmapsets_list_by_username"
)]
pub async fn list_mapper_beatmapsets() {}

#[utoipa::path(
    get,
    path = "/mappers/by-id/{osu_user_id}/beatmapsets",
    tag = "Mappers::BeatmapsetsById",
    summary = "List mapper beatmapsets (by osu user id)",
    params(
        ("osu_user_id" = i64, Path, description = "osu! user id"),
        BeatmapsetListQuery
    ),
    responses(
        (status = 200, description = "Mapper beatmapsets", body = BeatmapsetListResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    operation_id = "mappers_beatmapsets_list_by_id"
)]
pub async fn list_mapper_beatmapsets_by_id() {}
