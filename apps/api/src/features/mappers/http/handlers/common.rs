use crate::entities::mapper;

use super::super::dto::MapperDtoV1;

pub const DEFAULT_LIMIT: u64 = 50;
pub const MAX_LIMIT: u64 = 200;

pub fn clamp_limit(limit: Option<u64>) -> u64 {
    let limit = limit.unwrap_or(DEFAULT_LIMIT);
    limit.clamp(1, MAX_LIMIT)
}

pub fn mapper_to_dto(model: mapper::Model) -> MapperDtoV1 {
    MapperDtoV1 {
        osu_user_id: model.osu_user_id,
        username: model.username,
        country_code: model.country_code,
        kudosu_available: model.kudosu_available,
        kudosu_total: model.kudosu_total,
        count_graveyard: model.count_graveyard,
        count_pending: model.count_pending,
        count_wip: model.count_wip,
        count_loved: model.count_loved,
        count_ranked: model.count_ranked,
        count_approved: model.count_approved,
        count_total: model.count_total,
        is_bn: model.is_bn,
        nominated_count: model.nominated_count,
    }
}
