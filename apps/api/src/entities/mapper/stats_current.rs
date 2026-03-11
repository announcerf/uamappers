use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "mapper_stats_current")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub osu_user_id: i64,
    pub total_mapsets: i32,
    pub ranked_mapsets: i32,
    pub loved_mapsets: i32,
    pub guest_mapsets: i32,
    pub nominated_mapsets: i32,
    pub graveyard_mapsets: i32,
    pub pending_mapsets: i32,
    pub total_playcount: i64,
    pub avg_rating: f32,
    pub weighted_rating: f32,
    pub avg_stars: f32,
    pub min_stars: f32,
    pub max_stars: f32,
    pub avg_bpm: f32,
    pub avg_length_seconds: f32,
    pub avg_ar: f32,
    pub avg_cs: f32,
    pub avg_od: f32,
    pub avg_hp: f32,
    pub first_submitted_date: Option<DateTimeUtc>,
    pub first_ranked_date: Option<DateTimeUtc>,
    pub last_mapset_updated_at: Option<DateTimeUtc>,
    pub main_mode: String,
    pub mapping_followers: i32,
    pub kudosu_total: i32,
    pub has_ranked: bool,
    pub has_loved: bool,
    pub has_guest: bool,
    pub has_nominated: bool,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
