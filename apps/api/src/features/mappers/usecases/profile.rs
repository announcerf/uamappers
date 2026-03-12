use crate::shared::ranking::rank_delta;

use sea_orm::DbErr;

use crate::features::leaderboards::http::dto::LeaderboardKeyDto;
use crate::features::mappers::storage::{
    beatmap_profile_repo::BeatmapProfileRepo, beatmapset_profile_repo::BeatmapsetProfileRepo,
    leaderboard_position_current_repo::LeaderboardPositionCurrentRepo,
    mapper_aggregate_snapshot_weekly_repo::MapperAggregateSnapshotWeeklyRepo,
    mapper_stats_current_repo::MapperStatsCurrentRepo, osu_user_beatmapset_repo::OsuUserBeatmapsetRepo,
    osu_user_repo::OsuUserRepo,
};

use super::chart_points::build_mapper_chart_points;
use super::current_stats::build_mapper_current_stats;
use super::types::{MapperLeaderboardPosition, MapperProfile};

pub struct MapperProfileReadRepos<'a> {
    pub beatmapset_profiles_repo: &'a BeatmapsetProfileRepo,
    pub beatmap_profiles_repo: &'a BeatmapProfileRepo,
    pub mapper_stats_repo: &'a MapperStatsCurrentRepo,
    pub leaderboard_positions_repo: &'a LeaderboardPositionCurrentRepo,
    pub snapshots_repo: &'a MapperAggregateSnapshotWeeklyRepo,
    pub osu_user_beatmapsets_repo: &'a OsuUserBeatmapsetRepo,
    pub osu_users_repo: &'a OsuUserRepo,
}

pub async fn load_mapper_profile(
    repos: MapperProfileReadRepos<'_>,
    mapper: Option<crate::entities::ua_mapper::Model>,
) -> Result<Option<MapperProfile>, DbErr> {
    let Some(mapper) = mapper else {
        return Ok(None);
    };

    let mapper_stats = match repos
        .mapper_stats_repo
        .get_by_osu_user_id(mapper.osu_user_id)
        .await?
    {
        Some(row) => {
            let relations = repos
                .osu_user_beatmapsets_repo
                .list_by_osu_user_id(mapper.osu_user_id)
                .await?;
            let mapset_ids = mapset_ids(&relations);
            let beatmapsets = repos
                .beatmapset_profiles_repo
                .list_by_osu_beatmapset_ids(&mapset_ids)
                .await?;
            let beatmaps = repos
                .beatmap_profiles_repo
                .list_by_osu_beatmapset_ids(&mapset_ids)
                .await?;

            Some(build_mapper_current_stats(row, &beatmapsets, &beatmaps))
        }
        None => None,
    };
    let leaderboard_positions = repos
        .leaderboard_positions_repo
        .list_by_osu_user_id(mapper.osu_user_id)
        .await?;
    let charts = build_mapper_chart_points(
        repos.snapshots_repo
            .list_by_osu_user_id(mapper.osu_user_id)
            .await?,
    );
    let mapper_fingerprint = repos
        .osu_users_repo
        .get_by_osu_user_id(mapper.osu_user_id)
        .await?
        .map(|row| row.raw);

    Ok(Some(MapperProfile {
        mapper,
        mapper_fingerprint,
        mapper_stats,
        leaderboard_positions: order_positions(leaderboard_positions),
        charts,
    }))
}

fn order_positions(
    mut positions: Vec<crate::entities::leaderboard_position_current::Model>,
) -> Vec<MapperLeaderboardPosition> {
    positions.sort_by_key(|row| leaderboard_order(&row.leaderboard_key));
    positions
        .into_iter()
        .map(|row| MapperLeaderboardPosition {
            leaderboard_key: row.leaderboard_key,
            current_rank: row.current_rank,
            previous_rank: row.previous_rank,
            rank_delta: rank_delta(row.current_rank, row.previous_rank),
            measured_at: row.measured_at,
        })
        .collect()
}

fn leaderboard_order(key: &str) -> usize {
    leaderboard_keys()
        .iter()
        .position(|candidate| *candidate == key)
        .unwrap_or(usize::MAX)
}

fn leaderboard_keys() -> [&'static str; 6] {
    [
        LeaderboardKeyDto::Followers.as_str(),
        LeaderboardKeyDto::Ranked.as_str(),
        LeaderboardKeyDto::GuestDiff.as_str(),
        LeaderboardKeyDto::Plays.as_str(),
        LeaderboardKeyDto::Kudosu.as_str(),
        LeaderboardKeyDto::Nominations.as_str(),
    ]
}

fn mapset_ids(relations: &[crate::entities::osu_user_beatmapset::Model]) -> Vec<i64> {
    use std::collections::HashSet;

    relations
        .iter()
        .map(|row| row.osu_beatmapset_id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}
