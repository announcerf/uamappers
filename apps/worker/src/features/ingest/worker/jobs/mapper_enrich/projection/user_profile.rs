use rosu_v2::model::user::UserExtended;
use uamappers_api::features::mappers::storage::mapper_profile_repo::NewMapperProfileRow;

use super::shared::{mode_to_str, offset_to_utc, to_json_array};

pub fn user_to_mapper_profile_row(
    user: &UserExtended,
    cached_at: chrono::DateTime<chrono::Utc>,
) -> NewMapperProfileRow {
    NewMapperProfileRow {
        osu_user_id: user.user_id as i64,
        username: user.username.to_string(),
        avatar_url: user.avatar_url.clone(),
        country: user.country.clone(),
        country_code: user.country_code.to_string(),
        cover_url: user.cover.url.clone(),
        primary_mode: mode_to_str(user.mode).to_string(),
        join_date: offset_to_utc(user.join_date),
        last_visit: user.last_visit.map(offset_to_utc),
        mapping_followers: user.mapping_follower_count.unwrap_or(0) as i32,
        kudosu_available: user.kudosu.available,
        kudosu_total: user.kudosu.total,
        badges_json: to_json_array(user.badges.as_ref()),
        groups_json: to_json_array(user.groups.as_ref()),
        is_bng: user.is_bng.unwrap_or(false),
        is_nat: user.is_nat.unwrap_or(false),
        is_gmt: user.is_gmt.unwrap_or(false),
        is_limited_bn: user.is_limited_bn.unwrap_or(false),
        is_full_bn: user.is_full_bn.unwrap_or(false),
        cached_at,
    }
}
