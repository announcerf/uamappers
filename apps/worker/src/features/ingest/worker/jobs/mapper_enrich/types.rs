use uamappers_api::features::ingest::storage::repo::ScanStateRepo;
use uamappers_api::features::mappers::storage::{
    beatmapset_repo::BeatmapsetRepo, osu_user_beatmapset_repo::OsuUserBeatmapsetRepo,
    osu_user_repo::OsuUserRepo, ua_mapper_repo::UaMapperRepo,
};

use crate::features::ingest::osu_client::OsuClient;
use crate::infra::config::WorkerConfig;

pub(crate) const USERS_SCAN_NAME: &str = "mapper_enrich_users";
pub(crate) const BEATMAPSETS_SCAN_NAME: &str = "mapper_enrich_beatmapsets";

pub struct MapperEnrich {
    pub(crate) beatmapsets_repo: BeatmapsetRepo,
    pub(crate) config: WorkerConfig,
    pub(crate) osu_client: OsuClient,
    pub(crate) osu_user_beatmapsets_repo: OsuUserBeatmapsetRepo,
    pub(crate) osu_users_repo: OsuUserRepo,
    pub(crate) scan_state_repo: ScanStateRepo,
    pub(crate) ua_mappers_repo: UaMapperRepo,
}

impl MapperEnrich {
    pub fn new(
        osu_client: OsuClient,
        config: WorkerConfig,
        ua_mappers_repo: UaMapperRepo,
        osu_users_repo: OsuUserRepo,
        beatmapsets_repo: BeatmapsetRepo,
        osu_user_beatmapsets_repo: OsuUserBeatmapsetRepo,
        scan_state_repo: ScanStateRepo,
    ) -> Self {
        Self {
            beatmapsets_repo,
            config,
            osu_client,
            osu_user_beatmapsets_repo,
            osu_users_repo,
            scan_state_repo,
            ua_mappers_repo,
        }
    }
}
