use std::collections::HashMap;

use chrono::Utc;
use sea_orm::sea_query::OnConflict;
use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set,
};

use crate::entities::leaderboard_position_current;

#[derive(Clone, Debug)]
pub struct NewLeaderboardPositionCurrentRow {
    pub leaderboard_key: String,
    pub osu_user_id: i64,
    pub current_rank: i32,
    pub previous_rank: Option<i32>,
    pub rank_delta: i32,
    pub measured_at: chrono::DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct LeaderboardPositionCurrentRepo {
    db: DatabaseConnection,
}

impl LeaderboardPositionCurrentRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    pub async fn list_by_leaderboard(
        &self,
        leaderboard_key: &str,
    ) -> Result<Vec<leaderboard_position_current::Model>, DbErr> {
        leaderboard_position_current::Entity::find()
            .filter(leaderboard_position_current::Column::LeaderboardKey.eq(leaderboard_key))
            .order_by_asc(leaderboard_position_current::Column::CurrentRank)
            .all(&self.db)
            .await
    }

    pub async fn list_by_osu_user_id(
        &self,
        osu_user_id: i64,
    ) -> Result<Vec<leaderboard_position_current::Model>, DbErr> {
        leaderboard_position_current::Entity::find()
            .filter(leaderboard_position_current::Column::OsuUserId.eq(osu_user_id))
            .all(&self.db)
            .await
    }

    pub async fn list_page_by_rank(
        &self,
        leaderboard_key: &str,
        after_rank: Option<i32>,
        limit: u64,
    ) -> Result<Vec<leaderboard_position_current::Model>, DbErr> {
        let query = leaderboard_position_current::Entity::find()
            .filter(leaderboard_position_current::Column::LeaderboardKey.eq(leaderboard_key));

        let query = match after_rank {
            Some(after_rank) => {
                query.filter(leaderboard_position_current::Column::CurrentRank.gt(after_rank))
            }
            None => query,
        };

        query
            .order_by_asc(leaderboard_position_current::Column::CurrentRank)
            .limit(limit)
            .all(&self.db)
            .await
    }

    pub async fn list_rank_map(
        &self,
        leaderboard_key: &str,
    ) -> Result<HashMap<i64, leaderboard_position_current::Model>, DbErr> {
        let rows = self.list_by_leaderboard(leaderboard_key).await?;
        Ok(rows.into_iter().map(|row| (row.osu_user_id, row)).collect())
    }

    pub async fn replace_for_leaderboard_with<C: ConnectionTrait>(
        &self,
        db: &C,
        leaderboard_key: &str,
        rows: Vec<NewLeaderboardPositionCurrentRow>,
    ) -> Result<(), DbErr> {
        leaderboard_position_current::Entity::delete_many()
            .filter(leaderboard_position_current::Column::LeaderboardKey.eq(leaderboard_key))
            .exec(db)
            .await?;

        if rows.is_empty() {
            return Ok(());
        }

        for row in rows {
            leaderboard_position_current::Entity::insert(self.to_active(row))
                .on_conflict(
                    OnConflict::columns([
                        leaderboard_position_current::Column::LeaderboardKey,
                        leaderboard_position_current::Column::OsuUserId,
                    ])
                    .update_columns([
                        leaderboard_position_current::Column::CurrentRank,
                        leaderboard_position_current::Column::PreviousRank,
                        leaderboard_position_current::Column::RankDelta,
                        leaderboard_position_current::Column::MeasuredAt,
                        leaderboard_position_current::Column::UpdatedAt,
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
        row: NewLeaderboardPositionCurrentRow,
    ) -> leaderboard_position_current::ActiveModel {
        let now = Utc::now();

        leaderboard_position_current::ActiveModel {
            leaderboard_key: Set(row.leaderboard_key),
            osu_user_id: Set(row.osu_user_id),
            current_rank: Set(row.current_rank),
            previous_rank: Set(row.previous_rank),
            rank_delta: Set(row.rank_delta),
            measured_at: Set(row.measured_at),
            updated_at: Set(now),
        }
    }
}
