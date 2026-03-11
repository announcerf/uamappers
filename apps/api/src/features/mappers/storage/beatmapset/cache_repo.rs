use chrono::Utc;
use sea_orm::sea_query::OnConflict;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, Set};

use crate::entities::beatmapset;

#[derive(Clone, Debug)]
pub struct NewBeatmapsetRow {
    pub osu_beatmapset_id: i64,
    pub last_updated: chrono::DateTime<Utc>,
    pub raw: sea_orm::JsonValue,
}

#[derive(Clone, Debug)]
pub struct BeatmapsetRepo {
    db: DatabaseConnection,
}

impl BeatmapsetRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    pub async fn upsert_many_with<C: ConnectionTrait>(
        &self,
        db: &C,
        rows: Vec<beatmapset::ActiveModel>,
    ) -> Result<(), DbErr> {
        if rows.is_empty() {
            return Ok(());
        }

        for row in rows {
            beatmapset::Entity::insert(row)
                .on_conflict(
                    OnConflict::column(beatmapset::Column::OsuBeatmapsetId)
                        .update_columns([
                            beatmapset::Column::LastUpdated,
                            beatmapset::Column::Raw,
                            beatmapset::Column::UpdatedAt,
                        ])
                        .to_owned(),
                )
                .exec(db)
                .await?;
        }

        Ok(())
    }

    pub fn to_active(&self, row: NewBeatmapsetRow) -> beatmapset::ActiveModel {
        let now = Utc::now();
        beatmapset::ActiveModel {
            osu_beatmapset_id: Set(row.osu_beatmapset_id),
            last_updated: Set(row.last_updated),
            raw: Set(row.raw),
            created_at: Set(now),
            updated_at: Set(now),
        }
    }
}
