use chrono::Utc;
use sea_orm::sea_query::OnConflict;
use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, Set,
};

use crate::entities::beatmapset;

#[derive(Clone, Debug)]
pub struct NewBeatmapsetRow {
    pub osu_beatmapset_id: i64,
    pub creator_osu_user_id: i64,
    pub creator_username: String,
    pub status: String,
    pub artist: String,
    pub title: String,
    pub artist_unicode: Option<String>,
    pub title_unicode: Option<String>,
    pub submitted_date: Option<chrono::DateTime<Utc>>,
    pub ranked_date: Option<chrono::DateTime<Utc>>,
    pub last_updated: chrono::DateTime<Utc>,
    pub play_count: i32,
    pub favourite_count: i32,
    pub raw: sea_orm::JsonValue,
}

#[derive(Clone, Debug)]
pub struct BeatmapsetRepo {
    db: DatabaseConnection,
}

impl BeatmapsetRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    pub async fn upsert_many_with<C: ConnectionTrait>(
        &self,
        db: &C,
        rows: Vec<beatmapset::ActiveModel>,
    ) -> Result<(), DbErr> {
        if rows.is_empty() {
            return Ok(());
        }

        for row in rows {
            beatmapset::Entity::insert(row)
                .on_conflict(
                    OnConflict::column(beatmapset::Column::OsuBeatmapsetId)
                        .update_columns([
                            beatmapset::Column::CreatorOsuUserId,
                            beatmapset::Column::CreatorUsername,
                            beatmapset::Column::Status,
                            beatmapset::Column::Artist,
                            beatmapset::Column::Title,
                            beatmapset::Column::ArtistUnicode,
                            beatmapset::Column::TitleUnicode,
                            beatmapset::Column::SubmittedDate,
                            beatmapset::Column::RankedDate,
                            beatmapset::Column::LastUpdated,
                            beatmapset::Column::PlayCount,
                            beatmapset::Column::FavouriteCount,
                            beatmapset::Column::Raw,
                            beatmapset::Column::UpdatedAt,
                        ])
                        .to_owned(),
                )
                .exec(db)
                .await?;
        }

        Ok(())
    }

    pub fn to_active(&self, row: NewBeatmapsetRow) -> beatmapset::ActiveModel {
        let now = Utc::now();
        beatmapset::ActiveModel {
            osu_beatmapset_id: Set(row.osu_beatmapset_id),
            creator_osu_user_id: Set(row.creator_osu_user_id),
            creator_username: Set(row.creator_username),
            status: Set(row.status),
            artist: Set(row.artist),
            title: Set(row.title),
            artist_unicode: Set(row.artist_unicode),
            title_unicode: Set(row.title_unicode),
            submitted_date: Set(row.submitted_date),
            ranked_date: Set(row.ranked_date),
            last_updated: Set(row.last_updated),
            play_count: Set(row.play_count),
            favourite_count: Set(row.favourite_count),
            raw: Set(row.raw),
            created_at: Set(now),
            updated_at: Set(now),
        }
    }

    pub async fn list_by_creator(
        &self,
        creator_osu_user_id: i64,
        limit: u64,
        offset: u64,
    ) -> Result<(Vec<beatmapset::Model>, u64), DbErr> {
        let total = beatmapset::Entity::find()
            .filter(beatmapset::Column::CreatorOsuUserId.eq(creator_osu_user_id))
            .count(&self.db)
            .await?;

        let rows = beatmapset::Entity::find()
            .filter(beatmapset::Column::CreatorOsuUserId.eq(creator_osu_user_id))
            .order_by_desc(beatmapset::Column::LastUpdated)
            .order_by_desc(beatmapset::Column::OsuBeatmapsetId)
            .limit(limit)
            .offset(offset)
            .all(&self.db)
            .await?;

        Ok((rows, total))
    }
}
