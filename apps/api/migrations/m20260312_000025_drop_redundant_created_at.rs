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
                ALTER TABLE beatmapset_profiles DROP COLUMN IF EXISTS created_at;
                ALTER TABLE beatmap_profiles DROP COLUMN IF EXISTS created_at;
                ALTER TABLE beatmapset_extras DROP COLUMN IF EXISTS created_at;
                ALTER TABLE osu_user_beatmapsets DROP COLUMN IF EXISTS created_at;
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
                    ADD COLUMN IF NOT EXISTS created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP;
                ALTER TABLE beatmap_profiles
                    ADD COLUMN IF NOT EXISTS created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP;
                ALTER TABLE beatmapset_extras
                    ADD COLUMN IF NOT EXISTS created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP;
                ALTER TABLE osu_user_beatmapsets
                    ADD COLUMN IF NOT EXISTS created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP;
                "#,
            )
            .await?;

        Ok(())
    }
}
