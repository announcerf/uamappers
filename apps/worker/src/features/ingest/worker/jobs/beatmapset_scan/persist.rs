use sea_orm::TransactionTrait;

use crate::shared::errors::WorkerError;

use super::types::SCAN_NAME;
use super::Scanner;

impl Scanner {
    pub(crate) async fn persist_page(
        &self,
        stats: Vec<uamappers_api::features::mappers::domain::model::MapperStats>,
        beatmapsets: Vec<uamappers_api::entities::beatmapset::ActiveModel>,
        cursor: Option<String>,
        page_index: u32,
    ) -> Result<(), WorkerError> {
        let txn = self.mappers_repo.db().begin().await?;

        self.beatmapsets_repo
            .upsert_many_with(&txn, beatmapsets)
            .await?;

        for stat in stats {
            self.mappers_repo.increment_with(&txn, &stat).await?;
        }

        self.scan_state_repo
            .upsert_cursor_with(&txn, SCAN_NAME, cursor)
            .await?;

        txn.commit().await?;

        tracing::info!(job = SCAN_NAME, page_index, "persisted scan page");
        Ok(())
    }

    pub(crate) async fn load_resume_page(&self) -> Result<u32, WorkerError> {
        let Some(state) = self.scan_state_repo.get_by_name(SCAN_NAME).await? else {
            return Ok(0);
        };

        let Some(cursor) = state.cursor else {
            return Ok(0);
        };

        let Some(value) = cursor.strip_prefix("page:") else {
            return Ok(0);
        };

        Ok(value.parse::<u32>().unwrap_or(0))
    }

    pub(crate) async fn record_failure(&self) -> Result<(), WorkerError> {
        let state = self.scan_state_repo.get_by_name(SCAN_NAME).await?;
        let retry_count = state.map(|s| s.retry_count).unwrap_or(0) + 1;

        let backoff_seconds: i64 = 30 * 2_i64.saturating_pow(retry_count.clamp(0, 10) as u32);
        let next_retry_at = chrono::Utc::now() + chrono::Duration::seconds(backoff_seconds);

        self.scan_state_repo
            .mark_error(SCAN_NAME, retry_count, Some(next_retry_at))
            .await?;

        Ok(())
    }
}
