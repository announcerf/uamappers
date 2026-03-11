use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "beatmapset_profiles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub osu_beatmapset_id: i64,
    pub creator_id: i64,
    pub creator_name: String,
    pub artist: String,
    pub artist_unicode: Option<String>,
    pub title: String,
    pub title_unicode: Option<String>,
    pub source: String,
    pub tags: String,
    pub genre: Option<String>,
    pub language: Option<String>,
    pub status: String,
    pub submitted_date: Option<DateTimeUtc>,
    pub ranked_date: Option<DateTimeUtc>,
    pub last_updated: DateTimeUtc,
    pub discussion_enabled: bool,
    pub discussion_locked: bool,
    pub can_be_hyped: bool,
    pub is_scoreable: bool,
    pub download_disabled: bool,
    pub nsfw: bool,
    pub video: bool,
    pub storyboard: bool,
    pub spotlight: bool,
    pub playcount: i64,
    pub favourite_count: i64,
    pub rating: f32,
    pub hype_current: i32,
    pub hype_required: i32,
    pub nominations_current: i32,
    pub cover_url: String,
    pub card_url: String,
    pub preview_url: String,
    pub bpm: f32,
    pub difficulty_count: i32,
    pub cached_at: DateTimeUtc,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
