use uamappers_api::features::ingest::storage::repo::ScanStateRepo;
use uamappers_api::features::mappers::storage::{
    beatmap_profile_repo::BeatmapProfileRepo, beatmapset_extra_repo::BeatmapsetExtraRepo,
    beatmapset_profile_repo::BeatmapsetProfileRepo,
    beatmapset_snapshot_weekly_repo::BeatmapsetSnapshotWeeklyRepo,
    leaderboard_position_current_repo::LeaderboardPositionCurrentRepo,
    mapper_aggregate_snapshot_weekly_repo::MapperAggregateSnapshotWeeklyRepo,
    mapper_stats_current_repo::MapperStatsCurrentRepo, osu_user_beatmapset_repo::OsuUserBeatmapsetRepo,
    osu_user_repo::OsuUserRepo, ua_mapper_repo::UaMapperRepo,
};

use crate::features::ingest::osu_client::OsuClient;
use crate::infra::config::WorkerConfig;

pub(crate) const USERS_SCAN_NAME: &str = "mapper_enrich_users";
pub(crate) const BEATMAPSETS_SCAN_NAME: &str = "mapper_enrich_beatmapsets";

pub struct MapperEnrichRepos {
    pub beatmap_profiles_repo: BeatmapProfileRepo,
    pub beatmapset_extras_repo: BeatmapsetExtraRepo,
    pub beatmapset_profiles_repo: BeatmapsetProfileRepo,
    pub beatmapset_snapshots_repo: BeatmapsetSnapshotWeeklyRepo,
    pub leaderboard_positions_repo: LeaderboardPositionCurrentRepo,
    pub mapper_aggregate_snapshots_repo: MapperAggregateSnapshotWeeklyRepo,
    pub mapper_stats_repo: MapperStatsCurrentRepo,
    pub osu_user_beatmapsets_repo: OsuUserBeatmapsetRepo,
    pub osu_users_repo: OsuUserRepo,
    pub scan_state_repo: ScanStateRepo,
    pub ua_mappers_repo: UaMapperRepo,
}

pub struct MapperEnrich {
    pub(crate) beatmap_profiles_repo: BeatmapProfileRepo,
    pub(crate) beatmapset_extras_repo: BeatmapsetExtraRepo,
    pub(crate) beatmapset_profiles_repo: BeatmapsetProfileRepo,
    pub(crate) beatmapset_snapshots_repo: BeatmapsetSnapshotWeeklyRepo,
    pub(crate) config: WorkerConfig,
    pub(crate) leaderboard_positions_repo: LeaderboardPositionCurrentRepo,
    pub(crate) mapper_aggregate_snapshots_repo: MapperAggregateSnapshotWeeklyRepo,
    pub(crate) mapper_stats_repo: MapperStatsCurrentRepo,
    pub(crate) osu_client: OsuClient,
    pub(crate) osu_user_beatmapsets_repo: OsuUserBeatmapsetRepo,
    pub(crate) osu_users_repo: OsuUserRepo,
    pub(crate) scan_state_repo: ScanStateRepo,
    pub(crate) ua_mappers_repo: UaMapperRepo,
}

impl MapperEnrich {
    pub fn new(osu_client: OsuClient, config: WorkerConfig, repos: MapperEnrichRepos) -> Self {
        Self {
            beatmap_profiles_repo: repos.beatmap_profiles_repo,
            beatmapset_extras_repo: repos.beatmapset_extras_repo,
            beatmapset_profiles_repo: repos.beatmapset_profiles_repo,
            beatmapset_snapshots_repo: repos.beatmapset_snapshots_repo,
            config,
            leaderboard_positions_repo: repos.leaderboard_positions_repo,
            mapper_aggregate_snapshots_repo: repos.mapper_aggregate_snapshots_repo,
            mapper_stats_repo: repos.mapper_stats_repo,
            osu_client,
            osu_user_beatmapsets_repo: repos.osu_user_beatmapsets_repo,
            osu_users_repo: repos.osu_users_repo,
            scan_state_repo: repos.scan_state_repo,
            ua_mappers_repo: repos.ua_mappers_repo,
        }
    }
}
