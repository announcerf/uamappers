use rosu_v2::model::user::UserBeatmapsetsKind;

use crate::shared::errors::WorkerError;
use crate::shared::time::format_duration;

use super::beatmapset::mapset_to_row;
use super::cursor::{next_beatmapsets_cursor, parse_beatmapsets_cursor, parse_last_id_cursor};
use super::raw::strip_top_level_id;
use super::types::{MapperEnrich, BEATMAPSETS_SCAN_NAME, USERS_SCAN_NAME};

const KIND_ORDER: [UserBeatmapsetsKind; 7] = [
    UserBeatmapsetsKind::Favourite,
    UserBeatmapsetsKind::Graveyard,
    UserBeatmapsetsKind::Guest,
    UserBeatmapsetsKind::Loved,
    UserBeatmapsetsKind::Nominated,
    UserBeatmapsetsKind::Pending,
    UserBeatmapsetsKind::Ranked,
];

impl MapperEnrich {
    pub async fn run(&self) -> Result<(), WorkerError> {
        if self.config.enrich_users {
            self.run_users().await?;
        }
        if self.config.enrich_beatmapsets {
            self.run_beatmapsets().await?;
        }
        Ok(())
    }

    async fn run_users(&self) -> Result<(), WorkerError> {
        let started_at = std::time::Instant::now();
        let state = self.scan_state_repo.get_by_name(USERS_SCAN_NAME).await?;
        let mut last_id = parse_last_id_cursor(state.and_then(|s| s.cursor));

        let progress_every = self.config.progress_log_every;
        tracing::info!(
            job = USERS_SCAN_NAME,
            resume_after_osu_user_id = last_id,
            batch_size = self.config.batch_size,
            progress_log_every = progress_every,
            osu_min_request_interval_ms = self.osu_client.min_request_interval_ms(),
            "starting user enrich"
        );

        let mut users_processed: u64 = 0;

        loop {
            let batch = self
                .ua_mappers_repo
                .list_after_id(last_id, self.config.batch_size as u64)
                .await?;
            if batch.is_empty() {
                break;
            }

            for mapper in batch {
                let extended = self.osu_client.user(mapper.osu_user_id as u32).await?;
                let raw = serde_json::to_value(&extended)
                    .map(strip_top_level_id)
                    .unwrap_or(serde_json::Value::Null);
                let fetched_at = chrono::Utc::now();

                self.persist_user_profile(mapper.osu_user_id, raw, fetched_at, mapper.osu_user_id)
                    .await?;

                last_id = mapper.osu_user_id;
                users_processed = users_processed.saturating_add(1);
                tracing::debug!(
                    job = USERS_SCAN_NAME,
                    osu_user_id = mapper.osu_user_id,
                    "persisted user profile"
                );

                if progress_every > 0 && users_processed.is_multiple_of(progress_every) {
                    let elapsed = started_at.elapsed();
                    let throttle = self.osu_client.throttle_snapshot().await;
                    tracing::info!(
                        job = USERS_SCAN_NAME,
                        users_processed,
                        last_osu_user_id = last_id,
                        elapsed_ms = elapsed.as_millis() as u64,
                        elapsed = %format_duration(elapsed),
                        osu_requests = throttle.acquires,
                        osu_throttle_sleep_ms = throttle.total_sleep_ms,
                        "user enrich progress"
                    );
                }
            }
        }

        self.scan_state_repo
            .upsert_cursor(USERS_SCAN_NAME, None)
            .await?;
        self.scan_state_repo.mark_success(USERS_SCAN_NAME).await?;

        let elapsed = started_at.elapsed();
        tracing::info!(
            job = USERS_SCAN_NAME,
            users_processed,
            elapsed_ms = elapsed.as_millis() as u64,
            elapsed = %format_duration(elapsed),
            "user enrich finished"
        );
        Ok(())
    }

