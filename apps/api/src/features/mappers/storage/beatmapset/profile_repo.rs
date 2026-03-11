use chrono::Utc;
use sea_orm::sea_query::OnConflict;
use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};

use crate::entities::beatmapset_profile;

#[derive(Clone, Debug)]
pub struct NewBeatmapsetProfileRow {
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
    pub submitted_date: Option<chrono::DateTime<Utc>>,
    pub ranked_date: Option<chrono::DateTime<Utc>>,
    pub last_updated: chrono::DateTime<Utc>,
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
    pub cached_at: chrono::DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct BeatmapsetProfileRepo {
    db: DatabaseConnection,
}

impl BeatmapsetProfileRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    pub async fn get_by_osu_beatmapset_id(
        &self,
        osu_beatmapset_id: i64,
    ) -> Result<Option<beatmapset_profile::Model>, DbErr> {
        beatmapset_profile::Entity::find_by_id(osu_beatmapset_id)
            .one(&self.db)
            .await
    }

    pub async fn list_by_osu_beatmapset_ids(
        &self,
        ids: &[i64],
    ) -> Result<Vec<beatmapset_profile::Model>, DbErr> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        beatmapset_profile::Entity::find()
            .filter(beatmapset_profile::Column::OsuBeatmapsetId.is_in(ids.to_vec()))
            .all(&self.db)
            .await
    }

    pub async fn upsert_many_with<C: ConnectionTrait>(
        &self,
        db: &C,
        rows: Vec<NewBeatmapsetProfileRow>,
    ) -> Result<(), DbErr> {
        if rows.is_empty() {
            return Ok(());
        }

        for row in rows {
            beatmapset_profile::Entity::insert(self.to_active(row))
                .on_conflict(
                    OnConflict::column(beatmapset_profile::Column::OsuBeatmapsetId)
                        .update_columns([
                            beatmapset_profile::Column::CreatorId,
                            beatmapset_profile::Column::CreatorName,
                            beatmapset_profile::Column::Artist,
                            beatmapset_profile::Column::ArtistUnicode,
                            beatmapset_profile::Column::Title,
                            beatmapset_profile::Column::TitleUnicode,
                            beatmapset_profile::Column::Source,
                            beatmapset_profile::Column::Tags,
                            beatmapset_profile::Column::Genre,
                            beatmapset_profile::Column::Language,
                            beatmapset_profile::Column::Status,
                            beatmapset_profile::Column::SubmittedDate,
                            beatmapset_profile::Column::RankedDate,
                            beatmapset_profile::Column::LastUpdated,
                            beatmapset_profile::Column::DiscussionEnabled,
                            beatmapset_profile::Column::DiscussionLocked,
                            beatmapset_profile::Column::CanBeHyped,
                            beatmapset_profile::Column::IsScoreable,
                            beatmapset_profile::Column::DownloadDisabled,
                            beatmapset_profile::Column::Nsfw,
                            beatmapset_profile::Column::Video,
                            beatmapset_profile::Column::Storyboard,
                            beatmapset_profile::Column::Spotlight,
                            beatmapset_profile::Column::Playcount,
                            beatmapset_profile::Column::FavouriteCount,
                            beatmapset_profile::Column::Rating,
                            beatmapset_profile::Column::HypeCurrent,
                            beatmapset_profile::Column::HypeRequired,
                            beatmapset_profile::Column::NominationsCurrent,
                            beatmapset_profile::Column::CoverUrl,
                            beatmapset_profile::Column::CardUrl,
                            beatmapset_profile::Column::PreviewUrl,
                            beatmapset_profile::Column::Bpm,
                            beatmapset_profile::Column::DifficultyCount,
                            beatmapset_profile::Column::CachedAt,
                            beatmapset_profile::Column::UpdatedAt,
                        ])
                        .to_owned(),
                )
                .exec(db)
                .await?;
        }

        Ok(())
    }

    fn to_active(&self, row: NewBeatmapsetProfileRow) -> beatmapset_profile::ActiveModel {
        let now = Utc::now();

        beatmapset_profile::ActiveModel {
            osu_beatmapset_id: Set(row.osu_beatmapset_id),
            creator_id: Set(row.creator_id),
            creator_name: Set(row.creator_name),
            artist: Set(row.artist),
            artist_unicode: Set(row.artist_unicode),
            title: Set(row.title),
            title_unicode: Set(row.title_unicode),
            source: Set(row.source),
            tags: Set(row.tags),
            genre: Set(row.genre),
            language: Set(row.language),
            status: Set(row.status),
            submitted_date: Set(row.submitted_date),
            ranked_date: Set(row.ranked_date),
            last_updated: Set(row.last_updated),
            discussion_enabled: Set(row.discussion_enabled),
            discussion_locked: Set(row.discussion_locked),
            can_be_hyped: Set(row.can_be_hyped),
            is_scoreable: Set(row.is_scoreable),
            download_disabled: Set(row.download_disabled),
            nsfw: Set(row.nsfw),
            video: Set(row.video),
            storyboard: Set(row.storyboard),
            spotlight: Set(row.spotlight),
            playcount: Set(row.playcount),
            favourite_count: Set(row.favourite_count),
            rating: Set(row.rating),
            hype_current: Set(row.hype_current),
            hype_required: Set(row.hype_required),
            nominations_current: Set(row.nominations_current),
            cover_url: Set(row.cover_url),
            card_url: Set(row.card_url),
            preview_url: Set(row.preview_url),
            bpm: Set(row.bpm),
            difficulty_count: Set(row.difficulty_count),
            cached_at: Set(row.cached_at),
            created_at: Set(now),
            updated_at: Set(now),
        }
    }
}
