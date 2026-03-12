use rosu_v2::model::user::UserBeatmapsetsKind;

use crate::shared::errors::WorkerError;
use crate::shared::time::format_utc;

use super::beatmapset::mapset_to_row;
use super::cursor::{next_beatmapsets_cursor, BeatmapsetsCursor};
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
        let mut cursor: Option<(chrono::DateTime<chrono::Utc>, i64)> = None;

        let progress_every = self.config.progress_log_every;
        tracing::info!("sets start");

        let mut mappers_processed: u64 = 0;
        let mut pages_persisted: u64 = 0;
        let mut beatmapsets_persisted: u64 = 0;
        let mut relations_upserted: u64 = 0;
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
                for kind in KIND_ORDER {
                    let kind_rows = self
                        .persist_user_kind_beatmapsets(mapper.osu_user_id, kind)
                        .await?;
                    pages_persisted = pages_persisted.saturating_add(kind_rows.0);
                    beatmapsets_persisted = beatmapsets_persisted.saturating_add(kind_rows.1);
                    relations_upserted = relations_upserted.saturating_add(kind_rows.2);
                }

                self.refresh_mapper_stats(mapper.osu_user_id).await?;
                mappers_processed = mappers_processed.saturating_add(1);

                if progress_every > 0 && mappers_processed.is_multiple_of(progress_every) {
                    let elapsed = started_at.elapsed();
                    tracing::info!(
                        "sets processed={} mapsets={} relations={} last={} seen={} {}s",
                        mappers_processed,
                        beatmapsets_persisted,
                        relations_upserted,
                        mapper.osu_user_id,
                        format_utc(mapper.last_seen_at),
                        elapsed.as_secs()
                    );
                }
            }

            cursor = batch_last;
        }

        self.scan_state_repo
            .upsert_cursor(BEATMAPSETS_SCAN_NAME, None)
            .await?;
        self.refresh_leaderboard_positions().await?;
        self.scan_state_repo
            .mark_success(BEATMAPSETS_SCAN_NAME)
            .await?;

        let elapsed = started_at.elapsed();
        tracing::info!(
            "sets done processed={} mapsets={} relations={} {}s",
            mappers_processed,
            beatmapsets_persisted,
            relations_upserted,
            elapsed.as_secs()
        );
        Ok(())
    }

    async fn persist_user_kind_beatmapsets(
        &self,
        osu_user_id: i64,
        kind: UserBeatmapsetsKind,
    ) -> Result<(u64, u64, u64), WorkerError> {
        let kind_str = kind_to_str(kind);
        let mut cursor = BeatmapsetsCursor::start();

        let mut pages_persisted = 0u64;
        let mut beatmapsets_persisted = 0u64;
        let mut relations_upserted = 0u64;

        loop {
            let page = self
                .osu_client
                .user_beatmapsets(
                    osu_user_id as u32,
                    kind,
                    self.config.beatmapsets_page_size,
                    cursor.offset,
                )
                .await?;

            let page_len = page.len();
            let (
                beatmapsets,
                beatmapset_profiles,
                beatmapset_snapshots,
                beatmap_profiles,
                beatmap_ids_by_mapset,
                beatmapset_ids,
            ) = build_page_payload(&self.beatmapsets_repo, &page);

            pages_persisted = pages_persisted.saturating_add(1);
            beatmapsets_persisted = beatmapsets_persisted.saturating_add(page_len as u64);
            relations_upserted = relations_upserted.saturating_add(beatmapset_ids.len() as u64);

            let next_cursor =
                next_beatmapsets_cursor(&cursor, page_len, self.config.beatmapsets_page_size);

            self.persist_beatmapsets_page(
                BeatmapsetsPersistPage {
                    beatmapsets,
                    beatmapset_profiles,
                    beatmapset_snapshots,
                    beatmap_profiles,
                    beatmap_ids_by_mapset,
                    beatmapset_ids,
                },
                osu_user_id,
                kind_str,
            )
            .await?;

            tracing::debug!(
                job = BEATMAPSETS_SCAN_NAME,
                osu_user_id,
                kind = kind_str,
                offset = cursor.offset,
                returned = page_len,
                "persisted beatmapsets page"
            );

            if page_len < self.config.beatmapsets_page_size {
                break;
            }

            cursor = next_cursor;
        }

        Ok((pages_persisted, beatmapsets_persisted, relations_upserted))
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

type BeatmapsetPagePayload = (
    Vec<uamappers_api::entities::beatmapset::ActiveModel>,
    Vec<uamappers_api::features::mappers::storage::beatmapset_profile_repo::NewBeatmapsetProfileRow>,
    Vec<uamappers_api::features::mappers::storage::beatmapset_snapshot_weekly_repo::NewBeatmapsetSnapshotWeeklyRow>,
    Vec<uamappers_api::features::mappers::storage::beatmap_profile_repo::NewBeatmapProfileRow>,
    Vec<(i64, Vec<i64>)>,
    Vec<i64>,
);

fn build_page_payload(
    beatmapsets_repo: &uamappers_api::features::mappers::storage::beatmapset_repo::BeatmapsetRepo,
    page: &[rosu_v2::model::beatmap::BeatmapsetExtended],
) -> BeatmapsetPagePayload {
    let mut beatmapsets = Vec::new();
    let mut beatmapset_profiles = Vec::new();
    let mut beatmapset_snapshots = Vec::new();
    let mut beatmap_profiles = Vec::new();
    let mut beatmap_ids_by_mapset = Vec::new();
    let mut beatmapset_ids = Vec::new();
    let cached_at = chrono::Utc::now();
    let weekly_snapshot = snapshot_week(cached_at);

    for mapset in page {
        let row = mapset_to_row(mapset);
        let active = beatmapsets_repo.to_active(row);
        beatmapsets.push(active);
        beatmapset_ids.push(mapset.mapset_id as i64);
        beatmapset_profiles.push(mapset_to_profile_row(mapset, cached_at));
        beatmapset_snapshots.push(mapset_to_snapshot_row(mapset, weekly_snapshot));

        let nested_rows = maps_to_profile_rows(mapset, cached_at);
        let keep_ids = nested_rows.iter().map(|row| row.osu_beatmap_id).collect();
        beatmap_ids_by_mapset.push((mapset.mapset_id as i64, keep_ids));
        beatmap_profiles.extend(nested_rows);
    }

    (
        beatmapsets,
        beatmapset_profiles,
        beatmapset_snapshots,
        beatmap_profiles,
        beatmap_ids_by_mapset,
        beatmapset_ids,
    )
}
