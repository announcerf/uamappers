use crate::entities::{beatmapset, ua_mapper};
use crate::features::mappers::usecases::{BeatmapsetPage, MapperPage, MapperProfile};

use super::super::dto::{
    BeatmapsetDtoV1, BeatmapsetListResponseV1, UaMapperDtoV1, UaMapperListResponseV1,
    UaMapperProfileDtoV1,
};

pub const DEFAULT_LIMIT: u64 = 50;
pub const MAX_LIMIT: u64 = 200;

pub fn clamp_limit(limit: Option<u64>) -> u64 {
    let limit = limit.unwrap_or(DEFAULT_LIMIT);
    limit.clamp(1, MAX_LIMIT)
}

pub fn ua_mapper_to_dto(model: ua_mapper::Model) -> UaMapperDtoV1 {
    UaMapperDtoV1 {
        osu_user_id: model.osu_user_id,
        username: model.username,
        country_code: model.country_code,
        first_seen_at: model.first_seen_at,
        last_seen_at: model.last_seen_at,
        updated_at: model.updated_at,
    }
}

pub fn mapper_page_to_dto(page: MapperPage) -> UaMapperListResponseV1 {
    let items = page.items.into_iter().map(ua_mapper_to_dto).collect();

    UaMapperListResponseV1 {
        items,
        limit: page.limit,
        offset: page.offset,
        total: page.total,
    }
}

pub fn mapper_profile_to_dto(profile: MapperProfile) -> UaMapperProfileDtoV1 {
    UaMapperProfileDtoV1 {
        mapper: ua_mapper_to_dto(profile.mapper),
        user: profile.user_raw,
        user_fetched_at: profile.user_fetched_at,
    }
}

pub fn beatmapset_page_to_dto(page: BeatmapsetPage) -> BeatmapsetListResponseV1 {
    let items = page.items.into_iter().map(beatmapset_to_dto).collect();

    BeatmapsetListResponseV1 {
        items,
        limit: page.limit,
        offset: page.offset,
        total: page.total,
    }
}

fn beatmapset_to_dto(model: beatmapset::Model) -> BeatmapsetDtoV1 {
    BeatmapsetDtoV1 {
        osu_beatmapset_id: model.osu_beatmapset_id,
        osu_last_updated: model.last_updated,
        cached_at: model.updated_at,
        raw: model.raw,
    }
}
