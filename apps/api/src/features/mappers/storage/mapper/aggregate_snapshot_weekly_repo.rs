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
    pub rating_sum: f32,
    pub beatmap_count: i32,
    pub stars_sum: f32,
    pub bpm_sum: f32,
    pub length_seconds_sum: f32,
    pub main_mode: i16,
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
                    mapper_aggregate_snapshot_weekly::Column::RatingSum,
                    mapper_aggregate_snapshot_weekly::Column::BeatmapCount,
                    mapper_aggregate_snapshot_weekly::Column::StarsSum,
                    mapper_aggregate_snapshot_weekly::Column::BpmSum,
                    mapper_aggregate_snapshot_weekly::Column::LengthSecondsSum,
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
            rating_sum: Set(row.rating_sum),
            beatmap_count: Set(row.beatmap_count),
            stars_sum: Set(row.stars_sum),
            bpm_sum: Set(row.bpm_sum),
            length_seconds_sum: Set(row.length_seconds_sum),
            main_mode: Set(row.main_mode),
            updated_at: Set(now),
        }
    }
}
