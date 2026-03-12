use chrono::{Datelike, Duration, TimeZone, Utc, Weekday};
use uamappers_api::features::mappers::storage::{
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
