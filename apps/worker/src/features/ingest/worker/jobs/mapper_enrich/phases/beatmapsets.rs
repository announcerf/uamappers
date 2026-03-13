use rosu_v2::model::beatmap::BeatmapsetExtended;
use rosu_v2::error::OsuError;
use rosu_v2::model::user::UserBeatmapsetsKind;
use uamappers_api::features::mappers::storage::codes::{genre_code, language_code};
use uamappers_api::entities::ua_mapper;
use std::collections::HashMap;

use crate::shared::errors::WorkerError;

use super::super::projection::{build_page_payload, kind_to_str, PersistedBeatmapset};
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
    pub(in super::super) async fn persist_mapper_beatmapsets(
        &self,
        mapper: &ua_mapper::Model,
    ) -> Result<(u64, u64), WorkerError> {
        let mut beatmapsets_persisted = 0u64;
        let mut relations_upserted = 0u64;

        for kind in KIND_ORDER {
            let (_, mapsets_added, relations_added) = self
                .persist_user_kind_beatmapsets(mapper.osu_user_id, kind)
                .await?;
            beatmapsets_persisted = beatmapsets_persisted.saturating_add(mapsets_added);
            relations_upserted = relations_upserted.saturating_add(relations_added);
        }

        Ok((beatmapsets_persisted, relations_upserted))
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
            let page = self.load_beatmapset_details(page).await?;

            let page_len = page.len();
            let payload = build_page_payload(&page);
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

    async fn load_beatmapset_details(
        &self,
        page: Vec<BeatmapsetExtended>,
    ) -> Result<Vec<PersistedBeatmapset>, WorkerError> {
        let ids = page.iter().map(|row| row.mapset_id as i64).collect::<Vec<_>>();
        let existing = self
            .beatmapset_profiles_repo
            .list_by_osu_beatmapset_ids(&ids)
            .await?
            .into_iter()
            .map(|row| (row.osu_beatmapset_id, row))
            .collect::<HashMap<_, _>>();
        let extras = self
            .beatmapset_extras_repo
            .list_by_osu_beatmapset_ids(&ids)
            .await?
            .into_iter()
            .map(|row| (row.osu_beatmapset_id, row))
            .collect::<HashMap<_, _>>();
        let mut detailed = Vec::with_capacity(page.len());

        for mapset in page {
            let existing_profile = existing.get(&(mapset.mapset_id as i64));
            let existing_extra = extras.get(&(mapset.mapset_id as i64));

            if !needs_detail_fetch(existing_profile, existing_extra, &mapset) {
                detailed.push(PersistedBeatmapset {
                    mapset,
                    details_unavailable: existing_extra
                        .map(|row| row.details_unavailable)
                        .unwrap_or(false),
                });
                continue;
            }

            match self.osu_client.beatmapset(mapset.mapset_id).await {
                Ok(full) => detailed.push(PersistedBeatmapset {
                    mapset: full,
                    details_unavailable: false,
                }),
                Err(WorkerError::OsuApi(OsuError::NotFound)) => {
                    tracing::warn!(
                        job = BEATMAPSETS_SCAN_NAME,
                        osu_beatmapset_id = mapset.mapset_id,
                        "beatmapset details missing, using partial user_beatmapsets payload"
                    );
                    detailed.push(PersistedBeatmapset {
                        mapset,
                        details_unavailable: true,
                    });
                }
                Err(err) => return Err(err),
            }
        }

        Ok(detailed)
    }
}

fn needs_detail_fetch(
    existing: Option<&uamappers_api::entities::beatmapset_profile::Model>,
    extra: Option<&uamappers_api::entities::beatmapset_extra::Model>,
    mapset: &BeatmapsetExtended,
) -> bool {
    if mapset.genre.is_some() && mapset.language.is_some() {
        return false;
    }

    let Some(existing) = existing else {
        return true;
    };

    let is_same_version = existing.last_updated.timestamp() == mapset.last_updated.unix_timestamp();
    let details_unavailable = extra.map(|row| row.details_unavailable).unwrap_or(false);

    if details_unavailable && is_same_version {
        return false;
    }

    if !is_same_version {
        return true;
    }

    existing.genre == genre_code("unspecified")
        || existing.language == language_code("unspecified")
}
