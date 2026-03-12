use chrono::Utc;
use sea_orm::sea_query::OnConflict;
use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder,
    Set,
};

use crate::entities::mapper_stats_current;

#[derive(Clone, Debug)]
pub struct NewMapperStatsCurrentRow {
    pub osu_user_id: i64,
    pub total_mapsets: i32,
    pub ranked_mapsets: i32,
    pub loved_mapsets: i32,
    pub guest_mapsets: i32,
    pub nominated_mapsets: i32,
    pub graveyard_mapsets: i32,
    pub pending_mapsets: i32,
    pub total_playcount: i64,
    pub first_submitted_date: Option<chrono::DateTime<Utc>>,
    pub first_ranked_date: Option<chrono::DateTime<Utc>>,
    pub last_mapset_updated_at: Option<chrono::DateTime<Utc>>,
    pub main_mode: i16,
    pub mapping_followers: i32,
    pub kudosu_available: i32,
    pub kudosu_total: i32,
}

#[derive(Clone, Debug)]
pub struct MapperStatsCurrentRepo {
    db: DatabaseConnection,
}

impl MapperStatsCurrentRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    pub async fn get_by_osu_user_id(
        &self,
        osu_user_id: i64,
    ) -> Result<Option<mapper_stats_current::Model>, DbErr> {
        mapper_stats_current::Entity::find_by_id(osu_user_id)
            .one(&self.db)
            .await
    }

    pub async fn list_by_osu_user_ids(
        &self,
        ids: &[i64],
    ) -> Result<Vec<mapper_stats_current::Model>, DbErr> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        mapper_stats_current::Entity::find()
            .filter(mapper_stats_current::Column::OsuUserId.is_in(ids.to_vec()))
            .all(&self.db)
            .await
    }

    pub async fn list_for_leaderboard(
        &self,
        leaderboard_key: &str,
    ) -> Result<Vec<mapper_stats_current::Model>, DbErr> {
        let query = mapper_stats_current::Entity::find();

        let query = match leaderboard_key {
            "followers" => query
                .filter(mapper_stats_current::Column::MappingFollowers.gt(0))
                .order_by_desc(mapper_stats_current::Column::MappingFollowers),
            "ranked" => query
                .filter(mapper_stats_current::Column::RankedMapsets.gt(0))
                .order_by_desc(mapper_stats_current::Column::RankedMapsets),
            "guest_diff" => query
                .filter(mapper_stats_current::Column::GuestMapsets.gt(0))
                .order_by_desc(mapper_stats_current::Column::GuestMapsets),
            "plays" => query
                .filter(mapper_stats_current::Column::TotalPlaycount.gt(0))
                .order_by_desc(mapper_stats_current::Column::TotalPlaycount),
            "kudosu" => query
                .filter(mapper_stats_current::Column::KudosuTotal.gt(0))
                .order_by_desc(mapper_stats_current::Column::KudosuTotal),
            "nominations" => query
                .filter(mapper_stats_current::Column::NominatedMapsets.gt(0))
                .order_by_desc(mapper_stats_current::Column::NominatedMapsets),
            _ => query
                .filter(mapper_stats_current::Column::RankedMapsets.gt(0))
                .order_by_desc(mapper_stats_current::Column::RankedMapsets),
        };

        query
            .order_by_asc(mapper_stats_current::Column::OsuUserId)
            .all(&self.db)
            .await
    }

    pub async fn upsert_with<C: ConnectionTrait>(
        &self,
        db: &C,
        row: NewMapperStatsCurrentRow,
    ) -> Result<(), DbErr> {
        let active = self.to_active(row);

        mapper_stats_current::Entity::insert(active)
            .on_conflict(
                OnConflict::column(mapper_stats_current::Column::OsuUserId)
                    .update_columns([
                        mapper_stats_current::Column::TotalMapsets,
                        mapper_stats_current::Column::RankedMapsets,
                        mapper_stats_current::Column::LovedMapsets,
                        mapper_stats_current::Column::GuestMapsets,
                        mapper_stats_current::Column::NominatedMapsets,
                        mapper_stats_current::Column::GraveyardMapsets,
                        mapper_stats_current::Column::PendingMapsets,
                        mapper_stats_current::Column::TotalPlaycount,
                        mapper_stats_current::Column::FirstSubmittedDate,
                        mapper_stats_current::Column::FirstRankedDate,
                        mapper_stats_current::Column::LastMapsetUpdatedAt,
                        mapper_stats_current::Column::MainMode,
                        mapper_stats_current::Column::MappingFollowers,
                        mapper_stats_current::Column::KudosuAvailable,
                        mapper_stats_current::Column::KudosuTotal,
                        mapper_stats_current::Column::UpdatedAt,
                    ])
                    .to_owned(),
            )
            .exec(db)
            .await?;

        Ok(())
    }

    fn to_active(&self, row: NewMapperStatsCurrentRow) -> mapper_stats_current::ActiveModel {
        let now = Utc::now();

        mapper_stats_current::ActiveModel {
            osu_user_id: Set(row.osu_user_id),
            total_mapsets: Set(row.total_mapsets),
            ranked_mapsets: Set(row.ranked_mapsets),
            loved_mapsets: Set(row.loved_mapsets),
            guest_mapsets: Set(row.guest_mapsets),
            nominated_mapsets: Set(row.nominated_mapsets),
            graveyard_mapsets: Set(row.graveyard_mapsets),
            pending_mapsets: Set(row.pending_mapsets),
            total_playcount: Set(row.total_playcount),
            first_submitted_date: Set(row.first_submitted_date),
            first_ranked_date: Set(row.first_ranked_date),
            last_mapset_updated_at: Set(row.last_mapset_updated_at),
            main_mode: Set(row.main_mode),
            mapping_followers: Set(row.mapping_followers),
            kudosu_available: Set(row.kudosu_available),
            kudosu_total: Set(row.kudosu_total),
            updated_at: Set(now),
        }
    }
}
