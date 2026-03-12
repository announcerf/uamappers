use chrono::{TimeZone, Utc};
use rosu_v2::model::beatmap::BeatmapsetExtended;
use uamappers_api::features::mappers::storage::codes::status_code;
use uamappers_api::features::mappers::storage::beatmapset_snapshot_weekly_repo::NewBeatmapsetSnapshotWeeklyRow;

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
    let beatmap_count = mapset
        .maps
        .as_ref()
        .map(|maps| maps.len() as i32)
        .unwrap_or(0);
    let passcount_sum = mapset
        .maps
        .as_ref()
        .map(|maps| maps.iter().map(|map| map.passcount as i64).sum())
        .unwrap_or(0);

    NewBeatmapsetSnapshotWeeklyRow {
        osu_beatmapset_id: mapset.mapset_id as i64,
        snapshot_week,
        status: status_code(rank_status_to_str(mapset.status)),
        playcount: mapset.playcount as i64,
        favourite_count: mapset.favourite_count as i64,
        rating: mapset.rating,
        beatmap_count,
        passcount_sum,
        pass_rate_sum: pass_rates.iter().copied().sum(),
        min_pass_rate: pass_rates.iter().copied().reduce(f32::min).unwrap_or(0.0),
        max_pass_rate: pass_rates.iter().copied().reduce(f32::max).unwrap_or(0.0),
        last_updated: offset_to_utc(mapset.last_updated),
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
