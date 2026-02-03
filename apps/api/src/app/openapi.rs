use utoipa::OpenApi;

use crate::features::{
    ingest::http::dto::{IngestStatusDtoV1, ScanStateDtoV1},
    mappers::http::dto::{MapperDtoV1, MapperListQuery, MapperListResponseV1, MapperSearchQuery},
    system::http::dto::HealthDtoV1,
};
use crate::shared::errors::ErrorResponse;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::features::mappers::http::handlers::list_mappers,
        crate::features::mappers::http::handlers::get_mapper,
        crate::features::mappers::http::handlers::search_mappers,
        crate::features::ingest::http::handlers::get_status,
        crate::features::system::http::handlers::health
    ),
    components(schemas(
        MapperDtoV1,
        MapperListResponseV1,
        MapperListQuery,
        MapperSearchQuery,
        IngestStatusDtoV1,
        ScanStateDtoV1,
        HealthDtoV1,
        ErrorResponse
    )),
    tags(
        (name = "Mappers::List"),
        (name = "Mappers::Get"),
        (name = "Mappers::Search"),
        (name = "Ingest::Status"),
        (name = "System::Health")
    )
)]
pub struct ApiDoc;
