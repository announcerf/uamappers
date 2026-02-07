use utoipa::OpenApi;

use crate::features::{
    ingest::http::dto::{IngestStatusDtoV1, ScanStateDtoV1},
    mappers::http::dto::{
        MapperBeatmapsetDtoV1, MapperBeatmapsetListQuery, MapperBeatmapsetListResponseV1,
        MapperDtoV1, MapperListQuery, MapperListResponseV1, MapperSearchQuery,
    },
    system::http::dto::HealthDtoV1,
};
use crate::shared::errors::ErrorResponse;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::features::mappers::http::handlers::list::list_mappers,
        crate::features::mappers::http::handlers::get::get_mapper,
        crate::features::mappers::http::handlers::get::get_mapper_by_id,
        crate::features::mappers::http::handlers::list::search_mappers,
        crate::features::mappers::http::handlers::beatmapsets::list_mapper_beatmapsets,
        crate::features::mappers::http::handlers::beatmapsets::list_mapper_beatmapsets_by_id,
        crate::features::ingest::http::handlers::get_status,
        crate::features::system::http::handlers::health
    ),
    components(schemas(
        MapperDtoV1,
        MapperListResponseV1,
        MapperListQuery,
        MapperSearchQuery,
        MapperBeatmapsetDtoV1,
        MapperBeatmapsetListQuery,
        MapperBeatmapsetListResponseV1,
        IngestStatusDtoV1,
        ScanStateDtoV1,
        HealthDtoV1,
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
