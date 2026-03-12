use crate::shared::errors::WorkerError;
use crate::shared::time::format_utc;

use super::projection::user_to_mapper_profile_row;
use super::raw::strip_top_level_id;
use super::types::{MapperEnrich, USERS_SCAN_NAME};

impl MapperEnrich {
    pub(super) async fn run_users(&self) -> Result<(), WorkerError> {
        let started_at = std::time::Instant::now();
        let mut cursor: Option<(chrono::DateTime<chrono::Utc>, i64)> = None;

        let progress_every = self.config.progress_log_every;
        tracing::info!("users start");

        let mut users_processed: u64 = 0;

        loop {
            let batch = self
                .ua_mappers_repo
                .list_for_enrich_batch(cursor, self.config.batch_size as u64)
                .await?;
            if batch.is_empty() {
                break;
            }

            let batch_last = batch
                .last()
                .map(|mapper| (mapper.last_seen_at, mapper.osu_user_id));

            for mapper in batch {
                let extended = self.osu_client.user(mapper.osu_user_id as u32).await?;
                let raw = serde_json::to_value(&extended)
                    .map(strip_top_level_id)
                    .unwrap_or(serde_json::Value::Null);
                let fetched_at = chrono::Utc::now();
                let profile = user_to_mapper_profile_row(&extended, fetched_at);

                self.persist_user_profile(mapper.osu_user_id, raw, profile, fetched_at)
                    .await?;

                users_processed = users_processed.saturating_add(1);
                tracing::debug!(
                    job = USERS_SCAN_NAME,
                    osu_user_id = mapper.osu_user_id,
                    "persisted user profile"
                );

                if progress_every > 0 && users_processed.is_multiple_of(progress_every) {
                    let elapsed = started_at.elapsed();
                    tracing::info!(
                        "users processed={} last={} seen={} {}s",
                        users_processed,
                        mapper.osu_user_id,
                        format_utc(mapper.last_seen_at),
                        elapsed.as_secs()
                    );
                }
            }

            cursor = batch_last;
        }

        self.scan_state_repo
            .upsert_cursor(USERS_SCAN_NAME, None)
            .await?;
        self.scan_state_repo.mark_success(USERS_SCAN_NAME).await?;

        let elapsed = started_at.elapsed();
        tracing::info!("users done processed={} {}s", users_processed, elapsed.as_secs());
        Ok(())
    }
}
