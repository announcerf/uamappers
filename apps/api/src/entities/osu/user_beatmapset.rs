use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "osu_user_beatmapsets")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub osu_user_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub kind: i16,
    #[sea_orm(primary_key, auto_increment = false)]
    pub osu_beatmapset_id: i64,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::beatmapset_profile::Entity",
        from = "Column::OsuBeatmapsetId",
        to = "super::beatmapset_profile::Column::OsuBeatmapsetId"
    )]
    Beatmapset,
}

impl Related<super::beatmapset_profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Beatmapset.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
