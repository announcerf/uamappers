use chrono::Utc;
use sea_orm::sea_query::OnConflict;
use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, Set,
};

use crate::entities::osu_user_beatmapset;
use crate::features::mappers::storage::codes::kind_code;

#[derive(Clone, Debug)]
pub struct OsuUserBeatmapsetRepo {
    db: DatabaseConnection,
}

impl OsuUserBeatmapsetRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    pub async fn upsert_many_with<C: ConnectionTrait>(
        &self,
        db: &C,
        osu_user_id: i64,
        kind: &str,
        beatmapset_ids: &[i64],
    ) -> Result<(), DbErr> {
        if beatmapset_ids.is_empty() {
            return Ok(());
        }

        let now = Utc::now();
        let kind = kind_code(kind);

        for osu_beatmapset_id in beatmapset_ids {
            let active = osu_user_beatmapset::ActiveModel {
                osu_user_id: Set(osu_user_id),
                kind: Set(kind),
                osu_beatmapset_id: Set(*osu_beatmapset_id),
                updated_at: Set(now),
            };

            osu_user_beatmapset::Entity::insert(active)
                .on_conflict(
                    OnConflict::columns([
                        osu_user_beatmapset::Column::OsuUserId,
                        osu_user_beatmapset::Column::Kind,
                        osu_user_beatmapset::Column::OsuBeatmapsetId,
                    ])
                    .update_columns([osu_user_beatmapset::Column::UpdatedAt])
                    .to_owned(),
                )
                .exec(db)
                .await?;
        }

        Ok(())
    }

    pub async fn list_beatmapset_ids(
        &self,
        osu_user_id: i64,
        kind: &str,
        limit: u64,
        offset: u64,
    ) -> Result<(Vec<i64>, u64), DbErr> {
        let kind = kind_code(kind);
        let total = osu_user_beatmapset::Entity::find()
            .filter(osu_user_beatmapset::Column::OsuUserId.eq(osu_user_id))
            .filter(osu_user_beatmapset::Column::Kind.eq(kind))
            .count(&self.db)
            .await?;

        let ids = osu_user_beatmapset::Entity::find()
            .filter(osu_user_beatmapset::Column::OsuUserId.eq(osu_user_id))
            .filter(osu_user_beatmapset::Column::Kind.eq(kind))
            .order_by_desc(osu_user_beatmapset::Column::UpdatedAt)
            .order_by_desc(osu_user_beatmapset::Column::OsuBeatmapsetId)
            .limit(limit)
            .offset(offset)
            .all(&self.db)
            .await?;

        let ids = ids
            .into_iter()
            .map(|row| row.osu_beatmapset_id)
            .collect::<Vec<_>>();

        Ok((ids, total))
    }

    pub async fn list_by_osu_user_id(
        &self,
        osu_user_id: i64,
    ) -> Result<Vec<osu_user_beatmapset::Model>, DbErr> {
        osu_user_beatmapset::Entity::find()
            .filter(osu_user_beatmapset::Column::OsuUserId.eq(osu_user_id))
            .all(&self.db)
            .await
    }
}
