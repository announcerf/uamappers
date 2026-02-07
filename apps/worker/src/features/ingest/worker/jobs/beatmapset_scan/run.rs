use crate::shared::errors::WorkerError;

use super::{Scanner, kudosu::KudosuCache, types::SCAN_NAME};

impl Scanner {
    pub async fn run(&self) -> Result<(), WorkerError> {
        match self.run_inner().await {
            Ok(()) => Ok(()),
            Err(err) => {
                if let Err(mark_err) = self.record_failure().await {
                    tracing::error!(error = ?mark_err, "failed to persist scan failure state");
                }
                Err(err)
            }
        }
    }

    async fn run_inner(&self) -> Result<(), WorkerError> {
        let state = self.scan_state_repo.get_by_name(SCAN_NAME).await?;
        if let Some(state) = &state
            && state.cursor.is_none()
            && state.last_success_at.is_some()
            && !self.config.force_rescan
        {
            tracing::info!(
                job = SCAN_NAME,
                last_success_at = ?state.last_success_at,
                "scan already completed; skipping (set SCAN_FORCE_RESCAN=1 to run again)"
            );
            return Ok(());
        }

        let resume_page = self.load_resume_page().await?;
        let mut kudosu_cache = KudosuCache::default();

        let mut result = self.osu_client.beatmapset_search_start().await?;

        let mut skipped: u32 = 0;
        while skipped < resume_page {
            if !result.has_more() {
                break;
            }
            let Some(next) = self.osu_client.beatmapset_search_next(&result).await? else {
                break;
            };
            result = next;
            skipped += 1;
        }

        let mut page_index: u32 = resume_page;
        let mut processed_this_run: u32 = 0;

        loop {
            let collected = Self::collect_page(&result);
            let page = self.enrich_page(&mut kudosu_cache, collected).await?;

            let next_cursor = if result.has_more() {
                Some(format!("page:{}", page_index + 1))
            } else {
                None
            };

            self.persist_page(page.stats, page.beatmapsets, next_cursor, page_index)
                .await?;

            if !result.has_more() {
                break;
            }

            processed_this_run += 1;
            if let Some(max_pages) = self.config.max_pages
                && processed_this_run >= max_pages
            {
                break;
            }

            tokio::time::sleep(std::time::Duration::from_millis(self.config.page_delay_ms)).await;
            let Some(next) = self.osu_client.beatmapset_search_next(&result).await? else {
                break;
            };
            result = next;
            page_index += 1;
        }

        self.scan_state_repo.mark_success(SCAN_NAME).await?;
        Ok(())
    }
}
