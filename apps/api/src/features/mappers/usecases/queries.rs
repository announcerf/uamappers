use sea_orm::DbErr;

use crate::features::leaderboards::http::dto::LeaderboardKeyDto;
use crate::features::mappers::storage::{
    leaderboard_position_current_repo::LeaderboardPositionCurrentRepo,
    mapper_aggregate_snapshot_weekly_repo::MapperAggregateSnapshotWeeklyRepo,
    mapper_stats_current_repo::MapperStatsCurrentRepo, osu_user_beatmapset_repo::OsuUserBeatmapsetRepo,
    osu_user_repo::OsuUserRepo, ua_mapper_repo::UaMapperRepo,
};

use super::types::{
    BeatmapsetPage, CursorInput, MapperCharts, MapperPage, MapperProfile, PageInput,
};

pub async fn list_mappers(
    ua_mappers_repo: &UaMapperRepo,
    cursor: CursorInput,
) -> Result<MapperPage, DbErr> {
    let total = ua_mappers_repo.count_all().await?;
    let after = cursor.after.unwrap_or(0);
    let items = ua_mappers_repo.list_after_id(after, cursor.limit).await?;
    let next_cursor = items.last().map(|row| row.osu_user_id);

    Ok(MapperPage {
        items,
        next_cursor,
        total,
    })
}

pub async fn search_mappers(
    ua_mappers_repo: &UaMapperRepo,
    query: &str,
    cursor: CursorInput,
) -> Result<MapperPage, DbErr> {
    let (items, total) = ua_mappers_repo
        .search_after_id(query, cursor.after, cursor.limit)
        .await?;
    let next_cursor = items.last().map(|row| row.osu_user_id);

    Ok(MapperPage {
        items,
        next_cursor,
        total,
    })
}

pub async fn get_mapper_profile_by_username(
    ua_mappers_repo: &UaMapperRepo,
    mapper_stats_repo: &MapperStatsCurrentRepo,
    leaderboard_positions_repo: &LeaderboardPositionCurrentRepo,
    snapshots_repo: &MapperAggregateSnapshotWeeklyRepo,
    osu_users_repo: &OsuUserRepo,
    username: &str,
) -> Result<Option<MapperProfile>, DbErr> {
    let mapper = ua_mappers_repo.get_by_username(username).await?;

    load_mapper_profile(
        mapper_stats_repo,
        leaderboard_positions_repo,
        snapshots_repo,
        osu_users_repo,
        mapper,
    )
    .await
}

pub async fn get_mapper_charts_by_username(
    ua_mappers_repo: &UaMapperRepo,
    snapshots_repo: &MapperAggregateSnapshotWeeklyRepo,
    username: &str,
) -> Result<Option<MapperCharts>, DbErr> {
    let mapper = ua_mappers_repo.get_by_username(username).await?;
    let Some(mapper) = mapper else {
        return Ok(None);
    };

    let points = snapshots_repo
        .list_by_osu_user_id(mapper.osu_user_id)
        .await?;

    Ok(Some(MapperCharts {
        osu_user_id: mapper.osu_user_id,
        points,
    }))
}

pub async fn get_mapper_profile_by_id(
    ua_mappers_repo: &UaMapperRepo,
    mapper_stats_repo: &MapperStatsCurrentRepo,
    leaderboard_positions_repo: &LeaderboardPositionCurrentRepo,
    snapshots_repo: &MapperAggregateSnapshotWeeklyRepo,
    osu_users_repo: &OsuUserRepo,
    osu_user_id: i64,
) -> Result<Option<MapperProfile>, DbErr> {
    let mapper = ua_mappers_repo.get_by_osu_user_id(osu_user_id).await?;

    load_mapper_profile(
        mapper_stats_repo,
        leaderboard_positions_repo,
        snapshots_repo,
        osu_users_repo,
        mapper,
    )
    .await
}

pub async fn get_mapper_charts_by_id(
    ua_mappers_repo: &UaMapperRepo,
    snapshots_repo: &MapperAggregateSnapshotWeeklyRepo,
    osu_user_id: i64,
) -> Result<Option<MapperCharts>, DbErr> {
    let mapper = ua_mappers_repo.get_by_osu_user_id(osu_user_id).await?;
    let Some(mapper) = mapper else {
        return Ok(None);
    };

    let points = snapshots_repo
        .list_by_osu_user_id(mapper.osu_user_id)
        .await?;

    Ok(Some(MapperCharts {
        osu_user_id: mapper.osu_user_id,
        points,
    }))
}

pub async fn list_mapper_beatmapsets_by_username(
    ua_mappers_repo: &UaMapperRepo,
    osu_user_beatmapsets_repo: &OsuUserBeatmapsetRepo,
    username: &str,
    kind: &str,
    page: PageInput,
) -> Result<Option<BeatmapsetPage>, DbErr> {
    let mapper = ua_mappers_repo.get_by_username(username).await?;

    let Some(mapper) = mapper else {
        return Ok(None);
    };

    let beatmapsets =
        list_mapper_beatmapsets_by_id(osu_user_beatmapsets_repo, mapper.osu_user_id, kind, page)
            .await?;

    Ok(Some(beatmapsets))
}

pub async fn list_mapper_beatmapsets_by_id(
    osu_user_beatmapsets_repo: &OsuUserBeatmapsetRepo,
    osu_user_id: i64,
    kind: &str,
    page: PageInput,
) -> Result<BeatmapsetPage, DbErr> {
    let (items, total) = osu_user_beatmapsets_repo
        .list_beatmapsets(osu_user_id, kind, page.limit, page.offset)
        .await?;

    Ok(BeatmapsetPage {
        items,
        limit: page.limit,
        offset: page.offset,
        total,
    })
}

async fn load_mapper_profile(
    mapper_stats_repo: &MapperStatsCurrentRepo,
    leaderboard_positions_repo: &LeaderboardPositionCurrentRepo,
    snapshots_repo: &MapperAggregateSnapshotWeeklyRepo,
    osu_users_repo: &OsuUserRepo,
    mapper: Option<crate::entities::ua_mapper::Model>,
) -> Result<Option<MapperProfile>, DbErr> {
    let Some(mapper) = mapper else {
        return Ok(None);
    };

    let mapper_stats = mapper_stats_repo
        .get_by_osu_user_id(mapper.osu_user_id)
        .await?;
    let leaderboard_positions = leaderboard_positions_repo
        .list_by_osu_user_id(mapper.osu_user_id)
        .await?;
    let charts = snapshots_repo
        .list_by_osu_user_id(mapper.osu_user_id)
        .await?;
    let user_row = osu_users_repo
        .get_by_osu_user_id(mapper.osu_user_id)
        .await?;
    let mapper_fingerprint = match user_row {
        Some(row) => Some(row.raw),
        None => None,
    };

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
) -> Vec<crate::entities::leaderboard_position_current::Model> {
    positions.sort_by_key(|row| leaderboard_order(&row.leaderboard_key));
    positions
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
