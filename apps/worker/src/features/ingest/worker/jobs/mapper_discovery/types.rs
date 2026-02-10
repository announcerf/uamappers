use uamappers_api::features::ingest::storage::repo::ScanStateRepo;
use uamappers_api::features::mappers::storage::ua_mapper_repo::UaMapperRepo;

use crate::features::ingest::osu_client::OsuClient;
use crate::infra::config::WorkerConfig;

pub(crate) const SCAN_NAME: &str = "mapper_discovery";

#[derive(Clone, Debug)]
pub(crate) enum DiscoveryResume {
    Start,
    Page(u32),
    Cursor(String),
}

pub struct MapperDiscovery {
    pub(crate) osu_client: OsuClient,
    pub(crate) config: WorkerConfig,
    pub(crate) scan_state_repo: ScanStateRepo,
    pub(crate) ua_mappers_repo: UaMapperRepo,
}

impl MapperDiscovery {
    pub fn new(
        osu_client: OsuClient,
        config: WorkerConfig,
        ua_mappers_repo: UaMapperRepo,
        scan_state_repo: ScanStateRepo,
    ) -> Self {
        Self {
            osu_client,
            config,
            scan_state_repo,
            ua_mappers_repo,
        }
    }
}
