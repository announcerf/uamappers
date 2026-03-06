use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OsuUserBeatmapsets::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OsuUserBeatmapsets::OsuUserId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(OsuUserBeatmapsets::Kind).string().not_null())
                    .col(
                        ColumnDef::new(OsuUserBeatmapsets::OsuBeatmapsetId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OsuUserBeatmapsets::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(OsuUserBeatmapsets::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        Index::create()
                            .col(OsuUserBeatmapsets::OsuUserId)
                            .col(OsuUserBeatmapsets::Kind)
                            .col(OsuUserBeatmapsets::OsuBeatmapsetId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_osu_user_beatmapsets_user_kind")
                    .table(OsuUserBeatmapsets::Table)
                    .if_not_exists()
                    .col(OsuUserBeatmapsets::OsuUserId)
                    .col(OsuUserBeatmapsets::Kind)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(OsuUserBeatmapsets::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum OsuUserBeatmapsets {
    Table,
    OsuUserId,
    Kind,
    OsuBeatmapsetId,
    CreatedAt,
    UpdatedAt,
}
