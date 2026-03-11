use chrono::Utc;
use sea_orm::sea_query::OnConflict;
use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder,
    Set,
};

use crate::entities::beatmapset_snapshot_weekly;

#[derive(Clone, Debug)]
pub struct NewBeatmapsetSnapshotWeeklyRow {
    pub osu_beatmapset_id: i64,
    pub snapshot_week: chrono::DateTime<Utc>,
    pub status: String,
    pub playcount: i64,
    pub favourite_count: i64,
    pub rating: f32,
    pub avg_passcount: f32,
    pub avg_pass_rate: f32,
    pub min_pass_rate: f32,
    pub max_pass_rate: f32,
    pub last_updated: chrono::DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct BeatmapsetSnapshotWeeklyRepo {
    db: DatabaseConnection,
}

impl BeatmapsetSnapshotWeeklyRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    pub async fn list_by_osu_beatmapset_id(
        &self,
        osu_beatmapset_id: i64,
    ) -> Result<Vec<beatmapset_snapshot_weekly::Model>, DbErr> {
        beatmapset_snapshot_weekly::Entity::find()
            .filter(beatmapset_snapshot_weekly::Column::OsuBeatmapsetId.eq(osu_beatmapset_id))
            .order_by_asc(beatmapset_snapshot_weekly::Column::SnapshotWeek)
            .all(&self.db)
            .await
    }

    pub async fn upsert_many_with<C: ConnectionTrait>(
        &self,
        db: &C,
        rows: Vec<NewBeatmapsetSnapshotWeeklyRow>,
    ) -> Result<(), DbErr> {
        if rows.is_empty() {
            return Ok(());
        }

        for row in rows {
            beatmapset_snapshot_weekly::Entity::insert(self.to_active(row))
                .on_conflict(
                    OnConflict::columns([
                        beatmapset_snapshot_weekly::Column::OsuBeatmapsetId,
                        beatmapset_snapshot_weekly::Column::SnapshotWeek,
                    ])
                    .update_columns([
                        beatmapset_snapshot_weekly::Column::Status,
                        beatmapset_snapshot_weekly::Column::Playcount,
                        beatmapset_snapshot_weekly::Column::FavouriteCount,
                        beatmapset_snapshot_weekly::Column::Rating,
                        beatmapset_snapshot_weekly::Column::AvgPasscount,
                        beatmapset_snapshot_weekly::Column::AvgPassRate,
                        beatmapset_snapshot_weekly::Column::MinPassRate,
                        beatmapset_snapshot_weekly::Column::MaxPassRate,
                        beatmapset_snapshot_weekly::Column::LastUpdated,
                        beatmapset_snapshot_weekly::Column::UpdatedAt,
                    ])
                    .to_owned(),
                )
                .exec(db)
                .await?;
        }

        Ok(())
    }

    fn to_active(
        &self,
        row: NewBeatmapsetSnapshotWeeklyRow,
    ) -> beatmapset_snapshot_weekly::ActiveModel {
        let now = Utc::now();

        beatmapset_snapshot_weekly::ActiveModel {
            osu_beatmapset_id: Set(row.osu_beatmapset_id),
            snapshot_week: Set(row.snapshot_week),
            status: Set(row.status),
            playcount: Set(row.playcount),
            favourite_count: Set(row.favourite_count),
            rating: Set(row.rating),
            avg_passcount: Set(row.avg_passcount),
            avg_pass_rate: Set(row.avg_pass_rate),
            min_pass_rate: Set(row.min_pass_rate),
            max_pass_rate: Set(row.max_pass_rate),
            last_updated: Set(row.last_updated),
            updated_at: Set(now),
        }
    }
}
