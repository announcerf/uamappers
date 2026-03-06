use crate::features::{
    ingest::storage::repo::ScanStateRepo,
    mappers::storage::{
        beatmapset_repo::BeatmapsetRepo, osu_user_beatmapset_repo::OsuUserBeatmapsetRepo,
        osu_user_repo::OsuUserRepo, ua_mapper_repo::UaMapperRepo,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub beatmapsets_repo: BeatmapsetRepo,
    pub osu_user_beatmapsets_repo: OsuUserBeatmapsetRepo,
    pub osu_users_repo: OsuUserRepo,
    pub scan_state_repo: ScanStateRepo,
    pub ua_mappers_repo: UaMapperRepo,
}

impl AppState {
    pub fn new(db: sea_orm::DatabaseConnection) -> Self {
        Self {
            beatmapsets_repo: BeatmapsetRepo::new(db.clone()),
            osu_user_beatmapsets_repo: OsuUserBeatmapsetRepo::new(db.clone()),
            osu_users_repo: OsuUserRepo::new(db.clone()),
            scan_state_repo: ScanStateRepo::new(db.clone()),
            ua_mappers_repo: UaMapperRepo::new(db.clone()),
        }
    }
}
