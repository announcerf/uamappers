use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(BeatmapsetSnapshotsWeekly::Table)
                    .drop_column(BeatmapsetSnapshotsWeekly::AvgPasscount)
                    .drop_column(BeatmapsetSnapshotsWeekly::AvgPassRate)
                    .add_column(
                        ColumnDef::new(BeatmapsetSnapshotsWeekly::BeatmapCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .add_column(
                        ColumnDef::new(BeatmapsetSnapshotsWeekly::PasscountSum)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .add_column(
                        ColumnDef::new(BeatmapsetSnapshotsWeekly::PassRateSum)
                            .float()
                            .not_null()
                            .default(0.0),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(BeatmapsetSnapshotsWeekly::Table)
                    .drop_column(BeatmapsetSnapshotsWeekly::BeatmapCount)
                    .drop_column(BeatmapsetSnapshotsWeekly::PasscountSum)
                    .drop_column(BeatmapsetSnapshotsWeekly::PassRateSum)
                    .add_column(
                        ColumnDef::new(BeatmapsetSnapshotsWeekly::AvgPasscount)
                            .float()
                            .not_null()
                            .default(0.0),
                    )
                    .add_column(
                        ColumnDef::new(BeatmapsetSnapshotsWeekly::AvgPassRate)
                            .float()
                            .not_null()
                            .default(0.0),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum BeatmapsetSnapshotsWeekly {
    Table,
    AvgPasscount,
    AvgPassRate,
    BeatmapCount,
    PasscountSum,
    PassRateSum,
}
