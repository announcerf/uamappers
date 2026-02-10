use crate::shared::errors::WorkerError;

use super::page::collect_creators;
use super::types::{DiscoveryResume, MapperDiscovery};

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

        let mut page_index: u32 = resume.page_index();
        let mut processed_this_run: u32 = 0;

        let mut pages_scanned: u32 = 0;
        let mut _creators_seen: u64 = 0;
        let mut _creators_existing: u64 = 0;
        let mut _creators_missing: u64 = 0;
        let mut ua_added: u64 = 0;
        let mut ua_refreshed: u64 = 0;
        let mut non_ua_skipped: u64 = 0;
        let mut _page_delay_sleeps: u64 = 0;
        let mut _page_delay_sleep_ms_total: u64 = 0;

        let (_stop_reason, _stopped_at_page) = loop {
            pages_scanned = pages_scanned.saturating_add(1);
            let creators = collect_creators(&result);
            _creators_seen = _creators_seen.saturating_add(creators.len() as u64);
            let creator_ids_i64: Vec<i64> = creators.iter().map(|c| c.osu_user_id as i64).collect();

            let existing = self
                .ua_mappers_repo
                .list_existing_ids(&creator_ids_i64)
                .await?;
            _creators_existing = _creators_existing.saturating_add(existing.len() as u64);
            let missing_ids: Vec<u32> = creators
                .iter()
                .filter(|c| !existing.contains(&(c.osu_user_id as i64)))
                .map(|c| c.osu_user_id)
                .collect();
            _creators_missing = _creators_missing.saturating_add(missing_ids.len() as u64);

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
                (false, true) => encode_search_cursor(&result),
                (false, false) => None,
            };

            let save_checkpoint = !self.config.discovery_oldest_first;
            self.persist_page(ua_users, next_cursor, save_checkpoint, page_index)
                .await?;

            let progress_every = self.config.progress_log_every;
            if progress_every > 0 && (pages_scanned as u64).is_multiple_of(progress_every) {
                let elapsed = started_at.elapsed();
                let _ = page_index;
                tracing::info!(
                    "discovery page={} ua+{} ua~{} skip{} {}s",
                    pages_scanned,
                    ua_added,
                    ua_refreshed,
                    non_ua_skipped,
                    elapsed.as_secs()
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

            let delay_ms = self.config.page_delay_ms;
            if delay_ms > 0 {
                _page_delay_sleeps = _page_delay_sleeps.saturating_add(1);
                _page_delay_sleep_ms_total = _page_delay_sleep_ms_total.saturating_add(delay_ms);
            }
            tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
            let Some(next) = self.osu_client.beatmapset_search_next(&result).await? else {
                break ("no_next_page", Some(page_index));
            };
            result = next;
            page_index += 1;
        };

        if !self.config.discovery_oldest_first {
            self.scan_state_repo.mark_success(self.scan_name()).await?;
        }

        let elapsed = started_at.elapsed();
        tracing::info!(
            "discovery done page={} ua+{} ua~{} skip{} {}s",
            pages_scanned,
            ua_added,
            ua_refreshed,
            non_ua_skipped,
            elapsed.as_secs()
        );
        Ok(())
    }
}

fn encode_search_cursor(result: &rosu_v2::prelude::BeatmapsetSearchResult) -> Option<String> {
    let value = serde_json::to_value(result).ok()?;
    let cursor = value.get("cursor_string")?.as_str()?;
    if cursor.is_empty() {
        return None;
    }
    Some(format!("cursor:{}", cursor))
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
