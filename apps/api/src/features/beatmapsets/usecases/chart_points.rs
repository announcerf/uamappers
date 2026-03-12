use crate::entities::beatmapset_snapshot_weekly;

use super::types::BeatmapsetChartPoint;

pub fn build_beatmapset_chart_points(
    points: Vec<beatmapset_snapshot_weekly::Model>,
) -> Vec<BeatmapsetChartPoint> {
    points
        .into_iter()
        .map(|point| BeatmapsetChartPoint {
            snapshot_week: point.snapshot_week,
            status: point.status,
            playcount: point.playcount,
            favourite_count: point.favourite_count,
            rating: point.rating,
            avg_passcount: average(point.passcount_sum as f32, point.beatmap_count),
            avg_pass_rate: average(point.pass_rate_sum, point.beatmap_count),
            min_pass_rate: point.min_pass_rate,
            max_pass_rate: point.max_pass_rate,
            last_updated: point.last_updated,
        })
        .collect()
}

fn average(sum: f32, count: i32) -> f32 {
    match count {
        0 => 0.0,
        _ => sum / count as f32,
    }
}
