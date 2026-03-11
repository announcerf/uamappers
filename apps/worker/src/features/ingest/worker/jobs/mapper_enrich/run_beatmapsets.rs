use rosu_v2::model::user::UserBeatmapsetsKind;

use crate::shared::errors::WorkerError;

use super::beatmapset::mapset_to_row;
use super::cursor::{next_beatmapsets_cursor, parse_beatmapsets_cursor};
use super::persist::BeatmapsetsPersistPage;
use super::projection::{maps_to_profile_rows, mapset_to_profile_row};
use super::snapshot::{mapset_to_snapshot_row, snapshot_week};
use super::types::{MapperEnrich, BEATMAPSETS_SCAN_NAME};

const KIND_ORDER: [UserBeatmapsetsKind; 6] = [
    UserBeatmapsetsKind::Graveyard,
    UserBeatmapsetsKind::Guest,
    UserBeatmapsetsKind::Loved,
    UserBeatmapsetsKind::Nominated,
    UserBeatmapsetsKind::Pending,
    UserBeatmapsetsKind::Ranked,
];

impl MapperEnrich {
    pub(super) async fn run_beatmapsets(&self) -> Result<(), WorkerError> {
        let started_at = std::time::Instant::now();
        let state = self
            .scan_state_repo
            .get_by_name(BEATMAPSETS_SCAN_NAME)
            .await?;
        let mut cursor = parse_beatmapsets_cursor(state.and_then(|s| s.cursor));

        let progress_every = self.config.progress_log_every;
        tracing::info!("sets start");

        let mut pages_persisted: u64 = 0;
        let mut beatmapsets_persisted: u64 = 0;
        let mut relations_upserted: u64 = 0;
        let mut _users_touched: u64 = 0;
        let mut _kinds_touched: u64 = 0;

        let mut last_user_id: Option<i64> = None;
        let mut last_kind_index: Option<usize> = None;

        let stop_reason = loop {
            let current_user_id = self.resolve_current_user_id(cursor.osu_user_id).await?;
            let Some(current_user_id) = current_user_id else {
                break "no_users";
            };
            cursor.osu_user_id = current_user_id;

            if last_user_id != Some(current_user_id) {
                _users_touched = _users_touched.saturating_add(1);
                last_user_id = Some(current_user_id);
                last_kind_index = None;
            }

            if cursor.kind_index >= KIND_ORDER.len() {
                self.refresh_mapper_stats(current_user_id).await?;
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
                _kinds_touched = _kinds_touched.saturating_add(1);
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
            let mut beatmapset_profiles = Vec::new();
            let mut beatmapset_snapshots = Vec::new();
            let mut beatmap_profiles = Vec::new();
            let mut beatmap_ids_by_mapset = Vec::new();
            let mut beatmapset_ids = Vec::new();
            let cached_at = chrono::Utc::now();
            let weekly_snapshot = snapshot_week(cached_at);

            for mapset in &page {
                let row = mapset_to_row(mapset);
                let active = self.beatmapsets_repo.to_active(row);
                beatmapsets.push(active);
                beatmapset_ids.push(mapset.mapset_id as i64);
                beatmapset_profiles.push(mapset_to_profile_row(mapset, cached_at));
                beatmapset_snapshots.push(mapset_to_snapshot_row(mapset, weekly_snapshot));

                let nested_rows = maps_to_profile_rows(mapset, cached_at);
                let keep_ids = nested_rows.iter().map(|row| row.osu_beatmap_id).collect();
                beatmap_ids_by_mapset.push((mapset.mapset_id as i64, keep_ids));
                beatmap_profiles.extend(nested_rows);
            }

            pages_persisted = pages_persisted.saturating_add(1);
            beatmapsets_persisted = beatmapsets_persisted.saturating_add(page.len() as u64);
            relations_upserted = relations_upserted.saturating_add(beatmapset_ids.len() as u64);

            let page_size = self.config.beatmapsets_page_size;
            let next_cursor = next_beatmapsets_cursor(&cursor, page.len(), page_size);

            self.persist_beatmapsets_page(
                BeatmapsetsPersistPage {
                    beatmapsets,
                    beatmapset_profiles,
                    beatmapset_snapshots,
                    beatmap_profiles,
                    beatmap_ids_by_mapset,
                    beatmapset_ids,
                },
                current_user_id,
                kind_str,
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
                tracing::info!(
                    "sets page={} sets{} rel{} {}s",
                    pages_persisted,
                    beatmapsets_persisted,
                    relations_upserted,
                    elapsed.as_secs()
                );
            }

            cursor = next_cursor;
        };

        self.scan_state_repo
            .upsert_cursor(BEATMAPSETS_SCAN_NAME, None)
            .await?;
        self.refresh_leaderboard_positions().await?;
        self.scan_state_repo
            .mark_success(BEATMAPSETS_SCAN_NAME)
            .await?;

        let elapsed = started_at.elapsed();
        let _ = stop_reason;
        tracing::info!(
            "sets done page={} sets{} rel{} {}s",
            pages_persisted,
            beatmapsets_persisted,
            relations_upserted,
            elapsed.as_secs()
        );
        Ok(())
    }
}

fn kind_to_str(kind: UserBeatmapsetsKind) -> &'static str {
    match kind {
        UserBeatmapsetsKind::Graveyard => "graveyard",
        UserBeatmapsetsKind::Guest => "guest",
        UserBeatmapsetsKind::Loved => "loved",
        UserBeatmapsetsKind::Nominated => "nominated",
        UserBeatmapsetsKind::Pending => "pending",
        UserBeatmapsetsKind::Ranked => "ranked",
        _ => unreachable!("unsupported beatmapset kind"),
    }
}
