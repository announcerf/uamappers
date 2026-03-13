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
                UPDATE beatmapset_profiles
                SET genre = 2
                WHERE genre IS NULL;

                UPDATE beatmapset_profiles
                SET language = 15
                WHERE language IS NULL;

                ALTER TABLE beatmapset_profiles
                    ALTER COLUMN genre SET DEFAULT 2,
                    ALTER COLUMN genre SET NOT NULL,
                    ALTER COLUMN language SET DEFAULT 15,
                    ALTER COLUMN language SET NOT NULL;
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
                    ALTER COLUMN genre DROP NOT NULL,
                    ALTER COLUMN genre DROP DEFAULT,
                    ALTER COLUMN language DROP NOT NULL,
                    ALTER COLUMN language DROP DEFAULT;
                "#,
            )
            .await?;

        Ok(())
    }
}
