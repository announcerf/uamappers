use crate::entities::ua_mapper;
use crate::features::mappers::storage::osu_user_fingerprint::{MapperFingerprint, MapperKudosu};
use crate::features::mappers::usecases::{MapperCharts, MapperPage, MapperProfile};

use super::super::dto::{
    MapperBioDto, MapperChartsPointDto, MapperChartsResponseDto, MapperDto, MapperKudosuDto,
    MapperLeaderboardPositionDto, MapperStatsCurrentDto, MapperTrackingDto, UaMapperDto,
    UaMapperListResponse, UaMapperProfileDto,
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
    let mapper = build_mapper(profile.mapper.clone(), profile.mapper_fingerprint);

    UaMapperProfileDto {
        mapper,
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

fn build_mapper(
    mapper: crate::entities::ua_mapper::Model,
    fingerprint: Option<serde_json::Value>,
) -> MapperDto {
    let fingerprint = fingerprint
        .as_ref()
        .and_then(MapperFingerprint::from_raw)
        .unwrap_or_else(|| fallback_fingerprint(&mapper));

    MapperDto {
        bio: MapperBioDto {
            osu_user_id: mapper.osu_user_id,
            username: mapper.username,
            country: fingerprint.country,
            country_code: mapper.country_code,
            avatar_url: fingerprint.avatar_url,
            cover: fingerprint.cover,
            badges: fingerprint.badges,
            groups: fingerprint.groups,
            primary_mode: fingerprint.primary_mode,
            is_bng: fingerprint.is_bng,
            is_nat: fingerprint.is_nat,
            is_gmt: fingerprint.is_gmt,
            is_probation_bn: fingerprint.is_probation_bn,
            is_full_bn: fingerprint.is_full_bn,
        },
        tracking: MapperTrackingDto {
            cached_at: fingerprint.cached_at,
            first_seen_at: mapper.first_seen_at,
            last_seen_at: mapper.last_seen_at,
            created_at: mapper.created_at,
            updated_at: mapper.updated_at,
        },
    }
}

fn fallback_fingerprint(mapper: &crate::entities::ua_mapper::Model) -> MapperFingerprint {
    MapperFingerprint {
        username: mapper.username.clone(),
        country: String::new(),
        country_code: mapper.country_code.clone(),
        avatar_url: String::new(),
        cover: serde_json::Value::Null,
        primary_mode: String::new(),
        mapping_followers: 0,
        kudosu: MapperKudosu {
            total: 0,
            available: 0,
        },
        badges: serde_json::json!([]),
        groups: serde_json::json!([]),
        is_bng: false,
        is_nat: false,
        is_gmt: false,
        is_probation_bn: false,
        is_full_bn: false,
        cached_at: mapper.updated_at,
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
        kudosu: MapperKudosuDto {
            total: model.kudosu_total,
            available: model.kudosu_available,
        },
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
