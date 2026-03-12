use sea_orm::entity::prelude::*;
use sea_orm::JsonValue;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "beatmapset_extras")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub osu_beatmapset_id: i64,
    pub creator_id: i64,
    pub creator_name: String,
    pub ratings_json: JsonValue,
    pub anime_cover: Option<String>,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
