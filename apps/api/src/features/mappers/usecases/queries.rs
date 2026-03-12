use sea_orm::DbErr;

use crate::features::mappers::storage::{
    beatmapset_extra_repo::BeatmapsetExtraRepo, beatmapset_profile_repo::BeatmapsetProfileRepo,
    mapper_aggregate_snapshot_weekly_repo::MapperAggregateSnapshotWeeklyRepo,
    osu_user_beatmapset_repo::OsuUserBeatmapsetRepo, ua_mapper_repo::UaMapperRepo,
};

use super::chart_points::build_mapper_chart_points;
use super::profile::load_mapper_profile;
use super::types::{
    BeatmapsetListItem, BeatmapsetPage, CursorInput, MapperCharts, MapperPage, MapperProfile,
    PageInput,
};
use super::MapperProfileReadRepos;

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
    read_repos: MapperProfileReadRepos<'_>,
    username: &str,
) -> Result<Option<MapperProfile>, DbErr> {
    let mapper = ua_mappers_repo.get_by_username(username).await?;

    load_mapper_profile(read_repos, mapper).await
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
        points: build_mapper_chart_points(points),
    }))
}

pub async fn get_mapper_profile_by_id(
    ua_mappers_repo: &UaMapperRepo,
    read_repos: MapperProfileReadRepos<'_>,
    osu_user_id: i64,
) -> Result<Option<MapperProfile>, DbErr> {
    let mapper = ua_mappers_repo.get_by_osu_user_id(osu_user_id).await?;

    load_mapper_profile(read_repos, mapper).await
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
        points: build_mapper_chart_points(points),
    }))
}

pub async fn list_mapper_beatmapsets_by_username(
    ua_mappers_repo: &UaMapperRepo,
    osu_user_beatmapsets_repo: &OsuUserBeatmapsetRepo,
    beatmapset_extras_repo: &BeatmapsetExtraRepo,
    beatmapset_profiles_repo: &BeatmapsetProfileRepo,
    username: &str,
    kind: &str,
    page: PageInput,
) -> Result<Option<BeatmapsetPage>, DbErr> {
    let mapper = ua_mappers_repo.get_by_username(username).await?;

    let Some(mapper) = mapper else {
        return Ok(None);
    };

    let beatmapsets =
        list_mapper_beatmapsets_by_id(
            osu_user_beatmapsets_repo,
            beatmapset_extras_repo,
            beatmapset_profiles_repo,
            mapper.osu_user_id,
            kind,
            page,
        )
        .await?;

    Ok(Some(beatmapsets))
}

pub async fn list_mapper_beatmapsets_by_id(
    osu_user_beatmapsets_repo: &OsuUserBeatmapsetRepo,
    beatmapset_extras_repo: &BeatmapsetExtraRepo,
    beatmapset_profiles_repo: &BeatmapsetProfileRepo,
    osu_user_id: i64,
    kind: &str,
    page: PageInput,
) -> Result<BeatmapsetPage, DbErr> {
    let (ids, total) = osu_user_beatmapsets_repo
        .list_beatmapset_ids(osu_user_id, kind, page.limit, page.offset)
        .await?;
    let items = beatmapset_profiles_repo
        .list_by_osu_beatmapset_ids(&ids)
        .await?;
    let extras = beatmapset_extras_repo.list_by_osu_beatmapset_ids(&ids).await?;
    let items = order_beatmapsets(items, extras, &ids);

    Ok(BeatmapsetPage {
        items,
        limit: page.limit,
        offset: page.offset,
        total,
    })
}

fn order_beatmapsets(
    items: Vec<crate::entities::beatmapset_profile::Model>,
    extras: Vec<crate::entities::beatmapset_extra::Model>,
    ids: &[i64],
) -> Vec<BeatmapsetListItem> {
    use std::collections::HashMap;

    let mut by_id = items
        .into_iter()
        .map(|row| (row.osu_beatmapset_id, row))
        .collect::<HashMap<_, _>>();
    let mut extras_by_id = extras
        .into_iter()
        .map(|row| (row.osu_beatmapset_id, row))
        .collect::<HashMap<_, _>>();

    ids.iter()
        .filter_map(|id| {
            by_id.remove(id).map(|profile| BeatmapsetListItem {
                profile,
                extra: extras_by_id.remove(id),
            })
        })
        .collect()
}
