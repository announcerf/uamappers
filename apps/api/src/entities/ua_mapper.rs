use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "ua_mappers")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub osu_user_id: i64,
    pub username: String,
    pub country_code: String,
    pub first_seen_at: DateTimeUtc,
    pub last_seen_at: DateTimeUtc,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
