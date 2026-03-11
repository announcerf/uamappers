use std::collections::HashSet;

use chrono::Utc;
use sea_orm::sea_query::extension::postgres::PgExpr;
use sea_orm::sea_query::{Expr, Func, OnConflict};
use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, Set,
};

use crate::entities::ua_mapper;

#[derive(Clone, Debug)]
pub struct UaMapperRepo {
    db: DatabaseConnection,
}

impl UaMapperRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    pub async fn list(
        &self,
        limit: u64,
        offset: u64,
    ) -> Result<(Vec<ua_mapper::Model>, u64), DbErr> {
        let total = ua_mapper::Entity::find().count(&self.db).await?;

        let rows = ua_mapper::Entity::find()
            .order_by_asc(ua_mapper::Column::OsuUserId)
            .limit(limit)
            .offset(offset)
            .all(&self.db)
            .await?;

        Ok((rows, total))
    }

    pub async fn count_all(&self) -> Result<u64, DbErr> {
        ua_mapper::Entity::find().count(&self.db).await
    }

    pub async fn list_after_id(
        &self,
        after_id: i64,
        limit: u64,
    ) -> Result<Vec<ua_mapper::Model>, DbErr> {
        ua_mapper::Entity::find()
            .filter(ua_mapper::Column::OsuUserId.gt(after_id))
            .order_by_asc(ua_mapper::Column::OsuUserId)
            .limit(limit)
            .all(&self.db)
            .await
    }

    pub async fn get_by_osu_user_id(
        &self,
        osu_user_id: i64,
    ) -> Result<Option<ua_mapper::Model>, DbErr> {
        ua_mapper::Entity::find_by_id(osu_user_id)
            .one(&self.db)
            .await
    }

    pub async fn get_by_username(&self, username: &str) -> Result<Option<ua_mapper::Model>, DbErr> {
        let username = username.trim();
        if username.is_empty() {
            return Ok(None);
        }

        let username_lower = username.to_ascii_lowercase();

        ua_mapper::Entity::find()
            .filter(
                Expr::expr(Func::lower(Expr::col(ua_mapper::Column::Username))).eq(username_lower),
            )
            .one(&self.db)
            .await
    }

    pub async fn search(
        &self,
        query: &str,
        limit: u64,
        offset: u64,
    ) -> Result<(Vec<ua_mapper::Model>, u64), DbErr> {
        let total = ua_mapper::Entity::find()
            .filter(Expr::col(ua_mapper::Column::Username).ilike(format!("%{}%", query)))
            .count(&self.db)
            .await?;

        let rows = ua_mapper::Entity::find()
            .filter(Expr::col(ua_mapper::Column::Username).ilike(format!("%{}%", query)))
            .order_by_asc(ua_mapper::Column::OsuUserId)
            .limit(limit)
            .offset(offset)
            .all(&self.db)
            .await?;

        Ok((rows, total))
    }

    pub async fn search_after_id(
        &self,
        query: &str,
        after_id: Option<i64>,
        limit: u64,
    ) -> Result<(Vec<ua_mapper::Model>, u64), DbErr> {
        let base = ua_mapper::Entity::find()
            .filter(Expr::col(ua_mapper::Column::Username).ilike(format!("%{}%", query)));

        let total = base.clone().count(&self.db).await?;

        let base = match after_id {
            Some(after_id) => base.filter(ua_mapper::Column::OsuUserId.gt(after_id)),
            None => base,
        };

        let rows = base
            .order_by_asc(ua_mapper::Column::OsuUserId)
            .limit(limit)
            .all(&self.db)
            .await?;

        Ok((rows, total))
    }

    pub async fn list_existing_ids(&self, ids: &[i64]) -> Result<HashSet<i64>, DbErr> {
        if ids.is_empty() {
            return Ok(HashSet::new());
        }

        let rows: Vec<ua_mapper::Model> = ua_mapper::Entity::find()
            .filter(ua_mapper::Column::OsuUserId.is_in(ids.to_vec()))
            .all(&self.db)
            .await?;

        Ok(rows.into_iter().map(|r| r.osu_user_id).collect())
    }

    pub async fn upsert_with<C: ConnectionTrait>(
        &self,
        db: &C,
        osu_user_id: i64,
        username: &str,
        country_code: &str,
    ) -> Result<(), DbErr> {
        let now = Utc::now();
        let active = ua_mapper::ActiveModel {
            osu_user_id: Set(osu_user_id),
            username: Set(username.to_string()),
            country_code: Set(country_code.to_string()),
            first_seen_at: Set(now),
            last_seen_at: Set(now),
            created_at: Set(now),
            updated_at: Set(now),
        };

        ua_mapper::Entity::insert(active)
            .on_conflict(
                OnConflict::column(ua_mapper::Column::OsuUserId)
                    .update_columns([
                        ua_mapper::Column::Username,
                        ua_mapper::Column::CountryCode,
                        ua_mapper::Column::LastSeenAt,
                        ua_mapper::Column::UpdatedAt,
                    ])
                    .to_owned(),
            )
            .exec(db)
            .await?;

        Ok(())
    }
}