    async fn run_beatmapsets(&self) -> Result<(), WorkerError> {
        let started_at = std::time::Instant::now();
        let state = self
            .scan_state_repo
            .get_by_name(BEATMAPSETS_SCAN_NAME)
            .await?;
        let mut cursor = parse_beatmapsets_cursor(state.and_then(|s| s.cursor));

        let progress_every = self.config.progress_log_every;
        tracing::info!(
            job = BEATMAPSETS_SCAN_NAME,
            resume_user_id = cursor.osu_user_id,
            resume_kind_index = cursor.kind_index,
            resume_offset = cursor.offset,
            page_size = self.config.beatmapsets_page_size,
            progress_log_every = progress_every,
            osu_min_request_interval_ms = self.osu_client.min_request_interval_ms(),
            "starting beatmapsets enrich"
        );

        let mut pages_persisted: u64 = 0;
        let mut beatmapsets_persisted: u64 = 0;
        let mut relations_upserted: u64 = 0;
        let mut users_touched: u64 = 0;
        let mut kinds_touched: u64 = 0;

        let mut last_user_id: Option<i64> = None;
        let mut last_kind_index: Option<usize> = None;

        let stop_reason = loop {
            let current_user_id = self.resolve_current_user_id(cursor.osu_user_id).await?;
            let Some(current_user_id) = current_user_id else {
                break "no_users";
            };
            cursor.osu_user_id = current_user_id;

            if last_user_id != Some(current_user_id) {
                users_touched = users_touched.saturating_add(1);
                last_user_id = Some(current_user_id);
                last_kind_index = None;
            }

            if cursor.kind_index >= KIND_ORDER.len() {
                let next_user = self
                    .ua_mappers_repo
                    .list_after_id(current_user_id, 1)
                    .await?;
                let Some(next_user) = next_user.first() else {
                    break "end_of_users";
                };
                cursor.osu_user_id = next_user.osu_user_id;
                cursor.kind_index = 0;
                cursor.offset = 0;
                continue;
            }

            let kind = KIND_ORDER[cursor.kind_index];
            let kind_str = kind_to_str(kind);

            if last_kind_index != Some(cursor.kind_index) {
                kinds_touched = kinds_touched.saturating_add(1);
                last_kind_index = Some(cursor.kind_index);
            }

            let page = self
                .osu_client
                .user_beatmapsets(
                    current_user_id as u32,
                    kind,
                    self.config.beatmapsets_page_size,
                    cursor.offset,
                )
                .await?;

            let mut beatmapsets = Vec::new();
            let mut beatmapset_ids = Vec::new();
            for mapset in &page {
                let row = mapset_to_row(mapset);
                let active = self.beatmapsets_repo.to_active(row);
                beatmapsets.push(active);
                beatmapset_ids.push(mapset.mapset_id as i64);
            }

            pages_persisted = pages_persisted.saturating_add(1);
            beatmapsets_persisted = beatmapsets_persisted.saturating_add(page.len() as u64);
            relations_upserted = relations_upserted.saturating_add(beatmapset_ids.len() as u64);

            let page_size = self.config.beatmapsets_page_size;
            let next_cursor = next_beatmapsets_cursor(&cursor, page.len(), page_size);

            self.persist_beatmapsets_page(
                beatmapsets,
                current_user_id,
                kind_str,
                beatmapset_ids,
                next_cursor.clone(),
            )
            .await?;

            tracing::debug!(
                job = BEATMAPSETS_SCAN_NAME,
                osu_user_id = current_user_id,
                kind = kind_str,
                offset = cursor.offset,
                returned = page.len(),
                "persisted beatmapsets page"
            );

            if progress_every > 0 && pages_persisted.is_multiple_of(progress_every) {
                let elapsed = started_at.elapsed();
                let throttle = self.osu_client.throttle_snapshot().await;
                tracing::info!(
                    job = BEATMAPSETS_SCAN_NAME,
                    pages_persisted,
                    beatmapsets_persisted,
                    relations_upserted,
                    current_osu_user_id = current_user_id,
                    kind = kind_str,
                    offset = cursor.offset,
                    returned = page.len(),
                    elapsed_ms = elapsed.as_millis() as u64,
                    elapsed = %format_duration(elapsed),
                    osu_requests = throttle.acquires,
                    osu_throttle_sleep_ms = throttle.total_sleep_ms,
                    "beatmapsets enrich progress"
                );
            }

            cursor = next_cursor;
        };

        self.scan_state_repo
            .upsert_cursor(BEATMAPSETS_SCAN_NAME, None)
            .await?;
        self.scan_state_repo
            .mark_success(BEATMAPSETS_SCAN_NAME)
            .await?;

        let elapsed = started_at.elapsed();
        tracing::info!(
            job = BEATMAPSETS_SCAN_NAME,
            users_touched,
            kinds_touched,
            pages_persisted,
            beatmapsets_persisted,
            relations_upserted,
            removed = 0u64,
            stop_reason,
            elapsed_ms = elapsed.as_millis() as u64,
            elapsed = %format_duration(elapsed),
            "beatmapsets enrich finished"
        );
        Ok(())
    }

    async fn resolve_current_user_id(&self, requested: i64) -> Result<Option<i64>, WorkerError> {
        if requested <= 0 {
            let rows = self.ua_mappers_repo.list_after_id(0, 1).await?;
            return Ok(rows.first().map(|r| r.osu_user_id));
        }

        if self
            .ua_mappers_repo
            .get_by_osu_user_id(requested)
            .await?
            .is_some()
        {
            return Ok(Some(requested));
        }

        let rows = self.ua_mappers_repo.list_after_id(requested, 1).await?;
        Ok(rows.first().map(|r| r.osu_user_id))
    }
}

fn kind_to_str(kind: UserBeatmapsetsKind) -> &'static str {
    match kind {
        UserBeatmapsetsKind::Favourite => "favourite",
        UserBeatmapsetsKind::Graveyard => "graveyard",
        UserBeatmapsetsKind::Guest => "guest",
        UserBeatmapsetsKind::Loved => "loved",
        UserBeatmapsetsKind::Nominated => "nominated",
        UserBeatmapsetsKind::Pending => "pending",
        UserBeatmapsetsKind::Ranked => "ranked",
    }
}
