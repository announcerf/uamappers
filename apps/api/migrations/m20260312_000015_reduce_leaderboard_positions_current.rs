use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(LeaderboardPositionsCurrent::Table)
                    .drop_column(LeaderboardPositionsCurrent::RankDelta)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(LeaderboardPositionsCurrent::Table)
                    .add_column(
                        ColumnDef::new(LeaderboardPositionsCurrent::RankDelta)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum LeaderboardPositionsCurrent {
    Table,
    RankDelta,
}
