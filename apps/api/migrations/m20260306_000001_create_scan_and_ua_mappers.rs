use sea_orm::ConnectionTrait;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ScanState::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ScanState::Name)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ScanState::Cursor).string())
                    .col(ColumnDef::new(ScanState::LastSuccessAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(ScanState::LastErrorAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(ScanState::RetryCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(ScanState::NextRetryAt).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(ScanState::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UaMappers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UaMappers::OsuUserId)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UaMappers::Username).string().not_null())
                    .col(
                        ColumnDef::new(UaMappers::CountryCode)
                            .char_len(2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UaMappers::FirstSeenAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(UaMappers::LastSeenAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(UaMappers::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(UaMappers::UpdatedAt)
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
                    .name("idx_ua_mappers_country_code")
                    .table(UaMappers::Table)
                    .if_not_exists()
                    .col(UaMappers::CountryCode)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_ua_mappers_updated_at")
                    .table(UaMappers::Table)
                    .if_not_exists()
                    .col(UaMappers::UpdatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "CREATE INDEX IF NOT EXISTS idx_ua_mappers_username_lower ON ua_mappers (LOWER(username));",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UaMappers::Table).if_exists().to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ScanState::Table).if_exists().to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ScanState {
    Table,
    Name,
    Cursor,
    LastSuccessAt,
    LastErrorAt,
    RetryCount,
    NextRetryAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum UaMappers {
    Table,
    OsuUserId,
    Username,
    CountryCode,
    FirstSeenAt,
    LastSeenAt,
    CreatedAt,
    UpdatedAt,
}
