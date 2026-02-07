use std::future::IntoFuture;
use std::time::Duration;

use rosu_v2::model::user::{User, UserExtended};
use rosu_v2::prelude::{BeatmapsetSearchResult, Osu};
use tokio::time::sleep;

use crate::shared::errors::WorkerError;

use super::throttle::OsuThrottle;

pub struct OsuClient {
    osu: Osu,
    throttle: OsuThrottle,
}

impl OsuClient {
    pub fn new(osu: Osu) -> Self {
        Self {
            osu,
            throttle: OsuThrottle::new(),
        }
    }

    pub async fn beatmapset_search_start(&self) -> Result<BeatmapsetSearchResult, WorkerError> {
        self.retry(|| self.osu.beatmapset_search().query("status=any").nsfw(true))
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
