use crate::features::{
    ingest::storage::repo::ScanStateRepo,
    mappers::storage::{
        beatmap_profile_repo::BeatmapProfileRepo, beatmapset_profile_repo::BeatmapsetProfileRepo,
        beatmapset_repo::BeatmapsetRepo,
        beatmapset_snapshot_weekly_repo::BeatmapsetSnapshotWeeklyRepo,
        leaderboard_position_current_repo::LeaderboardPositionCurrentRepo,
        mapper_aggregate_snapshot_weekly_repo::MapperAggregateSnapshotWeeklyRepo,
        mapper_profile_repo::MapperProfileRepo, mapper_stats_current_repo::MapperStatsCurrentRepo,
        osu_user_beatmapset_repo::OsuUserBeatmapsetRepo, osu_user_repo::OsuUserRepo,
        ua_mapper_repo::UaMapperRepo,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub beatmap_profiles_repo: BeatmapProfileRepo,
    pub beatmapset_profiles_repo: BeatmapsetProfileRepo,
    pub beatmapset_snapshots_repo: BeatmapsetSnapshotWeeklyRepo,
    pub beatmapsets_repo: BeatmapsetRepo,
    pub leaderboard_positions_repo: LeaderboardPositionCurrentRepo,
    pub mapper_aggregate_snapshots_repo: MapperAggregateSnapshotWeeklyRepo,
    pub mapper_profiles_repo: MapperProfileRepo,
    pub mapper_stats_repo: MapperStatsCurrentRepo,
    pub osu_user_beatmapsets_repo: OsuUserBeatmapsetRepo,
    pub osu_users_repo: OsuUserRepo,
    pub scan_state_repo: ScanStateRepo,
    pub ua_mappers_repo: UaMapperRepo,
}

impl AppState {
    pub fn new(db: sea_orm::DatabaseConnection) -> Self {
        Self {
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
            scan_state_repo: ScanStateRepo::new(db.clone()),
            ua_mappers_repo: UaMapperRepo::new(db.clone()),
        }
    }
}
