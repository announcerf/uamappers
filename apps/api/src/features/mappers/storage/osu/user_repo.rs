use chrono::Utc;
use sea_orm::sea_query::OnConflict;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, Set};

use crate::entities::osu_user;

#[derive(Clone, Debug)]
pub struct OsuUserRepo {
    db: DatabaseConnection,
}

impl OsuUserRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    pub async fn get_by_osu_user_id(
        &self,
        osu_user_id: i64,
    ) -> Result<Option<osu_user::Model>, DbErr> {
        osu_user::Entity::find_by_id(osu_user_id)
            .one(&self.db)
            .await
    }

    pub async fn upsert_with<C: ConnectionTrait>(
        &self,
        db: &C,
        osu_user_id: i64,
        raw: sea_orm::JsonValue,
        fetched_at: chrono::DateTime<Utc>,
    ) -> Result<(), DbErr> {
        let now = Utc::now();
        let active = osu_user::ActiveModel {
            osu_user_id: Set(osu_user_id),
            raw: Set(raw),
            fetched_at: Set(fetched_at),
            created_at: Set(now),
            updated_at: Set(now),
        };

        osu_user::Entity::insert(active)
            .on_conflict(
                OnConflict::column(osu_user::Column::OsuUserId)
                    .update_columns([
                        osu_user::Column::Raw,
                        osu_user::Column::FetchedAt,
                        osu_user::Column::UpdatedAt,
                    ])
                    .to_owned(),
            )
            .exec(db)
            .await?;

        Ok(())
    }
}
