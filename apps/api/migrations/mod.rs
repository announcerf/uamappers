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
        ]
    }
}
