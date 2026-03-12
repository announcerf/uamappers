use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(BeatmapProfiles::Table)
                    .drop_column(BeatmapProfiles::CreatorId)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(BeatmapsetProfiles::Table)
                    .drop_column(BeatmapsetProfiles::CreatorId)
                    .drop_column(BeatmapsetProfiles::CreatorName)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(BeatmapProfiles::Table)
                    .add_column(
                        ColumnDef::new(BeatmapProfiles::CreatorId)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(BeatmapsetProfiles::Table)
                    .add_column(
                        ColumnDef::new(BeatmapsetProfiles::CreatorId)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .add_column(
                        ColumnDef::new(BeatmapsetProfiles::CreatorName)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum BeatmapProfiles {
    Table,
    CreatorId,
}

#[derive(DeriveIden)]
enum BeatmapsetProfiles {
    Table,
    CreatorId,
    CreatorName,
}
