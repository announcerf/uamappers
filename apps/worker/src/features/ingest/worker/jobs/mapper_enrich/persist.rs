use sea_orm::TransactionTrait;
use uamappers_api::features::mappers::storage::{
    beatmap_profile_repo::NewBeatmapProfileRow,
    beatmapset_profile_repo::NewBeatmapsetProfileRow,
    beatmapset_snapshot_weekly_repo::NewBeatmapsetSnapshotWeeklyRow,
    mapper_profile_repo::NewMapperProfileRow,
};

use crate::shared::errors::WorkerError;

use super::cursor::{format_beatmapsets_cursor, format_last_id_cursor};
use super::types::{BEATMAPSETS_SCAN_NAME, MapperEnrich, USERS_SCAN_NAME};

pub(crate) struct BeatmapsetsPersistPage {
    pub beatmapsets: Vec<uamappers_api::entities::beatmapset::ActiveModel>,
    pub beatmapset_profiles: Vec<NewBeatmapsetProfileRow>,
    pub beatmapset_snapshots: Vec<NewBeatmapsetSnapshotWeeklyRow>,
    pub beatmap_profiles: Vec<NewBeatmapProfileRow>,
    pub beatmap_ids_by_mapset: Vec<(i64, Vec<i64>)>,
    pub beatmapset_ids: Vec<i64>,
}

impl MapperEnrich {
    pub(crate) async fn persist_user_profile(
        &self,
        osu_user_id: i64,
        raw: sea_orm::JsonValue,
        profile: NewMapperProfileRow,
        fetched_at: chrono::DateTime<chrono::Utc>,
        last_id_cursor: i64,
    ) -> Result<(), WorkerError> {
        let txn = self.osu_users_repo.db().begin().await?;

        self.osu_users_repo
            .upsert_with(&txn, osu_user_id, raw, fetched_at)
            .await?;

        self.mapper_profiles_repo.upsert_with(&txn, profile).await?;

        self.scan_state_repo
            .upsert_cursor_with(
                &txn,
                USERS_SCAN_NAME,
                Some(format_last_id_cursor(last_id_cursor)),
            )
            .await?;

        txn.commit().await?;
        Ok(())
    }

    pub(crate) async fn persist_beatmapsets_page(
        &self,
        page: BeatmapsetsPersistPage,
        osu_user_id: i64,
        kind: &str,
        cursor: super::cursor::BeatmapsetsCursor,
    ) -> Result<(), WorkerError> {
        let txn = self.beatmapsets_repo.db().begin().await?;

        self.beatmapsets_repo
            .upsert_many_with(&txn, page.beatmapsets)
            .await?;

        self.beatmapset_profiles_repo
            .upsert_many_with(&txn, page.beatmapset_profiles)
            .await?;

        self.beatmapset_snapshots_repo
            .upsert_many_with(&txn, page.beatmapset_snapshots)
            .await?;

        self.beatmap_profiles_repo
            .upsert_many_with(&txn, page.beatmap_profiles)
            .await?;

        for (osu_beatmapset_id, keep_ids) in page.beatmap_ids_by_mapset {
            self.beatmap_profiles_repo
                .delete_missing_for_mapset_with(&txn, osu_beatmapset_id, &keep_ids)
                .await?;
        }

        self.osu_user_beatmapsets_repo
            .upsert_many_with(&txn, osu_user_id, kind, &page.beatmapset_ids)
            .await?;

        self.scan_state_repo
            .upsert_cursor_with(
                &txn,
                BEATMAPSETS_SCAN_NAME,
                Some(format_beatmapsets_cursor(&cursor)),
            )
            .await?;

        txn.commit().await?;
        Ok(())
    }
}
