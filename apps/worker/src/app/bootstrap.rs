use rosu_v2::prelude::Osu;
use uamappers_api::{
    features::{
        ingest::storage::repo::ScanStateRepo,
        mappers::storage::{
            beatmapset_repo::BeatmapsetRepo, osu_user_beatmapset_repo::OsuUserBeatmapsetRepo,
            osu_user_repo::OsuUserRepo, ua_mapper_repo::UaMapperRepo,
        },
    },
    infra::db,
};

use crate::{
    features::ingest::osu_client::OsuClient,
    features::ingest::worker::jobs::{
        mapper_discovery::MapperDiscovery, mapper_enrich::MapperEnrich,
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
        ua_mappers_repo,
        OsuUserRepo::new(db.clone()),
        BeatmapsetRepo::new(db.clone()),
        OsuUserBeatmapsetRepo::new(db.clone()),
        scan_state_repo,
    );

    Ok(WorkerRuntime {
        config,
        discovery,
        enrich,
        osu_client_stats,
    })
}
