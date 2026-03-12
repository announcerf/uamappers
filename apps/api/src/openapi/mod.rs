use utoipa::OpenApi;

use crate::features::{beatmapsets, ingest, leaderboards, mappers, system};
use crate::shared::errors::ErrorResponse;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "uamappers API",
        description = "Backend API for indexing and serving Ukrainian osu! mappers.",
        version = "1.0.0"
    ),
    paths(
        mappers::openapi::paths::list_mappers,
        mappers::openapi::paths::get_mapper,
        mappers::openapi::paths::get_mapper_by_id,
        mappers::openapi::paths::get_mapper_charts,
        mappers::openapi::paths::get_mapper_charts_by_id,
        mappers::openapi::paths::search_mappers,
        mappers::openapi::paths::list_mapper_beatmapsets,
        mappers::openapi::paths::list_mapper_beatmapsets_by_id,
        beatmapsets::openapi::paths::get_beatmapset_details,
        beatmapsets::openapi::paths::get_beatmapset_charts,
        leaderboards::openapi::paths::get_leaderboard,
        ingest::openapi::paths::get_status,
        system::openapi::paths::health
    ),
    components(schemas(
        mappers::openapi::UaMapperDto,
        mappers::openapi::UaMapperListResponse,
        mappers::openapi::UaMapperListQuery,
        mappers::openapi::UaMapperSearchQuery,
        mappers::openapi::UaMapperProfileDto,
        mappers::openapi::MapperChartsPointDto,
        mappers::openapi::MapperChartsResponseDto,
        mappers::openapi::MapperBioDto,
        mappers::openapi::MapperDto,
        mappers::openapi::MapperLeaderboardPositionDto,
        mappers::openapi::MapperKudosuDto,
        mappers::openapi::MapperStatsCurrentDto,
        mappers::openapi::MapperTrackingDto,
        mappers::openapi::UserBeatmapsetsKindDto,
        mappers::openapi::BeatmapsetDto,
        mappers::openapi::BeatmapsetListQuery,
        mappers::openapi::BeatmapsetListResponse,
        beatmapsets::openapi::BeatmapsetDetailsDto,
        beatmapsets::openapi::BeatmapsetHeaderDto,
        beatmapsets::openapi::BeatmapsetHeadlineStatsDto,
        beatmapsets::openapi::BeatmapDifficultyOverviewDto,
        beatmapsets::openapi::BeatmapDifficultyDetailDto,
        beatmapsets::openapi::BeatmapsetAnalyticsDto,
        beatmapsets::openapi::BeatmapsetChartsPointDto,
        beatmapsets::openapi::BeatmapsetChartsResponseDto,
        leaderboards::openapi::LeaderboardKeyDto,
        leaderboards::openapi::LeaderboardQueryDto,
        leaderboards::openapi::LeaderboardRowDto,
        leaderboards::openapi::LeaderboardResponseDto,
        ingest::openapi::IngestStatusDto,
        ingest::openapi::ScanStateDto,
        system::openapi::HealthDto,
        ErrorResponse
    )),
    tags(
        (name = "Mappers::List"),
        (name = "Mappers::Get"),
        (name = "Mappers::GetById"),
        (name = "Mappers::Charts"),
        (name = "Mappers::ChartsById"),
        (name = "Mappers::Search"),
        (name = "Mappers::Beatmapsets"),
        (name = "Mappers::BeatmapsetsById"),
        (name = "Beatmapsets::Get"),
        (name = "Beatmapsets::Charts"),
        (name = "Leaderboards::Get"),
        (name = "Ingest::Status"),
        (name = "System::Health")
    )
)]
pub struct ApiDoc;
