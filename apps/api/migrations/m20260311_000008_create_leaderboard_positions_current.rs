use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(LeaderboardPositionsCurrent::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LeaderboardPositionsCurrent::LeaderboardKey)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LeaderboardPositionsCurrent::OsuUserId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LeaderboardPositionsCurrent::CurrentRank)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(LeaderboardPositionsCurrent::PreviousRank).integer())
                    .col(
                        ColumnDef::new(LeaderboardPositionsCurrent::RankDelta)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(LeaderboardPositionsCurrent::MeasuredAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(LeaderboardPositionsCurrent::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        Index::create()
                            .col(LeaderboardPositionsCurrent::LeaderboardKey)
                            .col(LeaderboardPositionsCurrent::OsuUserId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_leaderboard_positions_current_lookup")
                    .table(LeaderboardPositionsCurrent::Table)
                    .if_not_exists()
                    .col(LeaderboardPositionsCurrent::LeaderboardKey)
                    .col(LeaderboardPositionsCurrent::CurrentRank)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(LeaderboardPositionsCurrent::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum LeaderboardPositionsCurrent {
    Table,
    LeaderboardKey,
    OsuUserId,
    CurrentRank,
    PreviousRank,
    RankDelta,
    MeasuredAt,
    UpdatedAt,
}
