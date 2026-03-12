use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "beatmapset_snapshots_weekly")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub osu_beatmapset_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub snapshot_week: DateTimeUtc,
    pub status: i16,
    pub playcount: i64,
    pub favourite_count: i64,
    pub rating: f32,
    pub beatmap_count: i32,
    pub passcount_sum: i64,
    pub pass_rate_sum: f32,
    pub min_pass_rate: f32,
    pub max_pass_rate: f32,
    pub last_updated: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
