use super::types::MapperEnrich;
use crate::shared::errors::WorkerError;

impl MapperEnrich {
    pub async fn run(&self) -> Result<(), WorkerError> {
        if self.config.enrich_users {
            self.run_users().await?;
        }
        if self.config.enrich_beatmapsets {
            self.run_beatmapsets().await?;
        }
        Ok(())
    }

    pub(super) async fn resolve_current_user_id(
        &self,
        requested: i64,
    ) -> Result<Option<i64>, WorkerError> {
        if requested <= 0 {
            let rows = self.ua_mappers_repo.list_after_id(0, 1).await?;
            return Ok(rows.first().map(|r| r.osu_user_id));
        }

        if self
            .ua_mappers_repo
            .get_by_osu_user_id(requested)
            .await?
            .is_some()
        {
            return Ok(Some(requested));
        }

        let rows = self.ua_mappers_repo.list_after_id(requested, 1).await?;
        Ok(rows.first().map(|r| r.osu_user_id))
    }
}
