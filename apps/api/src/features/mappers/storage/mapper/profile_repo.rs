use chrono::Utc;
use sea_orm::sea_query::OnConflict;
use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};

use crate::entities::mapper_profile;

#[derive(Clone, Debug)]
pub struct NewMapperProfileRow {
    pub osu_user_id: i64,
    pub username: String,
    pub avatar_url: String,
    pub country: String,
    pub country_code: String,
    pub cover_url: String,
    pub primary_mode: String,
    pub join_date: chrono::DateTime<Utc>,
    pub last_visit: Option<chrono::DateTime<Utc>>,
    pub mapping_followers: i32,
    pub kudosu_available: i32,
    pub kudosu_total: i32,
    pub badges_json: sea_orm::JsonValue,
    pub groups_json: sea_orm::JsonValue,
    pub is_bng: bool,
    pub is_nat: bool,
    pub is_gmt: bool,
    pub is_limited_bn: bool,
    pub is_full_bn: bool,
    pub cached_at: chrono::DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct MapperProfileRepo {
    db: DatabaseConnection,
}

impl MapperProfileRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_by_osu_user_id(
        &self,
        osu_user_id: i64,
    ) -> Result<Option<mapper_profile::Model>, DbErr> {
        mapper_profile::Entity::find_by_id(osu_user_id)
            .one(&self.db)
            .await
    }

    pub async fn list_by_osu_user_ids(
        &self,
        ids: &[i64],
    ) -> Result<Vec<mapper_profile::Model>, DbErr> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        mapper_profile::Entity::find()
            .filter(mapper_profile::Column::OsuUserId.is_in(ids.to_vec()))
            .all(&self.db)
            .await
    }

    pub async fn upsert_with<C: ConnectionTrait>(
        &self,
        db: &C,
        row: NewMapperProfileRow,
    ) -> Result<(), DbErr> {
        let active = self.to_active(row);

        mapper_profile::Entity::insert(active)
            .on_conflict(
                OnConflict::column(mapper_profile::Column::OsuUserId)
                    .update_columns([
                        mapper_profile::Column::Username,
                        mapper_profile::Column::AvatarUrl,
                        mapper_profile::Column::Country,
                        mapper_profile::Column::CountryCode,
                        mapper_profile::Column::CoverUrl,
                        mapper_profile::Column::PrimaryMode,
                        mapper_profile::Column::JoinDate,
                        mapper_profile::Column::LastVisit,
                        mapper_profile::Column::MappingFollowers,
                        mapper_profile::Column::KudosuAvailable,
                        mapper_profile::Column::KudosuTotal,
                        mapper_profile::Column::BadgesJson,
                        mapper_profile::Column::GroupsJson,
                        mapper_profile::Column::IsBng,
                        mapper_profile::Column::IsNat,
                        mapper_profile::Column::IsGmt,
                        mapper_profile::Column::IsLimitedBn,
                        mapper_profile::Column::IsFullBn,
                        mapper_profile::Column::CachedAt,
                        mapper_profile::Column::UpdatedAt,
                    ])
                    .to_owned(),
            )
            .exec(db)
            .await?;

        Ok(())
    }

    fn to_active(&self, row: NewMapperProfileRow) -> mapper_profile::ActiveModel {
        let now = Utc::now();

        mapper_profile::ActiveModel {
            osu_user_id: Set(row.osu_user_id),
            username: Set(row.username),
            avatar_url: Set(row.avatar_url),
            country: Set(row.country),
            country_code: Set(row.country_code),
            cover_url: Set(row.cover_url),
            primary_mode: Set(row.primary_mode),
            join_date: Set(row.join_date),
            last_visit: Set(row.last_visit),
            mapping_followers: Set(row.mapping_followers),
            kudosu_available: Set(row.kudosu_available),
            kudosu_total: Set(row.kudosu_total),
            badges_json: Set(row.badges_json),
            groups_json: Set(row.groups_json),
            is_bng: Set(row.is_bng),
            is_nat: Set(row.is_nat),
            is_gmt: Set(row.is_gmt),
            is_limited_bn: Set(row.is_limited_bn),
            is_full_bn: Set(row.is_full_bn),
            cached_at: Set(row.cached_at),
            created_at: Set(now),
            updated_at: Set(now),
        }
    }
}
