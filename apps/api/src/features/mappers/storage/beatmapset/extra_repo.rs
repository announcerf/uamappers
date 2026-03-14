use chrono::Utc;
use sea_orm::sea_query::OnConflict;
use sea_orm::{ColumnTrait, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set};

use crate::entities::beatmapset_extra;

#[derive(Clone, Debug)]
pub struct NewBeatmapsetExtraRow {
    pub osu_beatmapset_id: i64,
    pub creator_id: i64,
    pub creator_name: String,
    pub anime_cover: Option<String>,
}

#[derive(Clone, Debug)]
pub struct BeatmapsetExtraRepo {
    db: DatabaseConnection,
}

impl BeatmapsetExtraRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    pub async fn get_by_osu_beatmapset_id(
        &self,
        osu_beatmapset_id: i64,
    ) -> Result<Option<beatmapset_extra::Model>, DbErr> {
        beatmapset_extra::Entity::find_by_id(osu_beatmapset_id)
            .one(&self.db)
            .await
    }

    pub async fn list_by_osu_beatmapset_ids(
        &self,
        ids: &[i64],
    ) -> Result<Vec<beatmapset_extra::Model>, DbErr> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        beatmapset_extra::Entity::find()
            .filter(beatmapset_extra::Column::OsuBeatmapsetId.is_in(ids.to_vec()))
            .all(&self.db)
            .await
    }

    pub async fn upsert_many_with<C: ConnectionTrait>(
        &self,
        db: &C,
        rows: Vec<NewBeatmapsetExtraRow>,
    ) -> Result<(), DbErr> {
        if rows.is_empty() {
            return Ok(());
        }

        for row in rows {
            beatmapset_extra::Entity::insert(self.to_active(row))
                .on_conflict(
                    OnConflict::column(beatmapset_extra::Column::OsuBeatmapsetId)
                        .update_columns([
                            beatmapset_extra::Column::CreatorId,
                            beatmapset_extra::Column::CreatorName,
                            beatmapset_extra::Column::AnimeCover,
                            beatmapset_extra::Column::UpdatedAt,
                        ])
                        .to_owned(),
                )
                .exec(db)
                .await?;
        }

        Ok(())
    }

    fn to_active(&self, row: NewBeatmapsetExtraRow) -> beatmapset_extra::ActiveModel {
        let now = Utc::now();

        beatmapset_extra::ActiveModel {
            osu_beatmapset_id: Set(row.osu_beatmapset_id),
            creator_id: Set(row.creator_id),
            creator_name: Set(row.creator_name),
            anime_cover: Set(row.anime_cover),
            updated_at: Set(now),
        }
    }
}
