use crate::shared::errors::WorkerError;
use uamappers_api::entities::ua_mapper;

use super::super::projection::user_to_mapper_profile_row;
use super::super::raw::strip_top_level_id;
use super::super::storage::persist_user_profile;
use super::super::types::MapperEnrich;

impl MapperEnrich {
    pub(in super::super) async fn persist_mapper_user(
        &self,
        mapper: &ua_mapper::Model,
    ) -> Result<(), WorkerError> {
        let extended = self.osu_client.user(mapper.osu_user_id as u32).await?;
        let raw = serde_json::to_value(&extended)
            .map(strip_top_level_id)
            .unwrap_or(serde_json::Value::Null);
        let fetched_at = chrono::Utc::now();
        let profile = user_to_mapper_profile_row(&extended, fetched_at);

        persist_user_profile(self, mapper.osu_user_id, raw, profile, fetched_at).await?;

        tracing::debug!(
            job = "mapper_enrich_users",
            osu_user_id = mapper.osu_user_id,
            "persisted user profile"
        );
        Ok(())
    }
}
