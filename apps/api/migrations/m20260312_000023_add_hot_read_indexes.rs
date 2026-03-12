use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .name("idx_osu_user_beatmapsets_user_kind_updated")
                    .table(OsuUserBeatmapsets::Table)
                    .if_not_exists()
                    .col(OsuUserBeatmapsets::OsuUserId)
                    .col(OsuUserBeatmapsets::Kind)
                    .col(OsuUserBeatmapsets::UpdatedAt)
                    .col(OsuUserBeatmapsets::OsuBeatmapsetId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_beatmap_profiles_osu_beatmapset_id")
                    .table(BeatmapProfiles::Table)
                    .if_not_exists()
                    .col(BeatmapProfiles::OsuBeatmapsetId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_beatmap_profiles_osu_beatmapset_id")
                    .table(BeatmapProfiles::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_osu_user_beatmapsets_user_kind_updated")
                    .table(OsuUserBeatmapsets::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum OsuUserBeatmapsets {
    Table,
    OsuUserId,
    Kind,
    UpdatedAt,
    OsuBeatmapsetId,
}

#[derive(DeriveIden)]
enum BeatmapProfiles {
    Table,
    OsuBeatmapsetId,
}
