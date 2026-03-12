use std::collections::HashMap;

use sea_orm::DbErr;
use serde_json::json;

use crate::features::leaderboards::http::dto::LeaderboardKeyDto;
use crate::features::mappers::storage::{
    leaderboard_position_current_repo::LeaderboardPositionCurrentRepo,
    mapper_stats_current_repo::MapperStatsCurrentRepo, osu_user_fingerprint::MapperFingerprint,
    osu_user_repo::OsuUserRepo, ua_mapper_repo::UaMapperRepo,
};

use super::types::{LeaderboardPage, LeaderboardRow};

const LEADERBOARD_LIMIT: u64 = 100;

pub async fn get_leaderboard(
    positions_repo: &LeaderboardPositionCurrentRepo,
    ua_mappers_repo: &UaMapperRepo,
    osu_users_repo: &OsuUserRepo,
    mapper_stats_repo: &MapperStatsCurrentRepo,
    leaderboard: LeaderboardKeyDto,
    cursor: Option<i32>,
) -> Result<LeaderboardPage, DbErr> {
    let rows = positions_repo
        .list_page_by_rank(leaderboard.as_str(), cursor, LEADERBOARD_LIMIT)
        .await?;
    let ids: Vec<i64> = rows.iter().map(|row| row.osu_user_id).collect();
    let mappers = ua_mappers_repo.list_existing_rows(&ids).await?;
    let fingerprints = osu_users_repo.list_by_osu_user_ids(&ids).await?;
    let stats = mapper_stats_repo.list_by_osu_user_ids(&ids).await?;

    let mappers = mappers
        .into_iter()
        .map(|row| (row.osu_user_id, row))
        .collect::<HashMap<_, _>>();
    let fingerprints = fingerprints
        .into_iter()
        .filter_map(|row| MapperFingerprint::from_raw(&row.raw).map(|raw| (row.osu_user_id, raw)))
        .collect::<HashMap<_, _>>();
    let stats = stats
        .into_iter()
        .map(|row| (row.osu_user_id, row))
        .collect::<HashMap<_, _>>();

    let mut items = Vec::new();
    for row in &rows {
        let Some(mapper) = mappers.get(&row.osu_user_id) else {
            continue;
        };
        let Some(stat) = stats.get(&row.osu_user_id) else {
            continue;
        };
        let fingerprint = fingerprints.get(&row.osu_user_id);

        items.push(LeaderboardRow {
            rank: row.current_rank,
            previous_rank: row.previous_rank,
            rank_delta: rank_delta(row.current_rank, row.previous_rank),
            osu_user_id: row.osu_user_id,
            username: mapper.username.clone(),
            avatar_url: fingerprint
                .map(|row| row.avatar_url.clone())
                .unwrap_or_default(),
            country_code: mapper.country_code.clone(),
            main_metric_key: leaderboard_metric_key(leaderboard).to_string(),
            main_metric_value: leaderboard_metric_value(leaderboard, stat),
            secondary_metrics: secondary_metrics(leaderboard, stat),
            profile_url: format!("/mappers/by-id/{}", row.osu_user_id),
        });
    }

    let updated_at = rows
        .first()
        .map(|row| row.measured_at)
        .unwrap_or_else(chrono::Utc::now);
    let next_cursor = rows.last().and_then(|row| {
        if rows.len() as u64 == LEADERBOARD_LIMIT {
            Some(row.current_rank)
        } else {
            None
        }
    });

    Ok(LeaderboardPage {
        leaderboard,
        updated_at,
        next_cursor,
        items,
    })
}

fn rank_delta(current_rank: i32, previous_rank: Option<i32>) -> i32 {
    previous_rank.map(|prev| prev - current_rank).unwrap_or(0)
}

fn leaderboard_metric_key(leaderboard: LeaderboardKeyDto) -> &'static str {
    match leaderboard {
        LeaderboardKeyDto::Followers => "mapping_followers",
        LeaderboardKeyDto::Ranked => "ranked_mapsets",
        LeaderboardKeyDto::GuestDiff => "guest_mapsets",
        LeaderboardKeyDto::Plays => "total_playcount",
        LeaderboardKeyDto::Kudosu => "kudosu_total",
        LeaderboardKeyDto::Nominations => "nominated_mapsets",
    }
}

fn leaderboard_metric_value(
    leaderboard: LeaderboardKeyDto,
    stat: &crate::entities::mapper_stats_current::Model,
) -> f64 {
    match leaderboard {
        LeaderboardKeyDto::Followers => stat.mapping_followers as f64,
        LeaderboardKeyDto::Ranked => stat.ranked_mapsets as f64,
        LeaderboardKeyDto::GuestDiff => stat.guest_mapsets as f64,
        LeaderboardKeyDto::Plays => stat.total_playcount as f64,
        LeaderboardKeyDto::Kudosu => stat.kudosu_total as f64,
        LeaderboardKeyDto::Nominations => stat.nominated_mapsets as f64,
    }
}

fn secondary_metrics(
    leaderboard: LeaderboardKeyDto,
    stat: &crate::entities::mapper_stats_current::Model,
) -> serde_json::Value {
    match leaderboard {
        LeaderboardKeyDto::Followers => json!({
            "ranked_mapsets": stat.ranked_mapsets,
            "total_playcount": stat.total_playcount
        }),
        LeaderboardKeyDto::Ranked => json!({
            "loved_mapsets": stat.loved_mapsets,
            "total_playcount": stat.total_playcount
        }),
        LeaderboardKeyDto::GuestDiff => json!({
            "ranked_mapsets": stat.ranked_mapsets,
            "total_mapsets": stat.total_mapsets,
            "total_playcount": stat.total_playcount
        }),
        LeaderboardKeyDto::Plays => json!({
            "ranked_mapsets": stat.ranked_mapsets,
            "total_mapsets": stat.total_mapsets
        }),
        LeaderboardKeyDto::Kudosu => json!({
            "ranked_mapsets": stat.ranked_mapsets,
            "nominated_mapsets": stat.nominated_mapsets,
            "mapping_followers": stat.mapping_followers
        }),
        LeaderboardKeyDto::Nominations => json!({
            "ranked_mapsets": stat.ranked_mapsets,
            "guest_mapsets": stat.guest_mapsets,
            "kudosu_total": stat.kudosu_total
        }),
    }
}
