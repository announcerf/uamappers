use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                UPDATE beatmapsets
                SET raw = CASE
                    WHEN jsonb_typeof(raw) = 'object' THEN jsonb_strip_nulls(jsonb_build_object(
                        'ratings', raw->'ratings',
                        'animeCover', raw->'anime_cover'
                    ))
                    ELSE raw
                END
                "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
