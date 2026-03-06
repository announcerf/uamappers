use sea_orm::TransactionTrait;

use crate::shared::errors::WorkerError;

use super::cursor::{format_beatmapsets_cursor, format_last_id_cursor};
use super::types::{BEATMAPSETS_SCAN_NAME, MapperEnrich, USERS_SCAN_NAME};

impl MapperEnrich {
    pub(crate) async fn persist_user_profile(
        &self,
        osu_user_id: i64,
        raw: sea_orm::JsonValue,
        fetched_at: chrono::DateTime<chrono::Utc>,
        last_id_cursor: i64,
    ) -> Result<(), WorkerError> {
        let txn = self.osu_users_repo.db().begin().await?;

        self.osu_users_repo
            .upsert_with(&txn, osu_user_id, raw, fetched_at)
            .await?;

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
        beatmapsets: Vec<uamappers_api::entities::beatmapset::ActiveModel>,
        osu_user_id: i64,
        kind: &str,
        beatmapset_ids: Vec<i64>,
        cursor: super::cursor::BeatmapsetsCursor,
    ) -> Result<(), WorkerError> {
        let txn = self.beatmapsets_repo.db().begin().await?;

        self.beatmapsets_repo
            .upsert_many_with(&txn, beatmapsets)
            .await?;

        self.osu_user_beatmapsets_repo
            .upsert_many_with(&txn, osu_user_id, kind, &beatmapset_ids)
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
