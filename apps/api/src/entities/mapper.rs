use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "mappers")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub osu_user_id: i64,
    pub username: String,
    pub country_code: String,
    pub count_graveyard: i32,
    pub count_pending: i32,
    pub count_wip: i32,
    pub count_loved: i32,
    pub count_ranked: i32,
    pub count_approved: i32,
    pub count_total: i32,
    pub is_bn: bool,
    pub nominated_count: Option<i32>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
