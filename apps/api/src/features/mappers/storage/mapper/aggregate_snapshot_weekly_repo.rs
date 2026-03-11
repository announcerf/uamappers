use chrono::Utc;
use sea_orm::sea_query::OnConflict;
use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder,
    Set,
};

use crate::entities::mapper_aggregate_snapshot_weekly;

#[derive(Clone, Debug)]
pub struct NewMapperAggregateSnapshotWeeklyRow {
    pub osu_user_id: i64,
    pub snapshot_week: chrono::DateTime<Utc>,
    pub total_mapsets: i32,
    pub ranked_mapsets: i32,
    pub loved_mapsets: i32,
    pub guest_mapsets: i32,
    pub nominated_mapsets: i32,
    pub graveyard_mapsets: i32,
    pub pending_mapsets: i32,
    pub total_playcount: i64,
    pub avg_rating: f32,
    pub avg_stars: f32,
    pub avg_bpm: f32,
    pub avg_length_seconds: f32,
    pub main_mode: String,
}

#[derive(Clone, Debug)]
pub struct MapperAggregateSnapshotWeeklyRepo {
    db: DatabaseConnection,
}

impl MapperAggregateSnapshotWeeklyRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    pub async fn list_by_osu_user_id(
        &self,
        osu_user_id: i64,
    ) -> Result<Vec<mapper_aggregate_snapshot_weekly::Model>, DbErr> {
        mapper_aggregate_snapshot_weekly::Entity::find()
            .filter(mapper_aggregate_snapshot_weekly::Column::OsuUserId.eq(osu_user_id))
            .order_by_asc(mapper_aggregate_snapshot_weekly::Column::SnapshotWeek)
            .all(&self.db)
            .await
    }

    pub async fn upsert_with<C: ConnectionTrait>(
        &self,
        db: &C,
        row: NewMapperAggregateSnapshotWeeklyRow,
    ) -> Result<(), DbErr> {
        let active = self.to_active(row);

        mapper_aggregate_snapshot_weekly::Entity::insert(active)
            .on_conflict(
                OnConflict::columns([
                    mapper_aggregate_snapshot_weekly::Column::OsuUserId,
                    mapper_aggregate_snapshot_weekly::Column::SnapshotWeek,
                ])
                .update_columns([
                    mapper_aggregate_snapshot_weekly::Column::TotalMapsets,
                    mapper_aggregate_snapshot_weekly::Column::RankedMapsets,
                    mapper_aggregate_snapshot_weekly::Column::LovedMapsets,
                    mapper_aggregate_snapshot_weekly::Column::GuestMapsets,
                    mapper_aggregate_snapshot_weekly::Column::NominatedMapsets,
                    mapper_aggregate_snapshot_weekly::Column::GraveyardMapsets,
                    mapper_aggregate_snapshot_weekly::Column::PendingMapsets,
                    mapper_aggregate_snapshot_weekly::Column::TotalPlaycount,
                    mapper_aggregate_snapshot_weekly::Column::AvgRating,
                    mapper_aggregate_snapshot_weekly::Column::AvgStars,
                    mapper_aggregate_snapshot_weekly::Column::AvgBpm,
                    mapper_aggregate_snapshot_weekly::Column::AvgLengthSeconds,
                    mapper_aggregate_snapshot_weekly::Column::MainMode,
                    mapper_aggregate_snapshot_weekly::Column::UpdatedAt,
                ])
                .to_owned(),
            )
            .exec(db)
            .await?;

        Ok(())
    }

    fn to_active(
        &self,
        row: NewMapperAggregateSnapshotWeeklyRow,
    ) -> mapper_aggregate_snapshot_weekly::ActiveModel {
        let now = Utc::now();

        mapper_aggregate_snapshot_weekly::ActiveModel {
            osu_user_id: Set(row.osu_user_id),
            snapshot_week: Set(row.snapshot_week),
            total_mapsets: Set(row.total_mapsets),
            ranked_mapsets: Set(row.ranked_mapsets),
            loved_mapsets: Set(row.loved_mapsets),
            guest_mapsets: Set(row.guest_mapsets),
            nominated_mapsets: Set(row.nominated_mapsets),
            graveyard_mapsets: Set(row.graveyard_mapsets),
            pending_mapsets: Set(row.pending_mapsets),
            total_playcount: Set(row.total_playcount),
            avg_rating: Set(row.avg_rating),
            avg_stars: Set(row.avg_stars),
            avg_bpm: Set(row.avg_bpm),
            avg_length_seconds: Set(row.avg_length_seconds),
            main_mode: Set(row.main_mode),
            updated_at: Set(now),
        }
    }
}
