use std::time::Duration;

use tokio::sync::Mutex;

const OSU_MIN_REQUEST_INTERVAL_MS: u64 = 1200;

#[derive(Debug)]
pub struct OsuThrottle {
    next_allowed_at: Mutex<tokio::time::Instant>,
}

impl OsuThrottle {
    pub fn new() -> Self {
        Self {
            next_allowed_at: Mutex::new(tokio::time::Instant::now()),
        }
    }

    pub async fn acquire(&self) {
        let mut next_allowed_at = self.next_allowed_at.lock().await;
        let now = tokio::time::Instant::now();

        if *next_allowed_at > now {
            tokio::time::sleep(*next_allowed_at - now).await;
        }

        *next_allowed_at =
            tokio::time::Instant::now() + Duration::from_millis(OSU_MIN_REQUEST_INTERVAL_MS);
    }
}
