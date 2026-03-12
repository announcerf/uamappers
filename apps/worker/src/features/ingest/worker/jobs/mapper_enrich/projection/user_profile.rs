use rosu_v2::model::user::UserExtended;
use serde_json::Value;
use uamappers_api::features::mappers::storage::osu_user_fingerprint::{
    MapperFingerprint, MapperKudosu,
};

use super::shared::{mode_to_str, to_json_array};

pub fn user_to_mapper_fingerprint(
    user: &UserExtended,
    cached_at: chrono::DateTime<chrono::Utc>,
) -> MapperFingerprint {
    MapperFingerprint {
        username: user.username.to_string(),
        country: user.country.clone(),
        country_code: user.country_code.to_string(),
        avatar_url: user.avatar_url.clone(),
        cover: serde_json::to_value(&user.cover).unwrap_or(Value::Null),
        primary_mode: mode_to_str(user.mode).to_string(),
        mapping_followers: user.mapping_follower_count.unwrap_or(0) as i32,
        kudosu: MapperKudosu {
            total: user.kudosu.total,
            available: user.kudosu.available,
        },
        badges: to_json_array(user.badges.as_ref()),
        groups: to_json_array(user.groups.as_ref()),
        is_bng: user.is_bng.unwrap_or(false),
        is_nat: user.is_nat.unwrap_or(false),
        is_gmt: user.is_gmt.unwrap_or(false),
        is_probationary_bn: user.is_limited_bn.unwrap_or(false),
        is_full_bn: user.is_full_bn.unwrap_or(false),
        cached_at,
    }
}
