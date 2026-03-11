use rosu_v2::prelude::Osu;
use uamappers_api::{
    features::{
        ingest::storage::repo::ScanStateRepo,
        mappers::storage::{
            beatmap_profile_repo::BeatmapProfileRepo,
            beatmapset_profile_repo::BeatmapsetProfileRepo, beatmapset_repo::BeatmapsetRepo,
            beatmapset_snapshot_weekly_repo::BeatmapsetSnapshotWeeklyRepo,
            leaderboard_position_current_repo::LeaderboardPositionCurrentRepo,
            mapper_aggregate_snapshot_weekly_repo::MapperAggregateSnapshotWeeklyRepo,
            mapper_profile_repo::MapperProfileRepo,
            mapper_stats_current_repo::MapperStatsCurrentRepo,
            osu_user_beatmapset_repo::OsuUserBeatmapsetRepo, osu_user_repo::OsuUserRepo,
            ua_mapper_repo::UaMapperRepo,
        },
    },
    infra::db,
};

use crate::{
    features::ingest::osu_client::OsuClient,
    features::ingest::worker::jobs::{
        mapper_discovery::MapperDiscovery,
        mapper_enrich::{MapperEnrich, MapperEnrichRepos},
    },
    infra::config::WorkerConfig,
    shared::errors::WorkerError,
};

pub struct WorkerRuntime {
    pub config: WorkerConfig,
    pub discovery: MapperDiscovery,
    pub enrich: MapperEnrich,
    pub osu_client_stats: OsuClient,
}

pub async fn build_runtime(config: WorkerConfig) -> Result<WorkerRuntime, WorkerError> {
    let db = db::connect(&config.database_url).await?;
    let osu = Osu::new(config.osu_client_id, config.osu_client_secret.clone()).await?;
    let osu_client = OsuClient::new(osu);
    let osu_client_stats = osu_client.clone();
    let scan_state_repo = ScanStateRepo::new(db.clone());
    let ua_mappers_repo = UaMapperRepo::new(db.clone());

    let discovery = MapperDiscovery::new(
        osu_client.clone(),
        config.clone(),
        ua_mappers_repo.clone(),
        scan_state_repo.clone(),
    );

    let enrich = MapperEnrich::new(
        osu_client,
        config.clone(),
        MapperEnrichRepos {
            beatmap_profiles_repo: BeatmapProfileRepo::new(db.clone()),
            beatmapset_profiles_repo: BeatmapsetProfileRepo::new(db.clone()),
            beatmapset_snapshots_repo: BeatmapsetSnapshotWeeklyRepo::new(db.clone()),
            beatmapsets_repo: BeatmapsetRepo::new(db.clone()),
            leaderboard_positions_repo: LeaderboardPositionCurrentRepo::new(db.clone()),
            mapper_aggregate_snapshots_repo: MapperAggregateSnapshotWeeklyRepo::new(db.clone()),
            mapper_profiles_repo: MapperProfileRepo::new(db.clone()),
            mapper_stats_repo: MapperStatsCurrentRepo::new(db.clone()),
            osu_user_beatmapsets_repo: OsuUserBeatmapsetRepo::new(db.clone()),
            osu_users_repo: OsuUserRepo::new(db.clone()),
            scan_state_repo,
            ua_mappers_repo,
        },
    );

    Ok(WorkerRuntime {
        config,
        discovery,
        enrich,
        osu_client_stats,
    })
}
