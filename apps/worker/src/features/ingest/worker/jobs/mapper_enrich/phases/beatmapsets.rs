use rosu_v2::model::user::UserBeatmapsetsKind;

use crate::shared::errors::WorkerError;
use crate::shared::time::format_utc;

use super::super::projection::{build_page_payload, kind_to_str};
use super::super::storage::persist_beatmapsets_page;
use super::super::types::{MapperEnrich, BEATMAPSETS_SCAN_NAME};
use super::{next_beatmapsets_cursor, BeatmapsetsCursor};

const KIND_ORDER: [UserBeatmapsetsKind; 6] = [
    UserBeatmapsetsKind::Graveyard,
    UserBeatmapsetsKind::Guest,
    UserBeatmapsetsKind::Loved,
    UserBeatmapsetsKind::Nominated,
    UserBeatmapsetsKind::Pending,
    UserBeatmapsetsKind::Ranked,
];

impl MapperEnrich {
    pub(in super::super) async fn run_beatmapsets(&self) -> Result<(), WorkerError> {
        let started_at = std::time::Instant::now();
        let mut cursor: Option<(chrono::DateTime<chrono::Utc>, i64)> = None;
        let progress_every = self.config.progress_log_every;
        let mut mappers_processed = 0u64;
        let mut beatmapsets_persisted = 0u64;
        let mut relations_upserted = 0u64;

        tracing::info!("sets start");

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
                for kind in KIND_ORDER {
                    let (_, mapsets_added, relations_added) = self
                        .persist_user_kind_beatmapsets(mapper.osu_user_id, kind)
                        .await?;
                    beatmapsets_persisted = beatmapsets_persisted.saturating_add(mapsets_added);
                    relations_upserted = relations_upserted.saturating_add(relations_added);
                }

                self.refresh_mapper_stats(mapper.osu_user_id).await?;
                mappers_processed = mappers_processed.saturating_add(1);

                if progress_every > 0 && mappers_processed.is_multiple_of(progress_every) {
                    tracing::info!(
                        "sets processed={} mapsets={} relations={} last={} seen={} {}s",
                        mappers_processed,
                        beatmapsets_persisted,
                        relations_upserted,
                        mapper.osu_user_id,
                        format_utc(mapper.last_seen_at),
                        started_at.elapsed().as_secs()
                    );
                }
            }
        }

        self.scan_state_repo
            .upsert_cursor(BEATMAPSETS_SCAN_NAME, None)
            .await?;
        self.refresh_leaderboard_positions().await?;
        self.scan_state_repo
            .mark_success(BEATMAPSETS_SCAN_NAME)
            .await?;

        tracing::info!(
            "sets done processed={} mapsets={} relations={} {}s",
            mappers_processed,
            beatmapsets_persisted,
            relations_upserted,
            started_at.elapsed().as_secs()
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
            let payload = build_page_payload(&self.beatmapsets_repo, &page);
            pages_persisted = pages_persisted.saturating_add(1);
            beatmapsets_persisted = beatmapsets_persisted.saturating_add(page_len as u64);
            relations_upserted =
                relations_upserted.saturating_add(payload.beatmapset_ids.len() as u64);

            let next_cursor =
                next_beatmapsets_cursor(&cursor, page_len, self.config.beatmapsets_page_size);
            persist_beatmapsets_page(self, payload, osu_user_id, kind_str).await?;

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
