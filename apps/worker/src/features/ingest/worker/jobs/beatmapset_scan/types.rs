use uamappers_api::features::{
    ingest::storage::repo::ScanStateRepo,
    mappers::storage::{beatmapset_repo::BeatmapsetRepo, repo::MapperRepo},
};

use crate::infra::config::WorkerConfig;

use crate::features::ingest::osu_client::OsuClient;

pub(crate) const SCAN_NAME: &str = "beatmapset_search";

#[derive(Default, Debug)]
pub(crate) struct EnrichedPage {
    pub(crate) stats: Vec<uamappers_api::features::mappers::domain::model::MapperStats>,
    pub(crate) beatmapsets: Vec<uamappers_api::entities::beatmapset::ActiveModel>,
}

pub struct Scanner {
    pub(crate) osu_client: OsuClient,
    pub(crate) config: WorkerConfig,
    pub(crate) mappers_repo: MapperRepo,
    pub(crate) beatmapsets_repo: BeatmapsetRepo,
    pub(crate) scan_state_repo: ScanStateRepo,
}

impl Scanner {
    pub fn new(
        osu: rosu_v2::prelude::Osu,
        config: WorkerConfig,
        mappers_repo: MapperRepo,
        beatmapsets_repo: BeatmapsetRepo,
        scan_state_repo: ScanStateRepo,
    ) -> Self {
        Self {
            osu_client: OsuClient::new(osu),
            config,
            mappers_repo,
            beatmapsets_repo,
            scan_state_repo,
        }
    }
}
