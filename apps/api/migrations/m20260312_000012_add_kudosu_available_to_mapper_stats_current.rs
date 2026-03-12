use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(MapperStatsCurrent::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(MapperStatsCurrent::KudosuAvailable)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(MapperStatsCurrent::Table)
                    .drop_column(MapperStatsCurrent::KudosuAvailable)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum MapperStatsCurrent {
    Table,
    KudosuAvailable,
}
