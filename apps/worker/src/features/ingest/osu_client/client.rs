use std::future::IntoFuture;
use std::sync::Arc;
use std::time::Duration;

use rosu_v2::model::beatmap::BeatmapsetExtended;
use rosu_v2::model::user::{User, UserBeatmapsetsKind, UserExtended};
use rosu_v2::prelude::{BeatmapsetSearchResult, BeatmapsetSearchSort, Osu};
use serde_json::json;
use tokio::sync::Mutex;
use tokio::time::sleep;

use crate::shared::errors::WorkerError;

use super::throttle::OsuThrottle;
use super::throttle::OsuThrottleSnapshot;

#[derive(Debug, Default)]
struct OsuClientStats {
    retries: u64,
}

#[derive(Debug, Copy, Clone)]
pub struct OsuClientStatsSnapshot {
    pub retries: u64,
}

#[derive(Clone)]
pub struct OsuClient {
    osu: Arc<Osu>,
    throttle: Arc<OsuThrottle>,
    stats: Arc<Mutex<OsuClientStats>>,
}

impl OsuClient {
    pub fn new(osu: Osu) -> Self {
        Self {
            osu: Arc::new(osu),
            throttle: Arc::new(OsuThrottle::new()),
            stats: Arc::new(Mutex::new(OsuClientStats::default())),
        }
    }

    pub fn min_request_interval_ms(&self) -> u64 {
        self.throttle.min_interval_ms()
    }

    pub async fn throttle_snapshot(&self) -> OsuThrottleSnapshot {
        self.throttle.snapshot().await
    }

    pub async fn stats_snapshot(&self) -> OsuClientStatsSnapshot {
        let stats = self.stats.lock().await;
        OsuClientStatsSnapshot {
            retries: stats.retries,
        }
    }

    pub async fn beatmapset_search_start(
        &self,
        descending: bool,
    ) -> Result<BeatmapsetSearchResult, WorkerError> {
        self.retry(|| {
            self.osu
                .beatmapset_search()
                .status(None)
                .nsfw(true)
                .sort(BeatmapsetSearchSort::LastUpdate, descending)
        })
        .await
    }

    pub async fn beatmapset_search_page(
        &self,
        page_index: u32,
        descending: bool,
    ) -> Result<BeatmapsetSearchResult, WorkerError> {
        let page = page_index.saturating_add(1);
        self.retry(|| {
            self.osu
                .beatmapset_search()
                .status(None)
                .nsfw(true)
                .sort(BeatmapsetSearchSort::LastUpdate, descending)
                .page(page)
        })
        .await
    }

    pub async fn beatmapset_search_from_cursor_string(
        &self,
        cursor_string: &str,
        descending: bool,
    ) -> Result<BeatmapsetSearchResult, WorkerError> {
        let seed = json!({
            "beatmapsets": [],
            "cursor_string": cursor_string,
            "search": {
                "status": "any",
                "video": false,
                "storyboard": false,
                "recommended": false,
                "converts": false,
                "follows": false,
                "spotlights": false,
                "featured_artists": false,
                "nsfw": true,
                "_sort": "updated",
                "descending": descending
            },
            "total": 0
        });

        let seed_bytes = serde_json::to_vec(&seed)
            .map_err(|err| WorkerError::Config(format!("invalid beatmapset search seed: {err}")))?;

        let seed: BeatmapsetSearchResult = serde_json::from_slice(&seed_bytes)
            .map_err(|err| WorkerError::Config(format!("invalid beatmapset search seed: {err}")))?;

        let mut attempt = 0u32;
        let mut delay = Duration::from_millis(500);

        loop {
            self.throttle.acquire().await;
            match seed.get_next(&self.osu).await {
                None => {
                    return Err(WorkerError::Config(
                        "beatmapset search cursor has no next page".to_string(),
                    ));
                }
                Some(Ok(next)) => return Ok(next),
                Some(Err(err)) => {
                    attempt += 1;
                    if attempt >= 5 {
                        return Err(err.into());
                    }
                    self.inc_retry().await;
                    tracing::warn!("osu retry a{} {}", attempt, err);
                    sleep(delay).await;
                    delay = delay.saturating_mul(2);
                }
            }
        }
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
                    self.inc_retry().await;
                    tracing::warn!("osu retry a{} {}", attempt, err);
                    sleep(delay).await;
                    delay = delay.saturating_mul(2);
                }
            }
        }
    }

    pub async fn beatmapset_search_resume(
        &self,
        page_index: u32,
        descending: bool,
    ) -> Result<BeatmapsetSearchResult, WorkerError> {
        match page_index {
            0 => self.beatmapset_search_start(descending).await,
            n => self.beatmapset_search_page(n, descending).await,
        }
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
                    self.inc_retry().await;
                    tracing::warn!("osu retry a{} {}", attempt, err);
                    sleep(delay).await;
                    delay = delay.saturating_mul(2);
                }
            }
        }
    }

    async fn inc_retry(&self) {
        let mut stats = self.stats.lock().await;
        stats.retries = stats.retries.saturating_add(1);
    }
}
