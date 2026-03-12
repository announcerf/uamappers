pub mod m20260306_000001_create_scan_and_ua_mappers;
pub mod m20260306_000002_create_osu_users_and_beatmapsets;
pub mod m20260306_000003_create_osu_user_beatmapsets;
pub mod m20260311_000004_create_mapper_profiles;
pub mod m20260311_000005_create_beatmapset_profiles;
pub mod m20260311_000006_create_beatmap_profiles;
pub mod m20260311_000007_create_mapper_stats_current;
pub mod m20260311_000008_create_leaderboard_positions_current;
pub mod m20260311_000009_create_mapper_aggregate_snapshots_weekly;
pub mod m20260311_000010_create_beatmapset_snapshots_weekly;
pub mod m20260312_000011_strip_page_from_osu_users_raw;
pub mod m20260312_000012_add_kudosu_available_to_mapper_stats_current;
pub mod m20260312_000013_drop_mapper_profiles;
pub mod m20260312_000014_reduce_mapper_stats_current;
pub mod m20260312_000015_reduce_leaderboard_positions_current;
pub mod m20260312_000016_reduce_beatmapset_profiles;
pub mod m20260312_000017_optimize_mapper_snapshots_weekly;
pub mod m20260312_000018_optimize_beatmapset_snapshots_weekly;
pub mod m20260312_000019_shrink_beatmapsets_raw;
pub mod m20260312_000020_create_beatmapset_extras;
pub mod m20260312_000021_drop_beatmapsets;
pub mod m20260312_000022_remove_creator_fields_from_profiles;
pub mod m20260312_000023_add_hot_read_indexes;
pub mod m20260312_000024_compact_storage_codes;
pub mod m20260312_000025_drop_redundant_created_at;
use sea_orm_migration::prelude::*;

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260306_000001_create_scan_and_ua_mappers::Migration),
            Box::new(m20260306_000002_create_osu_users_and_beatmapsets::Migration),
            Box::new(m20260306_000003_create_osu_user_beatmapsets::Migration),
            Box::new(m20260311_000004_create_mapper_profiles::Migration),
            Box::new(m20260311_000005_create_beatmapset_profiles::Migration),
            Box::new(m20260311_000006_create_beatmap_profiles::Migration),
            Box::new(m20260311_000007_create_mapper_stats_current::Migration),
            Box::new(m20260311_000008_create_leaderboard_positions_current::Migration),
            Box::new(m20260311_000009_create_mapper_aggregate_snapshots_weekly::Migration),
            Box::new(m20260311_000010_create_beatmapset_snapshots_weekly::Migration),
            Box::new(m20260312_000011_strip_page_from_osu_users_raw::Migration),
            Box::new(m20260312_000012_add_kudosu_available_to_mapper_stats_current::Migration),
            Box::new(m20260312_000013_drop_mapper_profiles::Migration),
            Box::new(m20260312_000014_reduce_mapper_stats_current::Migration),
            Box::new(m20260312_000015_reduce_leaderboard_positions_current::Migration),
            Box::new(m20260312_000016_reduce_beatmapset_profiles::Migration),
            Box::new(m20260312_000017_optimize_mapper_snapshots_weekly::Migration),
            Box::new(m20260312_000018_optimize_beatmapset_snapshots_weekly::Migration),
            Box::new(m20260312_000019_shrink_beatmapsets_raw::Migration),
            Box::new(m20260312_000020_create_beatmapset_extras::Migration),
            Box::new(m20260312_000021_drop_beatmapsets::Migration),
            Box::new(m20260312_000022_remove_creator_fields_from_profiles::Migration),
            Box::new(m20260312_000023_add_hot_read_indexes::Migration),
            Box::new(m20260312_000024_compact_storage_codes::Migration),
            Box::new(m20260312_000025_drop_redundant_created_at::Migration),
        ]
    }
}
