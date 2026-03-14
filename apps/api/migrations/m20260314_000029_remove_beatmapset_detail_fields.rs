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
                ALTER TABLE beatmapset_profiles DROP COLUMN IF EXISTS source;
                ALTER TABLE beatmapset_profiles DROP COLUMN IF EXISTS tags;
                ALTER TABLE beatmapset_profiles DROP COLUMN IF EXISTS genre;
                ALTER TABLE beatmapset_profiles DROP COLUMN IF EXISTS language;

                ALTER TABLE beatmapset_extras DROP COLUMN IF EXISTS details_unavailable;
                "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                ALTER TABLE beatmapset_profiles
                    ADD COLUMN IF NOT EXISTS source text NOT NULL DEFAULT '',
                    ADD COLUMN IF NOT EXISTS tags text NOT NULL DEFAULT '',
                    ADD COLUMN IF NOT EXISTS genre smallint NOT NULL DEFAULT 2,
                    ADD COLUMN IF NOT EXISTS language smallint NOT NULL DEFAULT 15;

                ALTER TABLE beatmapset_extras
                    ADD COLUMN IF NOT EXISTS details_unavailable boolean NOT NULL DEFAULT false;
                "#,
            )
            .await?;

        Ok(())
    }
}
