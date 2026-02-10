use sea_orm::TransactionTrait;

use crate::shared::errors::WorkerError;

use super::types::{DiscoveryResume, MapperDiscovery, SCAN_NAME};

impl DiscoveryResume {
    pub(crate) fn page_index(&self) -> u32 {
        match self {
            Self::Start => 0,
            Self::Page(value) => *value,
            Self::Cursor(_) => 0,
        }
    }
}

impl MapperDiscovery {
    pub(crate) async fn persist_page(
        &self,
        ua_users: Vec<(i64, String, String)>,
        cursor: Option<String>,
        page_index: u32,
    ) -> Result<(), WorkerError> {
        let txn = self.ua_mappers_repo.db().begin().await?;

        for (osu_user_id, username, country_code) in ua_users {
            self.ua_mappers_repo
                .upsert_with(&txn, osu_user_id, &username, &country_code)
                .await?;
        }

        self.scan_state_repo
            .upsert_cursor_with(&txn, SCAN_NAME, cursor)
            .await?;

        txn.commit().await?;

        tracing::debug!(job = SCAN_NAME, page_index, "persisted discovery page");
        Ok(())
    }

    pub(crate) async fn load_resume(&self) -> Result<DiscoveryResume, WorkerError> {
        if !self.config.resume_from_checkpoint {
            return Ok(DiscoveryResume::Start);
        }

        let Some(state) = self.scan_state_repo.get_by_name(SCAN_NAME).await? else {
            return Ok(DiscoveryResume::Start);
        };

        let Some(cursor) = state.cursor else {
            return Ok(DiscoveryResume::Start);
        };

        let cursor = cursor.trim().to_string();
        if cursor.is_empty() {
            return Ok(DiscoveryResume::Start);
        }

        match cursor.strip_prefix("cursor:") {
            Some(value) if !value.is_empty() => Ok(DiscoveryResume::Cursor(value.to_string())),
            _ => match cursor.strip_prefix("page:") {
                Some(value) => Ok(DiscoveryResume::Page(value.parse::<u32>().unwrap_or(0))),
                None => Ok(DiscoveryResume::Cursor(cursor)),
            },
        }
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
