use sea_orm::entity::prelude::*;
use sea_orm::JsonValue;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "beatmap_profiles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub osu_beatmap_id: i64,
    pub osu_beatmapset_id: i64,
    pub creator_id: i64,
    pub version: String,
    pub mode: String,
    pub stars: f32,
    pub ar: f32,
    pub cs: f32,
    pub od: f32,
    pub hp: f32,
    pub bpm: f32,
    pub seconds_total: i32,
    pub seconds_drain: i32,
    pub max_combo: Option<i32>,
    pub playcount: i64,
    pub passcount: i64,
    pub count_circles: i32,
    pub count_sliders: i32,
    pub count_spinners: i32,
    pub owners_json: JsonValue,
    pub status: String,
    pub is_scoreable: bool,
    pub last_updated: DateTimeUtc,
    pub cached_at: DateTimeUtc,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
