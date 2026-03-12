use crate::features::mappers::storage::codes::status_str;

use super::super::dto::{BeatmapsetChartsPointDto, BeatmapsetChartsResponseDto};

pub fn beatmapset_charts_to_dto(
    charts: crate::features::beatmapsets::usecases::BeatmapsetCharts,
) -> BeatmapsetChartsResponseDto {
    BeatmapsetChartsResponseDto {
        osu_beatmapset_id: charts.osu_beatmapset_id,
        points: charts
            .points
            .into_iter()
            .map(|point| BeatmapsetChartsPointDto {
                snapshot_week: point.snapshot_week,
                status: status_str(point.status).to_string(),
                playcount: point.playcount,
                favourite_count: point.favourite_count,
                rating: point.rating,
                avg_passcount: point.avg_passcount,
                avg_pass_rate: point.avg_pass_rate,
                min_pass_rate: point.min_pass_rate,
                max_pass_rate: point.max_pass_rate,
                last_updated: point.last_updated,
            })
            .collect(),
    }
}
