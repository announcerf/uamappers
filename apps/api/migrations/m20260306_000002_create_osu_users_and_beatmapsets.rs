use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OsuUsers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OsuUsers::OsuUserId)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(OsuUsers::Raw)
                            .json_binary()
                            .not_null()
                            .default(Expr::cust("'{}'::jsonb")),
                    )
                    .col(
                        ColumnDef::new(OsuUsers::FetchedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(OsuUsers::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(OsuUsers::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_osu_users_updated_at")
                    .table(OsuUsers::Table)
                    .if_not_exists()
                    .col(OsuUsers::UpdatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Beatmapsets::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Beatmapsets::OsuBeatmapsetId)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Beatmapsets::LastUpdated)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Beatmapsets::Raw)
                            .json_binary()
                            .not_null()
                            .default(Expr::cust("'{}'::jsonb")),
                    )
                    .col(
                        ColumnDef::new(Beatmapsets::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Beatmapsets::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_beatmapsets_last_updated")
                    .table(Beatmapsets::Table)
                    .if_not_exists()
                    .col(Beatmapsets::LastUpdated)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Beatmapsets::Table).if_exists().to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(OsuUsers::Table).if_exists().to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum OsuUsers {
    Table,
    OsuUserId,
    Raw,
    FetchedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Beatmapsets {
    Table,
    OsuBeatmapsetId,
    LastUpdated,
    Raw,
    CreatedAt,
    UpdatedAt,
}
