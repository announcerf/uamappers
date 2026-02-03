use chrono::Utc;
use sea_orm::sea_query::extension::postgres::PgExpr;
use sea_orm::sea_query::{Expr, OnConflict};
use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, Set,
};

use crate::entities::mapper;
use crate::features::mappers::domain::model::MapperStats;

#[derive(Clone, Debug)]
pub struct MapperRepo {
    db: DatabaseConnection,
}

impl MapperRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    pub async fn list_ua(
        &self,
        limit: u64,
        offset: u64,
    ) -> Result<(Vec<mapper::Model>, u64), DbErr> {
        let total = mapper::Entity::find()
            .filter(mapper::Column::CountryCode.eq("UA"))
            .count(&self.db)
            .await?;

        let rows = mapper::Entity::find()
            .filter(mapper::Column::CountryCode.eq("UA"))
            .order_by_asc(mapper::Column::OsuUserId)
            .limit(limit)
            .offset(offset)
            .all(&self.db)
            .await?;

        Ok((rows, total))
    }

    pub async fn get_by_osu_user_id(
        &self,
        osu_user_id: i64,
    ) -> Result<Option<mapper::Model>, DbErr> {
        mapper::Entity::find_by_id(osu_user_id).one(&self.db).await
    }

    pub async fn search_ua(
        &self,
        query: &str,
        limit: u64,
        offset: u64,
    ) -> Result<(Vec<mapper::Model>, u64), DbErr> {
        let total = mapper::Entity::find()
            .filter(mapper::Column::CountryCode.eq("UA"))
            .filter(Expr::col(mapper::Column::Username).ilike(format!("%{}%", query)))
            .count(&self.db)
            .await?;

        let rows = mapper::Entity::find()
            .filter(mapper::Column::CountryCode.eq("UA"))
            .filter(Expr::col(mapper::Column::Username).ilike(format!("%{}%", query)))
            .order_by_asc(mapper::Column::OsuUserId)
            .limit(limit)
            .offset(offset)
            .all(&self.db)
            .await?;

        Ok((rows, total))
    }

    pub async fn upsert(&self, stats: &MapperStats) -> Result<(), DbErr> {
        self.upsert_with(&self.db, stats).await
    }

    pub async fn upsert_with<C: ConnectionTrait>(
        &self,
        db: &C,
        stats: &MapperStats,
    ) -> Result<(), DbErr> {
        let now = Utc::now();
        let active = mapper::ActiveModel {
            osu_user_id: Set(stats.osu_user_id),
            username: Set(stats.username.clone()),
            country_code: Set(stats.country_code.clone()),
            count_graveyard: Set(stats.count_graveyard),
            count_pending: Set(stats.count_pending),
            count_wip: Set(stats.count_wip),
            count_loved: Set(stats.count_loved),
            count_ranked: Set(stats.count_ranked),
            count_approved: Set(stats.count_approved),
            count_total: Set(stats.count_total),
            is_bn: Set(stats.is_bn),
            nominated_count: Set(stats.nominated_count),
            created_at: Set(now),
            updated_at: Set(now),
        };

        mapper::Entity::insert(active)
            .on_conflict(
                OnConflict::column(mapper::Column::OsuUserId)
                    .update_columns([
                        mapper::Column::Username,
                        mapper::Column::CountryCode,
                        mapper::Column::CountGraveyard,
                        mapper::Column::CountPending,
                        mapper::Column::CountWip,
                        mapper::Column::CountLoved,
                        mapper::Column::CountRanked,
                        mapper::Column::CountApproved,
                        mapper::Column::CountTotal,
                        mapper::Column::IsBn,
                        mapper::Column::NominatedCount,
                        mapper::Column::UpdatedAt,
                    ])
                    .to_owned(),
            )
            .exec(db)
            .await?;

        Ok(())
    }

    pub async fn increment_with<C: ConnectionTrait>(
        &self,
        db: &C,
        stats: &MapperStats,
    ) -> Result<(), DbErr> {
        let now = Utc::now();
        let active = mapper::ActiveModel {
            osu_user_id: Set(stats.osu_user_id),
            username: Set(stats.username.clone()),
            country_code: Set(stats.country_code.clone()),
            count_graveyard: Set(stats.count_graveyard),
            count_pending: Set(stats.count_pending),
            count_wip: Set(stats.count_wip),
            count_loved: Set(stats.count_loved),
            count_ranked: Set(stats.count_ranked),
            count_approved: Set(stats.count_approved),
            count_total: Set(stats.count_total),
            is_bn: Set(stats.is_bn),
            nominated_count: Set(stats.nominated_count),
            created_at: Set(now),
            updated_at: Set(now),
        };

        mapper::Entity::insert(active)
            .on_conflict(
                OnConflict::column(mapper::Column::OsuUserId)
                    .do_nothing()
                    .to_owned(),
            )
            .exec(db)
            .await?;

        mapper::Entity::update_many()
            .filter(mapper::Column::OsuUserId.eq(stats.osu_user_id))
            .col_expr(
                mapper::Column::Username,
                Expr::value(stats.username.clone()),
            )
            .col_expr(
                mapper::Column::CountryCode,
                Expr::value(stats.country_code.clone()),
            )
            .col_expr(
                mapper::Column::CountGraveyard,
                Expr::col(mapper::Column::CountGraveyard).add(stats.count_graveyard),
            )
            .col_expr(
                mapper::Column::CountPending,
                Expr::col(mapper::Column::CountPending).add(stats.count_pending),
            )
            .col_expr(
                mapper::Column::CountWip,
                Expr::col(mapper::Column::CountWip).add(stats.count_wip),
            )
            .col_expr(
                mapper::Column::CountLoved,
                Expr::col(mapper::Column::CountLoved).add(stats.count_loved),
            )
            .col_expr(
                mapper::Column::CountRanked,
                Expr::col(mapper::Column::CountRanked).add(stats.count_ranked),
            )
            .col_expr(
                mapper::Column::CountApproved,
                Expr::col(mapper::Column::CountApproved).add(stats.count_approved),
            )
            .col_expr(
                mapper::Column::CountTotal,
                Expr::col(mapper::Column::CountTotal).add(stats.count_total),
            )
            .col_expr(mapper::Column::IsBn, Expr::value(stats.is_bn))
            .col_expr(
                mapper::Column::NominatedCount,
                Expr::value(stats.nominated_count),
            )
            .col_expr(mapper::Column::UpdatedAt, Expr::value(now))
            .exec(db)
            .await?;

        Ok(())
    }
}
