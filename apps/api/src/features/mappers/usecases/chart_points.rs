use crate::entities::mapper_aggregate_snapshot_weekly;
use crate::features::mappers::storage::codes::mode_str;

use super::types::MapperChartPoint;

pub fn build_mapper_chart_points(
    points: Vec<mapper_aggregate_snapshot_weekly::Model>,
) -> Vec<MapperChartPoint> {
    points
        .into_iter()
        .map(|point| MapperChartPoint {
            snapshot_week: point.snapshot_week,
            total_mapsets: point.total_mapsets,
            ranked_mapsets: point.ranked_mapsets,
            loved_mapsets: point.loved_mapsets,
            guest_mapsets: point.guest_mapsets,
            nominated_mapsets: point.nominated_mapsets,
            total_playcount: point.total_playcount,
            avg_rating: average(point.rating_sum, point.total_mapsets),
            avg_stars: average(point.stars_sum, point.beatmap_count),
            avg_bpm: average(point.bpm_sum, point.beatmap_count),
            avg_length_seconds: average(point.length_seconds_sum, point.beatmap_count),
            main_mode: mode_str(point.main_mode).to_string(),
        })
        .collect()
}

fn average(sum: f32, count: i32) -> f32 {
    match count {
        0 => 0.0,
        _ => sum / count as f32,
    }
}
