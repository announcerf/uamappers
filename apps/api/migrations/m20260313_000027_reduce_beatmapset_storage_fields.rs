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
                ALTER TABLE beatmapset_profiles DROP COLUMN IF EXISTS discussion_enabled;
                ALTER TABLE beatmapset_profiles DROP COLUMN IF EXISTS discussion_locked;
                ALTER TABLE beatmapset_profiles DROP COLUMN IF EXISTS can_be_hyped;
                ALTER TABLE beatmapset_profiles DROP COLUMN IF EXISTS is_scoreable;
                ALTER TABLE beatmapset_profiles DROP COLUMN IF EXISTS download_disabled;
                ALTER TABLE beatmapset_profiles DROP COLUMN IF EXISTS hype_current;
                ALTER TABLE beatmapset_profiles DROP COLUMN IF EXISTS hype_required;
                ALTER TABLE beatmapset_profiles DROP COLUMN IF EXISTS nominations_current;
                ALTER TABLE beatmapset_profiles DROP COLUMN IF EXISTS preview_url;

                ALTER TABLE beatmap_profiles DROP COLUMN IF EXISTS is_scoreable;
                ALTER TABLE beatmapset_extras DROP COLUMN IF EXISTS ratings_json;
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
                ALTER TABLE beatmapset_profiles ADD COLUMN IF NOT EXISTS discussion_enabled boolean NOT NULL DEFAULT false;
                ALTER TABLE beatmapset_profiles ADD COLUMN IF NOT EXISTS discussion_locked boolean NOT NULL DEFAULT false;
                ALTER TABLE beatmapset_profiles ADD COLUMN IF NOT EXISTS can_be_hyped boolean NOT NULL DEFAULT false;
                ALTER TABLE beatmapset_profiles ADD COLUMN IF NOT EXISTS is_scoreable boolean NOT NULL DEFAULT false;
                ALTER TABLE beatmapset_profiles ADD COLUMN IF NOT EXISTS download_disabled boolean NOT NULL DEFAULT false;
                ALTER TABLE beatmapset_profiles ADD COLUMN IF NOT EXISTS hype_current integer NOT NULL DEFAULT 0;
                ALTER TABLE beatmapset_profiles ADD COLUMN IF NOT EXISTS hype_required integer NOT NULL DEFAULT 0;
                ALTER TABLE beatmapset_profiles ADD COLUMN IF NOT EXISTS nominations_current integer NOT NULL DEFAULT 0;
                ALTER TABLE beatmapset_profiles ADD COLUMN IF NOT EXISTS preview_url text NOT NULL DEFAULT '';

                ALTER TABLE beatmap_profiles ADD COLUMN IF NOT EXISTS is_scoreable boolean NOT NULL DEFAULT false;
                ALTER TABLE beatmapset_extras ADD COLUMN IF NOT EXISTS ratings_json jsonb NOT NULL DEFAULT '[]'::jsonb;
                "#,
            )
            .await?;

        Ok(())
    }
}
