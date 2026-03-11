use sea_orm::entity::prelude::*;
use sea_orm::JsonValue;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "mapper_profiles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub osu_user_id: i64,
    pub username: String,
    pub avatar_url: String,
    pub country: String,
    pub country_code: String,
    pub cover_url: String,
    pub primary_mode: String,
    pub join_date: DateTimeUtc,
    pub last_visit: Option<DateTimeUtc>,
    pub mapping_followers: i32,
    pub kudosu_available: i32,
    pub kudosu_total: i32,
    pub badges_json: JsonValue,
    pub groups_json: JsonValue,
    pub is_bng: bool,
    pub is_nat: bool,
    pub is_gmt: bool,
    pub is_limited_bn: bool,
    pub is_full_bn: bool,
    pub cached_at: DateTimeUtc,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
