use crate::entities::ua_mapper;
use crate::features::mappers::usecases::{MapperCharts, MapperPage, MapperProfile};

use super::super::dto::{
    MapperChartsPointDto, MapperChartsResponseDto, MapperLeaderboardPositionDto,
    MapperProfileProjectionDto, MapperStatsCurrentDto, UaMapperDto, UaMapperListResponse,
    UaMapperProfileDto,
};

pub fn ua_mapper_to_dto(model: ua_mapper::Model) -> UaMapperDto {
    UaMapperDto {
        osu_user_id: model.osu_user_id,
        username: model.username,
        country_code: model.country_code,
        first_seen_at: model.first_seen_at,
        last_seen_at: model.last_seen_at,
        updated_at: model.updated_at,
    }
}

pub fn mapper_page_to_dto(page: MapperPage) -> UaMapperListResponse {
    let items = page.items.into_iter().map(ua_mapper_to_dto).collect();

    UaMapperListResponse {
        items,
        next_cursor: page.next_cursor,
        total: page.total,
    }
}

pub fn mapper_profile_to_dto(profile: MapperProfile) -> UaMapperProfileDto {
    UaMapperProfileDto {
        mapper: ua_mapper_to_dto(profile.mapper.clone()),
        profile: profile.mapper_profile.map(mapper_profile_projection_to_dto),
        stats: profile.mapper_stats.map(mapper_stats_to_dto),
        leaderboard_positions: profile
            .leaderboard_positions
            .into_iter()
            .map(mapper_leaderboard_position_to_dto)
            .collect(),
        charts: mapper_charts_to_dto(MapperCharts {
            osu_user_id: profile.mapper.osu_user_id,
            points: profile.charts,
        }),
        user: profile.user_raw,
        user_fetched_at: profile.user_fetched_at,
    }
}

pub fn mapper_charts_to_dto(charts: MapperCharts) -> MapperChartsResponseDto {
    MapperChartsResponseDto {
        osu_user_id: charts.osu_user_id,
        points: charts
            .points
            .into_iter()
            .map(|point| MapperChartsPointDto {
                snapshot_week: point.snapshot_week,
                total_mapsets: point.total_mapsets,
                ranked_mapsets: point.ranked_mapsets,
                loved_mapsets: point.loved_mapsets,
                guest_mapsets: point.guest_mapsets,
                nominated_mapsets: point.nominated_mapsets,
                total_playcount: point.total_playcount,
                avg_rating: point.avg_rating,
                avg_stars: point.avg_stars,
                avg_bpm: point.avg_bpm,
                avg_length_seconds: point.avg_length_seconds,
                main_mode: point.main_mode,
            })
            .collect(),
    }
}

fn mapper_profile_projection_to_dto(
    model: crate::entities::mapper_profile::Model,
) -> MapperProfileProjectionDto {
    MapperProfileProjectionDto {
        avatar_url: model.avatar_url,
        country: model.country,
        country_code: model.country_code,
        cover_url: model.cover_url,
        primary_mode: model.primary_mode,
        join_date: model.join_date,
        last_visit: model.last_visit,
        mapping_followers: model.mapping_followers,
        kudosu_available: model.kudosu_available,
        kudosu_total: model.kudosu_total,
        badges: model.badges_json,
        groups: model.groups_json,
        is_bng: model.is_bng,
        is_nat: model.is_nat,
        is_gmt: model.is_gmt,
        is_limited_bn: model.is_limited_bn,
        is_full_bn: model.is_full_bn,
        cached_at: model.cached_at,
    }
}

fn mapper_stats_to_dto(
    model: crate::entities::mapper_stats_current::Model,
) -> MapperStatsCurrentDto {
    MapperStatsCurrentDto {
        total_mapsets: model.total_mapsets,
        ranked_mapsets: model.ranked_mapsets,
        loved_mapsets: model.loved_mapsets,
        guest_mapsets: model.guest_mapsets,
        nominated_mapsets: model.nominated_mapsets,
        graveyard_mapsets: model.graveyard_mapsets,
        pending_mapsets: model.pending_mapsets,
        total_playcount: model.total_playcount,
        avg_rating: model.avg_rating,
        weighted_rating: model.weighted_rating,
        avg_stars: model.avg_stars,
        min_stars: model.min_stars,
        max_stars: model.max_stars,
        avg_bpm: model.avg_bpm,
        avg_length_seconds: model.avg_length_seconds,
        avg_ar: model.avg_ar,
        avg_cs: model.avg_cs,
        avg_od: model.avg_od,
        avg_hp: model.avg_hp,
        first_submitted_date: model.first_submitted_date,
        first_ranked_date: model.first_ranked_date,
        last_mapset_updated_at: model.last_mapset_updated_at,
        main_mode: model.main_mode,
        mapping_followers: model.mapping_followers,
        kudosu_total: model.kudosu_total,
        has_ranked: model.has_ranked,
        has_loved: model.has_loved,
        has_guest: model.has_guest,
        has_nominated: model.has_nominated,
        updated_at: model.updated_at,
    }
}

fn mapper_leaderboard_position_to_dto(
    model: crate::entities::leaderboard_position_current::Model,
) -> MapperLeaderboardPositionDto {
    MapperLeaderboardPositionDto {
        leaderboard_key: model.leaderboard_key,
        current_rank: model.current_rank,
        previous_rank: model.previous_rank,
        rank_delta: model.rank_delta,
        measured_at: model.measured_at,
    }
}
