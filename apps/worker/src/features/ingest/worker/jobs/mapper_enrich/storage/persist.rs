use sea_orm::TransactionTrait;

use crate::shared::errors::WorkerError;

use super::super::projection::BeatmapsetsPersistPage;
use super::super::types::MapperEnrich;

pub async fn persist_user_profile(
    job: &MapperEnrich,
    osu_user_id: i64,
    raw: sea_orm::JsonValue,
    fetched_at: chrono::DateTime<chrono::Utc>,
) -> Result<(), WorkerError> {
    let txn = job.osu_users_repo.db().begin().await?;

    job.osu_users_repo
        .upsert_with(&txn, osu_user_id, raw, fetched_at)
        .await?;

    txn.commit().await?;
    Ok(())
}

pub async fn persist_beatmapsets_page(
    job: &MapperEnrich,
    page: BeatmapsetsPersistPage,
    osu_user_id: i64,
    kind: &str,
) -> Result<(), WorkerError> {
    let txn = job.beatmapset_extras_repo.db().begin().await?;

    job.beatmapset_extras_repo
        .upsert_many_with(&txn, page.beatmapset_extras)
        .await?;
    job.beatmapset_profiles_repo
        .upsert_many_with(&txn, page.beatmapset_profiles)
        .await?;
    job.beatmapset_snapshots_repo
        .upsert_many_with(&txn, page.beatmapset_snapshots)
        .await?;
    job.beatmap_profiles_repo
        .upsert_many_with(&txn, page.beatmap_profiles)
        .await?;

    for (osu_beatmapset_id, keep_ids) in page.beatmap_ids_by_mapset {
        job.beatmap_profiles_repo
            .delete_missing_for_mapset_with(&txn, osu_beatmapset_id, &keep_ids)
            .await?;
    }

    job.osu_user_beatmapsets_repo
        .upsert_many_with(&txn, osu_user_id, kind, &page.beatmapset_ids)
        .await?;

    txn.commit().await?;
    Ok(())
}
