use std::future::IntoFuture;
use std::sync::Arc;
use std::time::Duration;

use rosu_v2::model::beatmap::BeatmapsetExtended;
use rosu_v2::model::user::{User, UserBeatmapsetsKind, UserExtended};
use rosu_v2::prelude::{BeatmapsetSearchResult, BeatmapsetSearchSort, Osu};
use tokio::time::sleep;

use crate::shared::errors::WorkerError;

use super::throttle::OsuThrottle;
use super::throttle::OsuThrottleSnapshot;

#[derive(Clone)]
pub struct OsuClient {
    osu: Arc<Osu>,
    throttle: Arc<OsuThrottle>,
}

impl OsuClient {
    pub fn new(osu: Osu) -> Self {
        Self {
            osu: Arc::new(osu),
            throttle: Arc::new(OsuThrottle::new()),
        }
    }

    pub fn min_request_interval_ms(&self) -> u64 {
        self.throttle.min_interval_ms()
    }

    pub async fn throttle_snapshot(&self) -> OsuThrottleSnapshot {
        self.throttle.snapshot().await
    }

    pub async fn beatmapset_search_start(&self) -> Result<BeatmapsetSearchResult, WorkerError> {
        self.retry(|| {
            self.osu
                .beatmapset_search()
                .query("status=any")
                .nsfw(true)
                .sort(BeatmapsetSearchSort::LastUpdate, true)
        })
        .await
    }

    pub async fn beatmapset_search_next(
        &self,
        current: &BeatmapsetSearchResult,
    ) -> Result<Option<BeatmapsetSearchResult>, WorkerError> {
        let mut attempt = 0u32;
        let mut delay = Duration::from_millis(500);

        loop {
            self.throttle.acquire().await;
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

    pub async fn beatmapset_search_resume(
        &self,
        page_index: u32,
    ) -> Result<BeatmapsetSearchResult, WorkerError> {
        // We store resume cursors as `page:N`. For incremental runs N is typically small,
        // so replaying `get_next` N times is acceptable and keeps us independent of rosu-v2 internals.
        let mut result = self.beatmapset_search_start().await?;

        let mut skipped: u32 = 0;
        while skipped < page_index {
            if !result.has_more() {
                break;
            }
            let Some(next) = self.beatmapset_search_next(&result).await? else {
                break;
            };
            result = next;
            skipped += 1;
        }

        Ok(result)
    }

    pub async fn users<I>(&self, user_ids: I) -> Result<Vec<User>, WorkerError>
    where
        I: IntoIterator<Item = u32> + Clone,
    {
        self.retry(|| self.osu.users(user_ids.clone().into_iter()))
            .await
    }

    pub async fn user(&self, user_id: u32) -> Result<UserExtended, WorkerError> {
        self.retry(|| self.osu.user(user_id)).await
    }

    pub async fn user_beatmapsets(
        &self,
        user_id: u32,
        kind: UserBeatmapsetsKind,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<BeatmapsetExtended>, WorkerError> {
        self.retry(|| {
            self.osu
                .user_beatmapsets(user_id, kind)
                .limit(limit)
                .offset(offset)
        })
        .await
    }

    async fn retry<T, Fut, F>(&self, mut action: F) -> Result<T, WorkerError>
    where
        F: FnMut() -> Fut,
        Fut: IntoFuture<Output = Result<T, rosu_v2::error::OsuError>>,
    {
        let mut attempt = 0u32;
        let mut delay = Duration::from_millis(500);

        loop {
            self.throttle.acquire().await;
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
}
