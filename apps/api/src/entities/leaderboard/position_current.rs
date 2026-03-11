use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "leaderboard_positions_current")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub leaderboard_key: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub osu_user_id: i64,
    pub current_rank: i32,
    pub previous_rank: Option<i32>,
    pub rank_delta: i32,
    pub measured_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
