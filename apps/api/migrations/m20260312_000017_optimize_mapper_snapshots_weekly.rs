use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(MapperAggregateSnapshotsWeekly::Table)
                    .drop_column(MapperAggregateSnapshotsWeekly::AvgRating)
                    .drop_column(MapperAggregateSnapshotsWeekly::AvgStars)
                    .drop_column(MapperAggregateSnapshotsWeekly::AvgBpm)
                    .drop_column(MapperAggregateSnapshotsWeekly::AvgLengthSeconds)
                    .add_column(
                        ColumnDef::new(MapperAggregateSnapshotsWeekly::RatingSum)
                            .float()
                            .not_null()
                            .default(0.0),
                    )
                    .add_column(
                        ColumnDef::new(MapperAggregateSnapshotsWeekly::BeatmapCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .add_column(
                        ColumnDef::new(MapperAggregateSnapshotsWeekly::StarsSum)
                            .float()
                            .not_null()
                            .default(0.0),
                    )
                    .add_column(
                        ColumnDef::new(MapperAggregateSnapshotsWeekly::BpmSum)
                            .float()
                            .not_null()
                            .default(0.0),
                    )
                    .add_column(
                        ColumnDef::new(MapperAggregateSnapshotsWeekly::LengthSecondsSum)
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
                    .table(MapperAggregateSnapshotsWeekly::Table)
                    .drop_column(MapperAggregateSnapshotsWeekly::RatingSum)
                    .drop_column(MapperAggregateSnapshotsWeekly::BeatmapCount)
                    .drop_column(MapperAggregateSnapshotsWeekly::StarsSum)
                    .drop_column(MapperAggregateSnapshotsWeekly::BpmSum)
                    .drop_column(MapperAggregateSnapshotsWeekly::LengthSecondsSum)
                    .add_column(
                        ColumnDef::new(MapperAggregateSnapshotsWeekly::AvgRating)
                            .float()
                            .not_null()
                            .default(0.0),
                    )
                    .add_column(
                        ColumnDef::new(MapperAggregateSnapshotsWeekly::AvgStars)
                            .float()
                            .not_null()
                            .default(0.0),
                    )
                    .add_column(
                        ColumnDef::new(MapperAggregateSnapshotsWeekly::AvgBpm)
                            .float()
                            .not_null()
                            .default(0.0),
                    )
                    .add_column(
                        ColumnDef::new(MapperAggregateSnapshotsWeekly::AvgLengthSeconds)
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
enum MapperAggregateSnapshotsWeekly {
    Table,
    AvgRating,
    AvgStars,
    AvgBpm,
    AvgLengthSeconds,
    RatingSum,
    BeatmapCount,
    StarsSum,
    BpmSum,
    LengthSecondsSum,
}
