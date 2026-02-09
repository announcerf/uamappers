use std::time::Duration;

use tokio::sync::Mutex;

const OSU_MIN_REQUEST_INTERVAL_MS: u64 = 1200;

#[derive(Debug, Copy, Clone)]
pub struct OsuThrottleSnapshot {
    pub acquires: u64,
    pub total_sleep_ms: u64,
}

#[derive(Debug)]
struct OsuThrottleStats {
    acquires: u64,
    total_sleep_ms: u64,
}

#[derive(Debug)]
pub struct OsuThrottle {
    next_allowed_at: Mutex<tokio::time::Instant>,
    stats: Mutex<OsuThrottleStats>,
}

impl OsuThrottle {
    pub fn new() -> Self {
        Self {
            next_allowed_at: Mutex::new(tokio::time::Instant::now()),
            stats: Mutex::new(OsuThrottleStats {
                acquires: 0,
                total_sleep_ms: 0,
            }),
        }
    }

    pub async fn acquire(&self) {
        let mut next_allowed_at = self.next_allowed_at.lock().await;
        let now = tokio::time::Instant::now();

        if *next_allowed_at > now {
            let sleep_for = *next_allowed_at - now;
            tokio::time::sleep(sleep_for).await;

            let mut stats = self.stats.lock().await;
            stats.total_sleep_ms = stats
                .total_sleep_ms
                .saturating_add(sleep_for.as_millis().try_into().unwrap_or(u64::MAX));
        }

        *next_allowed_at =
            tokio::time::Instant::now() + Duration::from_millis(OSU_MIN_REQUEST_INTERVAL_MS);

        let mut stats = self.stats.lock().await;
        stats.acquires = stats.acquires.saturating_add(1);
    }

    pub const fn min_interval_ms(&self) -> u64 {
        OSU_MIN_REQUEST_INTERVAL_MS
    }

    pub async fn snapshot(&self) -> OsuThrottleSnapshot {
        let stats = self.stats.lock().await;
        OsuThrottleSnapshot {
            acquires: stats.acquires,
            total_sleep_ms: stats.total_sleep_ms,
        }
    }
}

impl Default for OsuThrottle {
    fn default() -> Self {
        Self::new()
    }
}
