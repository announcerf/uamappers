use crate::shared::time::format_utc;

use super::types::{MapperEnrich, BEATMAPSETS_SCAN_NAME, USERS_SCAN_NAME};
use crate::shared::errors::WorkerError;

impl MapperEnrich {
    pub async fn run(&self) -> Result<(), WorkerError> {
        let started_at = std::time::Instant::now();
        let progress_every = self.config.progress_log_every;
        let mut cursor: Option<(chrono::DateTime<chrono::Utc>, i64)> = None;
        let mut processed = 0u64;
        let mut mapsets = 0u64;
        let mut relations = 0u64;

        tracing::info!(
            "enrich start users={} sets={}",
            on_off(self.config.enrich_users),
            on_off(self.config.enrich_beatmapsets)
        );

        loop {
            let batch = self
                .ua_mappers_repo
                .list_for_enrich_batch(cursor, self.config.batch_size as u64)
                .await?;
            if batch.is_empty() {
                break;
            }

            cursor = batch
                .last()
                .map(|mapper| (mapper.last_seen_at, mapper.osu_user_id));

            for mapper in batch {
                if self.config.enrich_users {
                    self.persist_mapper_user(&mapper).await?;
                }

                if self.config.enrich_beatmapsets {
                    let (mapper_mapsets, mapper_relations) =
                        self.persist_mapper_beatmapsets(&mapper).await?;
                    mapsets = mapsets.saturating_add(mapper_mapsets);
                    relations = relations.saturating_add(mapper_relations);
                    self.refresh_mapper_stats(mapper.osu_user_id).await?;
                }

                processed = processed.saturating_add(1);

                if progress_every > 0 && processed.is_multiple_of(progress_every) {
                    tracing::info!(
                        "enrich processed={} mapsets={} relations={} last={} seen={} {}s",
                        processed,
                        mapsets,
                        relations,
                        mapper.osu_user_id,
                        format_utc(mapper.last_seen_at),
                        started_at.elapsed().as_secs()
                    );
                }
            }
        }

        self.mark_enrich_success(processed).await?;

        tracing::info!(
            "enrich done processed={} mapsets={} relations={} {}s",
            processed,
            mapsets,
            relations,
            started_at.elapsed().as_secs()
        );
        Ok(())
    }

    async fn mark_enrich_success(&self, processed: u64) -> Result<(), WorkerError> {
        if self.config.enrich_users {
            self.scan_state_repo
                .upsert_cursor(USERS_SCAN_NAME, None)
                .await?;
            self.scan_state_repo.mark_success(USERS_SCAN_NAME).await?;
        }

        if self.config.enrich_beatmapsets {
            self.scan_state_repo
                .upsert_cursor(BEATMAPSETS_SCAN_NAME, None)
                .await?;
            self.refresh_leaderboard_positions().await?;
            self.scan_state_repo
                .mark_success(BEATMAPSETS_SCAN_NAME)
                .await?;
        }

        if processed == 0 {
            tracing::info!("enrich nothing-to-do");
        }

        Ok(())
    }
}

fn on_off(enabled: bool) -> &'static str {
    match enabled {
        true => "on",
        false => "off",
    }
}
