use crate::shared::errors::WorkerError;

use super::super::creators::collect_creators;
use super::super::types::{DiscoveryResume, MapperDiscovery};
use super::search::{encode_search_cursor, page_is_before_or_equal_cutoff};

impl MapperDiscovery {
    pub async fn run(&self) -> Result<(), WorkerError> {
        match self.run_inner().await {
            Ok(()) => Ok(()),
            Err(err) => {
                if let Err(mark_err) = self.record_failure().await {
                    tracing::error!(error = ?mark_err, "failed to persist scan failure state");
                }
                Err(err)
            }
        }
    }

    async fn run_inner(&self) -> Result<(), WorkerError> {
        let started_at = std::time::Instant::now();
        let cutoff = match self.config.discovery_oldest_first {
            true => None,
            false => self
                .scan_state_repo
                .get_by_name(self.scan_name())
                .await?
                .as_ref()
                .and_then(|s| s.last_success_at),
        };
        let resume = self.load_resume().await?;

        match &resume {
            DiscoveryResume::Start => tracing::info!("discovery start p0"),
            DiscoveryResume::Page(page) => tracing::info!("discovery start p{}", page),
            DiscoveryResume::Cursor(_) => tracing::info!("discovery start cursor"),
        }

        let descending = !self.config.discovery_oldest_first;
        let mut result = match &resume {
            DiscoveryResume::Start => {
                self.osu_client
                    .beatmapset_search_resume(0, descending)
                    .await?
            }
            DiscoveryResume::Page(page) => {
                self.osu_client
                    .beatmapset_search_resume(*page, descending)
                    .await?
            }
            DiscoveryResume::Cursor(cursor) => {
                self.osu_client
                    .beatmapset_search_from_cursor_string(cursor, descending)
                    .await?
            }
        };

        let mut page_index = resume.page_index();
        let mut processed_this_run = 0u32;
        let mut pages_scanned = 0u32;
        let mut ua_added = 0u64;
        let mut ua_refreshed = 0u64;
        let mut non_ua_skipped = 0u64;

        let (_stop_reason, _stopped_at_page) = loop {
            pages_scanned = pages_scanned.saturating_add(1);
            let creators = collect_creators(&result);
            let creator_ids_i64: Vec<i64> = creators
                .iter()
                .map(|creator| creator.osu_user_id as i64)
                .collect();
            let existing = self
                .ua_mappers_repo
                .list_existing_ids(&creator_ids_i64)
                .await?;
            let missing_ids: Vec<u32> = creators
                .iter()
                .filter(|creator| !existing.contains(&(creator.osu_user_id as i64)))
                .map(|creator| creator.osu_user_id)
                .collect();

            let mut ua_users = Vec::new();
            for creator in &creators {
                if !existing.contains(&(creator.osu_user_id as i64)) {
                    continue;
                }

                ua_refreshed = ua_refreshed.saturating_add(1);
                ua_users.push((
                    creator.osu_user_id as i64,
                    creator.username.clone(),
                    "UA".to_string(),
                ));
            }

            for chunk in missing_ids.chunks(self.config.batch_size) {
                let users = self.osu_client.users(chunk.iter().copied()).await?;
                for user in users {
                    if user.country_code != "UA" {
                        non_ua_skipped = non_ua_skipped.saturating_add(1);
                        continue;
                    }

                    ua_added = ua_added.saturating_add(1);
                    ua_users.push((
                        user.user_id as i64,
                        user.username.to_string(),
                        user.country_code.to_string(),
                    ));
                }
            }

            let stop_after_this_page = match (cutoff, self.config.force_rescan) {
                (Some(cutoff), false) => page_is_before_or_equal_cutoff(&result, cutoff),
                _ => false,
            };
            let has_more = result.has_more();
            let next_cursor = match (stop_after_this_page, has_more) {
                (true, _) => None,
                (false, true) => encode_search_cursor(&result),
                (false, false) => None,
            };

            self.persist_page(
                ua_users,
                next_cursor,
                !self.config.discovery_oldest_first,
                page_index,
            )
            .await?;
            log_progress(
                self.config.progress_log_every,
                pages_scanned,
                ua_added,
                ua_refreshed,
                non_ua_skipped,
                started_at.elapsed(),
            );

            if stop_after_this_page || !has_more {
                let stop_reason = match (stop_after_this_page, has_more) {
                    (true, _) => "cutoff_reached",
                    (false, false) => "no_more_pages",
                    _ => "unknown",
                };
                break (stop_reason, Some(page_index));
            }

            processed_this_run = processed_this_run.saturating_add(1);
            if self.reached_max_pages(processed_this_run) {
                break ("max_pages", Some(page_index));
            }

            tokio::time::sleep(std::time::Duration::from_millis(self.config.page_delay_ms)).await;
            let Some(next) = self.osu_client.beatmapset_search_next(&result).await? else {
                break ("no_next_page", Some(page_index));
            };
            result = next;
            page_index = page_index.saturating_add(1);
        };

        if !self.config.discovery_oldest_first {
            self.scan_state_repo.mark_success(self.scan_name()).await?;
        }

        tracing::info!(
            "discovery done pages={} added={} refreshed={} skipped={} {}s",
            pages_scanned,
            ua_added,
            ua_refreshed,
            non_ua_skipped,
            started_at.elapsed().as_secs()
        );
        Ok(())
    }

    fn reached_max_pages(&self, processed_this_run: u32) -> bool {
        match self.config.max_pages {
            Some(max_pages) => processed_this_run >= max_pages,
            None => false,
        }
    }
}

fn log_progress(
    progress_every: u64,
    pages_scanned: u32,
    ua_added: u64,
    ua_refreshed: u64,
    non_ua_skipped: u64,
    elapsed: std::time::Duration,
) {
    if progress_every == 0 || !(pages_scanned as u64).is_multiple_of(progress_every) {
        return;
    }

    tracing::info!(
        "discovery pages={} added={} refreshed={} skipped={} {}s",
        pages_scanned,
        ua_added,
        ua_refreshed,
        non_ua_skipped,
        elapsed.as_secs()
    );
}
