use chrono::{Datelike, Duration, TimeZone, Utc, Weekday};
use rosu_v2::model::beatmap::BeatmapsetExtended;
use uamappers_api::features::mappers::storage::{
    beatmapset_snapshot_weekly_repo::NewBeatmapsetSnapshotWeeklyRow,
    mapper_aggregate_snapshot_weekly_repo::NewMapperAggregateSnapshotWeeklyRow,
    mapper_stats_current_repo::NewMapperStatsCurrentRow,
};

pub fn snapshot_week(now: chrono::DateTime<Utc>) -> chrono::DateTime<Utc> {
    let days_from_monday = match now.weekday() {
        Weekday::Mon => 0,
        Weekday::Tue => 1,
        Weekday::Wed => 2,
        Weekday::Thu => 3,
        Weekday::Fri => 4,
        Weekday::Sat => 5,
        Weekday::Sun => 6,
    };
    let start = now.date_naive() - Duration::days(days_from_monday);

    Utc.from_utc_datetime(
        &start
            .and_hms_opt(0, 0, 0)
            .expect("valid start of week datetime"),
    )
}

pub fn mapper_stats_row_to_snapshot_row(
    stats: &NewMapperStatsCurrentRow,
    snapshot_week: chrono::DateTime<Utc>,
) -> NewMapperAggregateSnapshotWeeklyRow {
    NewMapperAggregateSnapshotWeeklyRow {
        osu_user_id: stats.osu_user_id,
        snapshot_week,
        total_mapsets: stats.total_mapsets,
        ranked_mapsets: stats.ranked_mapsets,
        loved_mapsets: stats.loved_mapsets,
        guest_mapsets: stats.guest_mapsets,
        nominated_mapsets: stats.nominated_mapsets,
        graveyard_mapsets: stats.graveyard_mapsets,
        pending_mapsets: stats.pending_mapsets,
        total_playcount: stats.total_playcount,
        avg_rating: stats.avg_rating,
        avg_stars: stats.avg_stars,
        avg_bpm: stats.avg_bpm,
        avg_length_seconds: stats.avg_length_seconds,
        main_mode: stats.main_mode.clone(),
    }
}

pub fn mapset_to_snapshot_row(
    mapset: &BeatmapsetExtended,
    snapshot_week: chrono::DateTime<Utc>,
) -> NewBeatmapsetSnapshotWeeklyRow {
    let pass_rates = mapset
        .maps
        .as_ref()
        .map(|maps| {
            maps.iter()
                .map(|map| {
                    if map.playcount == 0 {
                        0.0
                    } else {
                        map.passcount as f32 / map.playcount as f32
                    }
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let avg_passcount = mapset
        .maps
        .as_ref()
        .map(|maps| maps.iter().map(|map| map.passcount as f32).sum::<f32>() / maps.len() as f32)
        .unwrap_or(0.0);
    let avg_pass_rate = average(pass_rates.iter().copied());
    let min_pass_rate = pass_rates.iter().copied().reduce(f32::min).unwrap_or(0.0);
    let max_pass_rate = pass_rates.iter().copied().reduce(f32::max).unwrap_or(0.0);

    NewBeatmapsetSnapshotWeeklyRow {
        osu_beatmapset_id: mapset.mapset_id as i64,
        snapshot_week,
        status: rank_status_to_str(mapset.status).to_string(),
        playcount: mapset.playcount as i64,
        favourite_count: mapset.favourite_count as i64,
        rating: mapset.rating,
        avg_passcount,
        avg_pass_rate,
        min_pass_rate,
        max_pass_rate,
        last_updated: offset_to_utc(mapset.last_updated),
    }
}

fn average(values: impl Iterator<Item = f32>) -> f32 {
    let mut total = 0.0f32;
    let mut count = 0u32;

    for value in values {
        total += value;
        count += 1;
    }

    if count == 0 {
        0.0
    } else {
        total / count as f32
    }
}

fn rank_status_to_str(status: rosu_v2::model::beatmap::RankStatus) -> &'static str {
    match status {
        rosu_v2::model::beatmap::RankStatus::Graveyard => "graveyard",
        rosu_v2::model::beatmap::RankStatus::WIP => "wip",
        rosu_v2::model::beatmap::RankStatus::Pending => "pending",
        rosu_v2::model::beatmap::RankStatus::Ranked => "ranked",
        rosu_v2::model::beatmap::RankStatus::Approved => "approved",
        rosu_v2::model::beatmap::RankStatus::Qualified => "qualified",
        rosu_v2::model::beatmap::RankStatus::Loved => "loved",
    }
}

fn offset_to_utc(dt: time::OffsetDateTime) -> chrono::DateTime<Utc> {
    let secs = dt.unix_timestamp();
    let nanos = dt.nanosecond();

    Utc.timestamp_opt(secs, nanos)
        .single()
        .unwrap_or_else(|| Utc.timestamp_opt(secs, 0).single().unwrap())
}
