use utoipa::OpenApi;

use crate::features::{ingest, mappers, system};
use crate::shared::errors::ErrorResponse;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "uamappers API",
        description = "Backend API for indexing and serving Ukrainian osu! mappers.",
        version = "v1"
    ),
    paths(
        mappers::openapi::paths::list_mappers,
        mappers::openapi::paths::get_mapper,
        mappers::openapi::paths::get_mapper_by_id,
        mappers::openapi::paths::search_mappers,
        mappers::openapi::paths::list_mapper_beatmapsets,
        mappers::openapi::paths::list_mapper_beatmapsets_by_id,
        ingest::openapi::paths::get_status,
        system::openapi::paths::health
    ),
    components(schemas(
        mappers::openapi::UaMapperDtoV1,
        mappers::openapi::UaMapperListResponseV1,
        mappers::openapi::UaMapperListQuery,
        mappers::openapi::UaMapperSearchQuery,
        mappers::openapi::UaMapperProfileDtoV1,
        mappers::openapi::UserBeatmapsetsKindDtoV1,
        mappers::openapi::BeatmapsetDtoV1,
        mappers::openapi::BeatmapsetListQuery,
        mappers::openapi::BeatmapsetListResponseV1,
        ingest::openapi::IngestStatusDtoV1,
        ingest::openapi::ScanStateDtoV1,
        system::openapi::HealthDtoV1,
        ErrorResponse
    )),
    tags(
        (name = "Mappers::List"),
        (name = "Mappers::Get"),
        (name = "Mappers::GetById"),
        (name = "Mappers::Search"),
        (name = "Mappers::Beatmapsets"),
        (name = "Mappers::BeatmapsetsById"),
        (name = "Ingest::Status"),
        (name = "System::Health")
    )
)]
pub struct ApiDoc;
