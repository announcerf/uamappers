mod leaderboards;
mod stats;
mod stats_helpers;

pub use leaderboards::LEADERBOARD_KEYS;
pub use stats::build_mapper_stats_row;

use sea_orm::TransactionTrait;
use uamappers_api::features::mappers::storage::osu_user_fingerprint::MapperFingerprint;

use crate::shared::errors::WorkerError;

use super::snapshot::{mapper_stats_row_to_snapshot_row, snapshot_week};
use super::types::MapperEnrich;

impl MapperEnrich {
    pub(crate) async fn refresh_mapper_stats(&self, osu_user_id: i64) -> Result<(), WorkerError> {
        let mapper_user = self
            .osu_users_repo
            .get_by_osu_user_id(osu_user_id)
            .await?;
        let mapper_fingerprint = mapper_user
            .as_ref()
            .and_then(|row| MapperFingerprint::from_raw(&row.raw));
        let relations = self
            .osu_user_beatmapsets_repo
            .list_by_osu_user_id(osu_user_id)
            .await?;

        let relevant_ids = stats_helpers::relevant_mapset_ids(&relations);
        let relevant_ids = relevant_ids.into_iter().collect::<Vec<_>>();
        let beatmapsets = self
            .beatmapset_profiles_repo
            .list_by_osu_beatmapset_ids(&relevant_ids)
            .await?;
        let beatmaps = self
            .beatmap_profiles_repo
            .list_by_osu_beatmapset_ids(&relevant_ids)
            .await?;

        let row = build_mapper_stats_row(
            osu_user_id,
            mapper_fingerprint.as_ref(),
            &relations,
            &beatmapsets,
            &beatmaps,
        );
        let snapshot = mapper_stats_row_to_snapshot_row(&row, snapshot_week(chrono::Utc::now()));
        let txn = self.mapper_stats_repo.db().begin().await?;
        self.mapper_stats_repo.upsert_with(&txn, row).await?;
        self.mapper_aggregate_snapshots_repo
            .upsert_with(&txn, snapshot)
            .await?;
        txn.commit().await?;

        Ok(())
    }

    pub(crate) async fn refresh_leaderboard_positions(&self) -> Result<(), WorkerError> {
        let txn = self.leaderboard_positions_repo.db().begin().await?;
        let measured_at = chrono::Utc::now();

        for leaderboard_key in LEADERBOARD_KEYS {
            let previous = self
                .leaderboard_positions_repo
                .list_rank_map(leaderboard_key)
                .await?;
            let stats = self
                .mapper_stats_repo
                .list_for_leaderboard(leaderboard_key)
                .await?;
            let rows = leaderboards::build_leaderboard_rows(
                leaderboard_key,
                measured_at,
                &previous,
                &stats,
            );

            self.leaderboard_positions_repo
                .replace_for_leaderboard_with(&txn, leaderboard_key, rows)
                .await?;
        }

        txn.commit().await?;
        Ok(())
    }
}
