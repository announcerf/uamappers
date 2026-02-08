use crate::entities::ua_mapper;

use super::super::dto::UaMapperDtoV1;

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
