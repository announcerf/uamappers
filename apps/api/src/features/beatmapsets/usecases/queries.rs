use sea_orm::DbErr;

use crate::features::mappers::storage::{
    beatmap_profile_repo::BeatmapProfileRepo, beatmapset_extra_repo::BeatmapsetExtraRepo,
    beatmapset_profile_repo::BeatmapsetProfileRepo,
    beatmapset_snapshot_weekly_repo::BeatmapsetSnapshotWeeklyRepo,
};

use super::chart_points::build_beatmapset_chart_points;
use super::types::{BeatmapsetCharts, BeatmapsetDetails};

pub async fn get_beatmapset_charts(
    beatmapset_profiles_repo: &BeatmapsetProfileRepo,
    beatmapset_snapshots_repo: &BeatmapsetSnapshotWeeklyRepo,
    osu_beatmapset_id: i64,
) -> Result<Option<BeatmapsetCharts>, DbErr> {
    let beatmapset = beatmapset_profiles_repo
        .get_by_osu_beatmapset_id(osu_beatmapset_id)
        .await?;
    let Some(beatmapset) = beatmapset else {
        return Ok(None);
    };

    let points = beatmapset_snapshots_repo
        .list_by_osu_beatmapset_id(beatmapset.osu_beatmapset_id)
        .await?;

    Ok(Some(BeatmapsetCharts {
        osu_beatmapset_id: beatmapset.osu_beatmapset_id,
        points: build_beatmapset_chart_points(points),
    }))
}

pub async fn get_beatmapset_details(
    beatmapset_extras_repo: &BeatmapsetExtraRepo,
    beatmapset_profiles_repo: &BeatmapsetProfileRepo,
    beatmap_profiles_repo: &BeatmapProfileRepo,
    beatmapset_snapshots_repo: &BeatmapsetSnapshotWeeklyRepo,
    osu_beatmapset_id: i64,
) -> Result<Option<BeatmapsetDetails>, DbErr> {
    let beatmapset = beatmapset_profiles_repo
        .get_by_osu_beatmapset_id(osu_beatmapset_id)
        .await?;
    let Some(beatmapset) = beatmapset else {
        return Ok(None);
    };

    let beatmaps = beatmap_profiles_repo
        .list_by_osu_beatmapset_id(osu_beatmapset_id)
        .await?;
    let extra = beatmapset_extras_repo
        .get_by_osu_beatmapset_id(osu_beatmapset_id)
        .await?;
    let charts = beatmapset_snapshots_repo
        .list_by_osu_beatmapset_id(osu_beatmapset_id)
        .await?;

    Ok(Some(BeatmapsetDetails {
        beatmapset,
        extra,
        beatmaps,
        charts: build_beatmapset_chart_points(charts),
    }))
}
