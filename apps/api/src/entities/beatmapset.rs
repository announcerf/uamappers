use sea_orm::entity::prelude::*;
use sea_orm::JsonValue;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "beatmapsets")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub osu_beatmapset_id: i64,
    pub creator_osu_user_id: i64,
    pub creator_username: String,
    pub status: String,
    pub artist: String,
    pub title: String,
    pub artist_unicode: Option<String>,
    pub title_unicode: Option<String>,
    pub submitted_date: Option<DateTimeUtc>,
    pub ranked_date: Option<DateTimeUtc>,
    pub last_updated: DateTimeUtc,
    pub play_count: i32,
    pub favourite_count: i32,
    pub raw: JsonValue,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
