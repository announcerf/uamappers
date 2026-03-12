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
}
