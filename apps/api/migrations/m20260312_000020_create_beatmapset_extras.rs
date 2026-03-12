use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BeatmapsetExtras::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BeatmapsetExtras::OsuBeatmapsetId)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(BeatmapsetExtras::CreatorId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BeatmapsetExtras::CreatorName)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BeatmapsetExtras::AnimeCover).text()
                    )
                    .col(
                        ColumnDef::new(BeatmapsetExtras::DetailsUnavailable)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(BeatmapsetExtras::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                INSERT INTO beatmapset_extras (
                    osu_beatmapset_id,
                    creator_id,
                    creator_name,
                    anime_cover,
                    details_unavailable,
                    updated_at
                )
                SELECT
                    bp.osu_beatmapset_id,
                    bp.creator_id,
                    bp.creator_name,
                    NULLIF(COALESCE(
                        b.raw->>'animeCover',
                        b.raw->>'anime_cover',
                        ''
                    ), ''),
                    false,
                    CURRENT_TIMESTAMP
                FROM beatmapset_profiles bp
                LEFT JOIN beatmapsets b
                    ON b.osu_beatmapset_id = bp.osu_beatmapset_id
                ON CONFLICT (osu_beatmapset_id) DO UPDATE SET
                    creator_id = EXCLUDED.creator_id,
                    creator_name = EXCLUDED.creator_name,
                    anime_cover = EXCLUDED.anime_cover,
                    details_unavailable = EXCLUDED.details_unavailable,
                    updated_at = CURRENT_TIMESTAMP
                "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(BeatmapsetExtras::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum BeatmapsetExtras {
    Table,
    OsuBeatmapsetId,
    CreatorId,
    CreatorName,
    AnimeCover,
    DetailsUnavailable,
    UpdatedAt,
}
