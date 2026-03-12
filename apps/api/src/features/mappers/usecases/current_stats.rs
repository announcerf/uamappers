use crate::entities::{beatmap_profile, beatmapset_profile, mapper_stats_current};
use crate::features::mappers::storage::codes::mode_str;

use super::types::{MapperCurrentStats, MapperKudosu};

pub fn build_mapper_current_stats(
    model: mapper_stats_current::Model,
    beatmapsets: &[beatmapset_profile::Model],
    beatmaps: &[beatmap_profile::Model],
) -> MapperCurrentStats {
    MapperCurrentStats {
        total_mapsets: model.total_mapsets,
        ranked_mapsets: model.ranked_mapsets,
        loved_mapsets: model.loved_mapsets,
        guest_mapsets: model.guest_mapsets,
        nominated_mapsets: model.nominated_mapsets,
        graveyard_mapsets: model.graveyard_mapsets,
        pending_mapsets: model.pending_mapsets,
        total_playcount: model.total_playcount,
        avg_rating: average_f32(beatmapsets.iter().map(|row| row.rating)),
        weighted_rating: weighted_rating(beatmapsets),
        avg_stars: average_f32(beatmaps.iter().map(|row| row.stars)),
        min_stars: min_f32(beatmaps.iter().map(|row| row.stars)),
        max_stars: max_f32(beatmaps.iter().map(|row| row.stars)),
        avg_bpm: average_f32(beatmaps.iter().map(|row| row.bpm)),
        avg_length_seconds: average_f32(beatmaps.iter().map(|row| row.seconds_total as f32)),
        avg_ar: average_f32(beatmaps.iter().map(|row| row.ar)),
        avg_cs: average_f32(beatmaps.iter().map(|row| row.cs)),
        avg_od: average_f32(beatmaps.iter().map(|row| row.od)),
        avg_hp: average_f32(beatmaps.iter().map(|row| row.hp)),
        first_submitted_date: model.first_submitted_date,
        first_ranked_date: model.first_ranked_date,
        last_mapset_updated_at: model.last_mapset_updated_at,
        main_mode: mode_str(model.main_mode).to_string(),
        mapping_followers: model.mapping_followers,
        kudosu: MapperKudosu {
            total: model.kudosu_total,
            available: model.kudosu_available,
        },
        has_ranked: model.ranked_mapsets > 0,
        has_loved: model.loved_mapsets > 0,
        has_guest: model.guest_mapsets > 0,
        has_nominated: model.nominated_mapsets > 0,
        updated_at: model.updated_at,
    }
}

pub fn rank_delta(current_rank: i32, previous_rank: Option<i32>) -> i32 {
    previous_rank.map(|prev| prev - current_rank).unwrap_or(0)
}

fn average_f32(values: impl Iterator<Item = f32>) -> f32 {
    let mut total = 0.0f32;
    let mut count = 0u32;

    for value in values {
        total += value;
        count += 1;
    }

    match count {
        0 => 0.0,
        _ => total / count as f32,
    }
}

fn min_f32(values: impl Iterator<Item = f32>) -> f32 {
    values.reduce(f32::min).unwrap_or(0.0)
}

fn max_f32(values: impl Iterator<Item = f32>) -> f32 {
    values.reduce(f32::max).unwrap_or(0.0)
}

fn weighted_rating(rows: &[beatmapset_profile::Model]) -> f32 {
    let weighted_sum: f32 = rows
        .iter()
        .map(|row| row.rating * row.playcount as f32)
        .sum();
    let total_weight: i64 = rows.iter().map(|row| row.playcount).sum();

    match total_weight {
        0 => 0.0,
        _ => weighted_sum / total_weight as f32,
    }
}
