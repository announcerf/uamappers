use chrono::Utc;
use sea_orm::sea_query::OnConflict;
use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, Set,
};

use crate::entities::{beatmapset, osu_user_beatmapset};

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
        for osu_beatmapset_id in beatmapset_ids {
            let active = osu_user_beatmapset::ActiveModel {
                osu_user_id: Set(osu_user_id),
                kind: Set(kind.to_string()),
                osu_beatmapset_id: Set(*osu_beatmapset_id),
                created_at: Set(now),
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

    pub async fn list_beatmapsets(
        &self,
        osu_user_id: i64,
        kind: &str,
        limit: u64,
        offset: u64,
    ) -> Result<(Vec<beatmapset::Model>, u64), DbErr> {
        let total = osu_user_beatmapset::Entity::find()
            .filter(osu_user_beatmapset::Column::OsuUserId.eq(osu_user_id))
            .filter(osu_user_beatmapset::Column::Kind.eq(kind))
            .count(&self.db)
            .await?;

        let rows = osu_user_beatmapset::Entity::find()
            .filter(osu_user_beatmapset::Column::OsuUserId.eq(osu_user_id))
            .filter(osu_user_beatmapset::Column::Kind.eq(kind))
            .find_also_related(beatmapset::Entity)
            .order_by_desc(beatmapset::Column::LastUpdated)
            .order_by_desc(beatmapset::Column::OsuBeatmapsetId)
            .limit(limit)
            .offset(offset)
            .all(&self.db)
            .await?;

        let beatmapsets = rows
            .into_iter()
            .filter_map(|(_, maybe)| maybe)
            .collect::<Vec<_>>();

        Ok((beatmapsets, total))
    }
}
