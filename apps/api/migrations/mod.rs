pub mod m20260306_000001_create_scan_and_ua_mappers;
pub mod m20260306_000002_create_osu_users_and_beatmapsets;
pub mod m20260306_000003_create_osu_user_beatmapsets;
use sea_orm_migration::prelude::*;

pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260306_000001_create_scan_and_ua_mappers::Migration),
            Box::new(m20260306_000002_create_osu_users_and_beatmapsets::Migration),
            Box::new(m20260306_000003_create_osu_user_beatmapsets::Migration),
        ]
    }
}
