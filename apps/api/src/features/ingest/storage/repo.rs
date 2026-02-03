use chrono::Utc;
use sea_orm::sea_query::OnConflict;
use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};

use crate::entities::scan_state;

#[derive(Clone, Debug)]
pub struct ScanStateRepo {
    db: DatabaseConnection,
}

impl ScanStateRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list_all(&self) -> Result<Vec<scan_state::Model>, DbErr> {
        scan_state::Entity::find().all(&self.db).await
    }

    pub async fn get_by_name(&self, name: &str) -> Result<Option<scan_state::Model>, DbErr> {
        scan_state::Entity::find_by_id(name.to_string())
            .one(&self.db)
            .await
    }

    pub async fn upsert_cursor(&self, name: &str, cursor: Option<String>) -> Result<(), DbErr> {
        self.upsert_cursor_with(&self.db, name, cursor).await
    }

    pub async fn upsert_cursor_with<C: ConnectionTrait>(
        &self,
        db: &C,
        name: &str,
        cursor: Option<String>,
    ) -> Result<(), DbErr> {
        let now = Utc::now();
        let active = scan_state::ActiveModel {
            name: Set(name.to_string()),
            cursor: Set(cursor),
            last_success_at: Set(None),
            last_error_at: Set(None),
            retry_count: Set(0),
            next_retry_at: Set(None),
            updated_at: Set(now),
        };

        scan_state::Entity::insert(active)
            .on_conflict(
                OnConflict::column(scan_state::Column::Name)
                    .update_columns([scan_state::Column::Cursor, scan_state::Column::UpdatedAt])
                    .to_owned(),
            )
            .exec(db)
            .await?;

        Ok(())
    }

    pub async fn mark_success(&self, name: &str) -> Result<(), DbErr> {
        let now = Utc::now();
        scan_state::Entity::update_many()
            .filter(scan_state::Column::Name.eq(name))
            .set(scan_state::ActiveModel {
                last_success_at: Set(Some(now)),
                last_error_at: Set(None),
                retry_count: Set(0),
                next_retry_at: Set(None),
                updated_at: Set(now),
                ..Default::default()
            })
            .exec(&self.db)
            .await?;
        Ok(())
    }

    pub async fn mark_error(
        &self,
        name: &str,
        retry_count: i32,
        next_retry_at: Option<chrono::DateTime<Utc>>,
    ) -> Result<(), DbErr> {
        let now = Utc::now();
        scan_state::Entity::update_many()
            .filter(scan_state::Column::Name.eq(name))
            .set(scan_state::ActiveModel {
                last_error_at: Set(Some(now)),
                retry_count: Set(retry_count),
                next_retry_at: Set(next_retry_at),
                updated_at: Set(now),
                ..Default::default()
            })
            .exec(&self.db)
            .await?;
        Ok(())
    }
}
