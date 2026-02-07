use rosu_v2::model::user::UserExtended;

use uamappers_api::features::mappers::domain::model::MapperStats;
use uamappers_api::features::mappers::storage::beatmapset_repo::NewBeatmapsetRow;

use crate::shared::errors::WorkerError;

use super::{
    accumulator::CollectedPage, beatmapset::mapset_to_row, kudosu::KudosuCache,
    types::EnrichedPage, Scanner,
};

impl Scanner {
    pub(crate) async fn enrich_page(
        &self,
        kudosu_cache: &mut KudosuCache,
        collected: CollectedPage,
    ) -> Result<EnrichedPage, WorkerError> {
        if collected.accumulators.is_empty() {
            return Ok(EnrichedPage::default());
        }

        let mut ids: Vec<u32> = collected.accumulators.keys().copied().collect();
        ids.sort_unstable();

        let mut stats = Vec::new();
        let mut beatmapsets = Vec::new();

        for chunk in ids.chunks(self.config.batch_size) {
            let user_ids: Vec<u32> = chunk.to_vec();
            let users = self.osu_client.users(user_ids).await?;

            for user in users {
                if user.country_code != "UA" {
                    continue;
                }

                let Some(acc) = collected.accumulators.get(&user.user_id) else {
                    continue;
                };

                let is_bn = user.is_full_bn.unwrap_or(false) || user.is_limited_bn.unwrap_or(false);
                let (kudosu_available, kudosu_total) =
                    self.load_kudosu(kudosu_cache, &user).await?;

                stats.push(MapperStats {
                    osu_user_id: user.user_id as i64,
                    username: user.username.to_string(),
                    country_code: user.country_code.to_string(),
                    kudosu_available,
                    kudosu_total,
                    count_graveyard: acc.count_graveyard,
                    count_pending: acc.count_pending,
                    count_wip: acc.count_wip,
                    count_loved: acc.count_loved,
                    count_ranked: acc.count_ranked,
                    count_approved: acc.count_approved,
                    count_total: acc.count_total,
                    is_bn,
                    nominated_count: None,
                });

                if let Some(mapsets) = collected.mapsets_by_creator.get(&user.user_id) {
                    for mapset in mapsets {
                        let row: NewBeatmapsetRow = mapset_to_row(mapset);
                        let active = self.beatmapsets_repo.to_active(row);
                        beatmapsets.push(active);
                    }
                }
            }
        }

        Ok(EnrichedPage { stats, beatmapsets })
    }

    async fn load_kudosu(
        &self,
        cache: &mut KudosuCache,
        user: &rosu_v2::model::user::User,
    ) -> Result<(Option<i32>, Option<i32>), WorkerError> {
        if let Some((available, total)) = cache.get(user.user_id) {
            return Ok((Some(available), Some(total)));
        }

        tokio::time::sleep(std::time::Duration::from_millis(self.config.user_delay_ms)).await;
        let extended: UserExtended = self.osu_client.user(user.user_id).await?;

        let available = extended.kudosu.available;
        let total = extended.kudosu.total;

        cache.insert(user.user_id, available, total);
        Ok((Some(available), Some(total)))
    }
}
