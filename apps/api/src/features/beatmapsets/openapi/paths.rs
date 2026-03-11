use crate::shared::errors::ErrorResponse;

use super::{BeatmapsetChartsResponseDto, BeatmapsetDetailsDto};

#[utoipa::path(
    get,
    path = "/beatmapsets/{osu_beatmapset_id}",
    tag = "Beatmapsets::Get",
    summary = "Get beatmapset details by osu beatmapset id",
    params(("osu_beatmapset_id" = i64, Path, description = "osu! beatmapset id")),
    responses(
        (status = 200, description = "Beatmapset details", body = BeatmapsetDetailsDto),
        (status = 404, description = "Not found", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    operation_id = "beatmapsets_get"
)]
pub async fn get_beatmapset_details() {}

#[utoipa::path(
    get,
    path = "/beatmapsets/{osu_beatmapset_id}/charts",
    tag = "Beatmapsets::Charts",
    summary = "Get beatmapset charts by osu beatmapset id",
    params(("osu_beatmapset_id" = i64, Path, description = "osu! beatmapset id")),
    responses(
        (status = 200, description = "Beatmapset charts", body = BeatmapsetChartsResponseDto),
        (status = 404, description = "Not found", body = ErrorResponse),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    operation_id = "beatmapsets_charts_get"
)]
pub async fn get_beatmapset_charts() {}
