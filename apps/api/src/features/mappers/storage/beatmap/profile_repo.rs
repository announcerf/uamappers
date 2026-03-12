use chrono::Utc;
use sea_orm::sea_query::OnConflict;
use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder,
    Set,
};

use crate::entities::beatmap_profile;

#[derive(Clone, Debug)]
pub struct NewBeatmapProfileRow {
    pub osu_beatmap_id: i64,
    pub osu_beatmapset_id: i64,
    pub version: String,
    pub mode: i16,
    pub stars: f32,
    pub ar: f32,
    pub cs: f32,
    pub od: f32,
    pub hp: f32,
    pub bpm: f32,
    pub seconds_total: i32,
    pub seconds_drain: i32,
    pub max_combo: Option<i32>,
    pub playcount: i64,
    pub passcount: i64,
    pub count_circles: i32,
    pub count_sliders: i32,
    pub count_spinners: i32,
    pub owners_json: sea_orm::JsonValue,
    pub status: i16,
    pub last_updated: chrono::DateTime<Utc>,
    pub cached_at: chrono::DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct BeatmapProfileRepo {
    db: DatabaseConnection,
}

impl BeatmapProfileRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    pub async fn list_by_osu_beatmapset_ids(
        &self,
        ids: &[i64],
    ) -> Result<Vec<beatmap_profile::Model>, DbErr> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        beatmap_profile::Entity::find()
            .filter(beatmap_profile::Column::OsuBeatmapsetId.is_in(ids.to_vec()))
            .all(&self.db)
            .await
    }

    pub async fn list_by_osu_beatmapset_id(
        &self,
        osu_beatmapset_id: i64,
    ) -> Result<Vec<beatmap_profile::Model>, DbErr> {
        beatmap_profile::Entity::find()
            .filter(beatmap_profile::Column::OsuBeatmapsetId.eq(osu_beatmapset_id))
            .order_by_desc(beatmap_profile::Column::Stars)
            .order_by_asc(beatmap_profile::Column::OsuBeatmapId)
            .all(&self.db)
            .await
    }

    pub async fn upsert_many_with<C: ConnectionTrait>(
        &self,
        db: &C,
        rows: Vec<NewBeatmapProfileRow>,
    ) -> Result<(), DbErr> {
        if rows.is_empty() {
            return Ok(());
        }

        for row in rows {
            beatmap_profile::Entity::insert(self.to_active(row))
                .on_conflict(
                    OnConflict::column(beatmap_profile::Column::OsuBeatmapId)
                        .update_columns([
                            beatmap_profile::Column::OsuBeatmapsetId,
                            beatmap_profile::Column::Version,
                            beatmap_profile::Column::Mode,
                            beatmap_profile::Column::Stars,
                            beatmap_profile::Column::Ar,
                            beatmap_profile::Column::Cs,
                            beatmap_profile::Column::Od,
                            beatmap_profile::Column::Hp,
                            beatmap_profile::Column::Bpm,
                            beatmap_profile::Column::SecondsTotal,
                            beatmap_profile::Column::SecondsDrain,
                            beatmap_profile::Column::MaxCombo,
                            beatmap_profile::Column::Playcount,
                            beatmap_profile::Column::Passcount,
                            beatmap_profile::Column::CountCircles,
                            beatmap_profile::Column::CountSliders,
                            beatmap_profile::Column::CountSpinners,
                            beatmap_profile::Column::OwnersJson,
                            beatmap_profile::Column::Status,
                            beatmap_profile::Column::LastUpdated,
                            beatmap_profile::Column::CachedAt,
                            beatmap_profile::Column::UpdatedAt,
                        ])
                        .to_owned(),
                )
                .exec(db)
                .await?;
        }

        Ok(())
    }

    pub async fn delete_missing_for_mapset_with<C: ConnectionTrait>(
        &self,
        db: &C,
        osu_beatmapset_id: i64,
        keep_ids: &[i64],
    ) -> Result<(), DbErr> {
        let query = beatmap_profile::Entity::delete_many()
            .filter(beatmap_profile::Column::OsuBeatmapsetId.eq(osu_beatmapset_id));

        let query = if keep_ids.is_empty() {
            query
        } else {
            query.filter(beatmap_profile::Column::OsuBeatmapId.is_not_in(keep_ids.to_vec()))
        };

        query.exec(db).await?;
        Ok(())
    }

    fn to_active(&self, row: NewBeatmapProfileRow) -> beatmap_profile::ActiveModel {
        let now = Utc::now();

        beatmap_profile::ActiveModel {
            osu_beatmap_id: Set(row.osu_beatmap_id),
            osu_beatmapset_id: Set(row.osu_beatmapset_id),
            version: Set(row.version),
            mode: Set(row.mode),
            stars: Set(row.stars),
            ar: Set(row.ar),
            cs: Set(row.cs),
            od: Set(row.od),
            hp: Set(row.hp),
            bpm: Set(row.bpm),
            seconds_total: Set(row.seconds_total),
            seconds_drain: Set(row.seconds_drain),
            max_combo: Set(row.max_combo),
            playcount: Set(row.playcount),
            passcount: Set(row.passcount),
            count_circles: Set(row.count_circles),
            count_sliders: Set(row.count_sliders),
            count_spinners: Set(row.count_spinners),
            owners_json: Set(row.owners_json),
            status: Set(row.status),
            last_updated: Set(row.last_updated),
            cached_at: Set(row.cached_at),
            updated_at: Set(now),
        }
    }
}
