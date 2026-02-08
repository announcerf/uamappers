use crate::shared::errors::WorkerError;
use crate::shared::time::format_duration;

use super::page::collect_creators;
use super::types::{MapperDiscovery, SCAN_NAME};

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
        let state = self.scan_state_repo.get_by_name(SCAN_NAME).await?;
        let cutoff = state.as_ref().and_then(|s| s.last_success_at);

        let resume_page = self.load_resume_page().await?;

        tracing::info!(
            job = SCAN_NAME,
            resume_page,
            cutoff = ?cutoff,
            force_rescan = self.config.force_rescan,
            batch_size = self.config.batch_size,
            max_pages = ?self.config.max_pages,
            scan_page_delay_ms = self.config.page_delay_ms,
            progress_log_every = self.config.progress_log_every,
            osu_min_request_interval_ms = self.osu_client.min_request_interval_ms(),
            "starting discovery scan"
        );

        let mut result = self
            .osu_client
            .beatmapset_search_resume(resume_page)
            .await?;

        let mut page_index: u32 = resume_page;
        let mut processed_this_run: u32 = 0;

        let mut pages_scanned: u32 = 0;
        let mut creators_seen: u64 = 0;
        let mut creators_existing: u64 = 0;
        let mut creators_missing: u64 = 0;
        let mut ua_added: u64 = 0;
        let mut ua_refreshed: u64 = 0;
        let mut non_ua_skipped: u64 = 0;

        let (stop_reason, stopped_at_page) = loop {
            pages_scanned = pages_scanned.saturating_add(1);
            let creators = collect_creators(&result);
            creators_seen = creators_seen.saturating_add(creators.len() as u64);
            let creator_ids_i64: Vec<i64> = creators.iter().map(|c| c.osu_user_id as i64).collect();

            let existing = self
                .ua_mappers_repo
                .list_existing_ids(&creator_ids_i64)
                .await?;
            creators_existing = creators_existing.saturating_add(existing.len() as u64);
            let missing_ids: Vec<u32> = creators
                .iter()
                .filter(|c| !existing.contains(&(c.osu_user_id as i64)))
                .map(|c| c.osu_user_id)
                .collect();
            creators_missing = creators_missing.saturating_add(missing_ids.len() as u64);

            let mut ua_users: Vec<(i64, String, String)> = Vec::new();

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
                let user_ids: Vec<u32> = chunk.to_vec();
                let users = self.osu_client.users(user_ids).await?;

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
                (false, true) => Some(format!("page:{}", page_index + 1)),
                (false, false) => None,
            };

            self.persist_page(ua_users, next_cursor, page_index).await?;

            let progress_every = self.config.progress_log_every;
            if progress_every > 0 && (pages_scanned as u64).is_multiple_of(progress_every) {
                let elapsed = started_at.elapsed();
                let throttle = self.osu_client.throttle_snapshot().await;
                let stats = self.osu_client.stats_snapshot().await;
                tracing::info!(
                    job = SCAN_NAME,
                    page_index,
                    pages_scanned,
                    creators_seen,
                    ua_added,
                    ua_refreshed,
                    non_ua_skipped,
                    elapsed_ms = elapsed.as_millis() as u64,
                    elapsed = %format_duration(elapsed),
                    osu_requests = throttle.acquires,
                    osu_retries = stats.retries,
                    osu_throttle_sleep_ms = throttle.total_sleep_ms,
                    "discovery progress"
                );
            }

            if stop_after_this_page || !has_more {
                let stop_reason = match (stop_after_this_page, has_more) {
                    (true, _) => "cutoff_reached",
                    (false, false) => "no_more_pages",
                    _ => "unknown",
                };
                break (stop_reason, Some(page_index));
            }

            processed_this_run += 1;
            if let Some(max_pages) = self.config.max_pages
                && processed_this_run >= max_pages
            {
                break ("max_pages", Some(page_index));
            }

            tokio::time::sleep(std::time::Duration::from_millis(self.config.page_delay_ms)).await;
            let Some(next) = self.osu_client.beatmapset_search_next(&result).await? else {
                break ("no_next_page", Some(page_index));
            };
            result = next;
            page_index += 1;
        };

        self.scan_state_repo.mark_success(SCAN_NAME).await?;

        let elapsed = started_at.elapsed();
        let throttle = self.osu_client.throttle_snapshot().await;
        let stats = self.osu_client.stats_snapshot().await;
        tracing::info!(
            job = SCAN_NAME,
            pages_scanned,
            creators_seen,
            creators_existing,
            creators_missing,
            ua_added,
            ua_refreshed,
            non_ua_skipped,
            removed = 0u64,
            stop_reason,
            stopped_at_page,
            elapsed_ms = elapsed.as_millis() as u64,
            elapsed = %format_duration(elapsed),
            osu_requests = throttle.acquires,
            osu_retries = stats.retries,
            osu_throttle_sleep_ms = throttle.total_sleep_ms,
            "discovery scan finished"
        );
        Ok(())
    }
}

fn page_is_before_or_equal_cutoff(
    result: &rosu_v2::prelude::BeatmapsetSearchResult,
    cutoff: chrono::DateTime<chrono::Utc>,
) -> bool {
    let mut min: Option<chrono::DateTime<chrono::Utc>> = None;
    for mapset in &result.mapsets {
        let dt = offset_to_utc(mapset.last_updated);
        min = Some(match min {
            Some(current) => current.min(dt),
            None => dt,
        });
    }

    min.is_some_and(|value| value <= cutoff)
}

fn offset_to_utc(dt: time::OffsetDateTime) -> chrono::DateTime<chrono::Utc> {
    use chrono::{TimeZone, Utc};
    let secs = dt.unix_timestamp();
    let nanos: u32 = dt.nanosecond();

    Utc.timestamp_opt(secs, nanos)
        .single()
        .unwrap_or_else(|| Utc.timestamp_opt(secs, 0).single().unwrap())
}
