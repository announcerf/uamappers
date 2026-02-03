use std::collections::HashMap;
use std::future::IntoFuture;
use std::time::Duration;

use rosu_v2::prelude::{BeatmapsetSearchResult, Osu, RankStatus};
use sea_orm::TransactionTrait;
use tokio::time::sleep;

use uamappers_api::features::{
    ingest::storage::repo::ScanStateRepo,
    mappers::{domain::model::MapperStats, storage::repo::MapperRepo},
};

use crate::{config::WorkerConfig, error::WorkerError};

const SCAN_NAME: &str = "beatmapset_search";

#[derive(Default, Debug, Clone)]
struct MapperAccumulator {
    count_graveyard: i32,
    count_pending: i32,
    count_wip: i32,
    count_loved: i32,
    count_ranked: i32,
    count_approved: i32,
    count_total: i32,
}

impl MapperAccumulator {
    fn add_status(&mut self, status: RankStatus) {
        self.count_total += 1;
        match status {
            RankStatus::Graveyard => self.count_graveyard += 1,
            RankStatus::Pending => self.count_pending += 1,
            RankStatus::WIP => self.count_wip += 1,
            RankStatus::Loved => self.count_loved += 1,
            RankStatus::Ranked => self.count_ranked += 1,
            RankStatus::Approved => self.count_approved += 1,
            RankStatus::Qualified => self.count_pending += 1,
        }
    }
}

pub struct Scanner {
    osu: Osu,
    config: WorkerConfig,
    mappers_repo: MapperRepo,
    scan_state_repo: ScanStateRepo,
}

impl Scanner {
    pub fn new(
        osu: Osu,
        config: WorkerConfig,
        mappers_repo: MapperRepo,
        scan_state_repo: ScanStateRepo,
    ) -> Self {
        Self {
            osu,
            config,
            mappers_repo,
            scan_state_repo,
        }
    }

    pub async fn run(&self) -> Result<(), WorkerError> {
        let mut page_index = self.load_resume_page().await?;

        let mut result = self
            .retry(|| self.osu.beatmapset_search().query("status=any").nsfw(true))
            .await?;

        if page_index > 0 {
            tracing::info!(page_index, "resuming beatmapset search from page");
        }

        while page_index > 0 {
            if !result.has_more() {
                break;
            }
            let Some(next) = self.next_page(&result).await? else {
                break;
            };
            result = next;
            page_index -= 1;
        }

        let mut processed_pages: u32 = 0;

        loop {
            let accumulators = self.collect_page(&result);
            let page_stats = self.enrich_page(accumulators).await?;

            processed_pages += 1;
            self.persist_page(page_stats, processed_pages).await?;

            if !result.has_more() {
                break;
            }

            if let Some(max_pages) = self.config.max_pages
                && processed_pages >= max_pages
            {
                break;
            }

            sleep(Duration::from_millis(self.config.page_delay_ms)).await;
            let Some(next) = self.next_page(&result).await? else {
                break;
            };
            result = next;
        }

        self.scan_state_repo.mark_success(SCAN_NAME).await?;
        Ok(())
    }

    async fn load_resume_page(&self) -> Result<u32, WorkerError> {
        let Some(state) = self.scan_state_repo.get_by_name(SCAN_NAME).await? else {
            return Ok(0);
        };

        let Some(cursor) = state.cursor else {
            return Ok(0);
        };

        if let Some(value) = cursor.strip_prefix("page:") {
            return Ok(value.parse::<u32>().unwrap_or(0));
        }

        Ok(0)
    }

    fn collect_page(&self, result: &BeatmapsetSearchResult) -> HashMap<u32, MapperAccumulator> {
        let mut accumulators: HashMap<u32, MapperAccumulator> = HashMap::new();

        for mapset in &result.mapsets {
            let creator_id = mapset.creator_id;
            let entry = accumulators.entry(creator_id).or_default();
            entry.add_status(mapset.status);
        }

        accumulators
    }

    async fn enrich_page(
        &self,
        accumulators: HashMap<u32, MapperAccumulator>,
    ) -> Result<Vec<MapperStats>, WorkerError> {
        if accumulators.is_empty() {
            return Ok(Vec::new());
        }

        let mut ids: Vec<u32> = accumulators.keys().copied().collect();
        ids.sort_unstable();

        let mut page_stats = Vec::new();

        for chunk in ids.chunks(self.config.batch_size) {
            let users = self.retry(|| self.osu.users(chunk.iter().copied())).await?;

            for user in users {
                if user.country_code != "UA" {
                    continue;
                }

                let Some(acc) = accumulators.get(&user.user_id) else {
                    continue;
                };

                page_stats.push(MapperStats {
                    osu_user_id: user.user_id as i64,
                    username: user.username.to_string(),
                    country_code: user.country_code.to_string(),
                    count_graveyard: acc.count_graveyard,
                    count_pending: acc.count_pending,
                    count_wip: acc.count_wip,
                    count_loved: acc.count_loved,
                    count_ranked: acc.count_ranked,
                    count_approved: acc.count_approved,
                    count_total: acc.count_total,
                    is_bn: false,
                    nominated_count: None,
                });
            }
        }

        Ok(page_stats)
    }

    async fn persist_page(
        &self,
        stats: Vec<MapperStats>,
        page_index: u32,
    ) -> Result<(), WorkerError> {
        let txn = self.mappers_repo.db().begin().await?;

        for stat in stats {
            self.mappers_repo.increment_with(&txn, &stat).await?;
        }

        self.scan_state_repo
            .upsert_cursor_with(&txn, SCAN_NAME, Some(format!("page:{}", page_index)))
            .await?;

        txn.commit().await?;
        Ok(())
    }

    async fn retry<T, Fut, F>(&self, mut action: F) -> Result<T, WorkerError>
    where
        F: FnMut() -> Fut,
        Fut: IntoFuture<Output = Result<T, rosu_v2::error::OsuError>>,
    {
        let mut attempt = 0u32;
        let mut delay = Duration::from_millis(500);

        loop {
            match action().into_future().await {
                Ok(result) => return Ok(result),
                Err(err) => {
                    attempt += 1;
                    if attempt >= 5 {
                        return Err(err.into());
                    }
                    tracing::warn!(attempt, error = ?err, "osu api request failed, retrying");
                    sleep(delay).await;
                    delay = delay.saturating_mul(2);
                }
            }
        }
    }

    async fn next_page(
        &self,
        current: &BeatmapsetSearchResult,
    ) -> Result<Option<BeatmapsetSearchResult>, WorkerError> {
        let mut attempt = 0u32;
        let mut delay = Duration::from_millis(500);

        loop {
            match current.get_next(&self.osu).await {
                None => return Ok(None),
                Some(Ok(next)) => return Ok(Some(next)),
                Some(Err(err)) => {
                    attempt += 1;
                    if attempt >= 5 {
                        return Err(err.into());
                    }
                    tracing::warn!(attempt, error = ?err, "osu api next page failed, retrying");
                    sleep(delay).await;
                    delay = delay.saturating_mul(2);
                }
            }
        }
    }
}
