use std::collections::{HashMap, HashSet};

use uamappers_api::entities::{beatmap_profile, beatmapset_profile, osu_user_beatmapset};

pub fn relevant_mapset_ids(relations: &[osu_user_beatmapset::Model]) -> HashSet<i64> {
    relations.iter().map(|row| row.osu_beatmapset_id).collect()
}

pub fn unique_kind_count(relations: &[osu_user_beatmapset::Model], kind: &str) -> i32 {
    relations
        .iter()
        .filter(|row| row.kind == kind)
        .map(|row| row.osu_beatmapset_id)
        .collect::<HashSet<_>>()
        .len() as i32
}

pub fn count_status(rows: &[&beatmapset_profile::Model], status: &str) -> i32 {
    rows.iter().filter(|row| row.status == status).count() as i32
}

pub fn average_f32(values: impl Iterator<Item = f32>) -> f32 {
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

pub fn min_f32(values: impl Iterator<Item = f32>) -> f32 {
    values.reduce(f32::min).unwrap_or(0.0)
}

pub fn max_f32(values: impl Iterator<Item = f32>) -> f32 {
    values.reduce(f32::max).unwrap_or(0.0)
}

pub fn weighted_rating(rows: &[&beatmapset_profile::Model]) -> f32 {
    let weighted_sum: f32 = rows
        .iter()
        .map(|row| row.rating * row.playcount as f32)
        .sum();
    let total_weight: i64 = rows.iter().map(|row| row.playcount).sum();

    if total_weight == 0 {
        0.0
    } else {
        weighted_sum / total_weight as f32
    }
}

pub fn dominant_mode(rows: &[&beatmap_profile::Model]) -> String {
    let mut modes: HashMap<&str, (usize, i64)> = HashMap::new();

    for row in rows {
        let entry = modes.entry(row.mode.as_str()).or_insert((0, 0));
        entry.0 += 1;
        entry.1 += row.playcount;
    }

    modes
        .into_iter()
        .max_by(|(left_mode, left), (right_mode, right)| {
            left.0
                .cmp(&right.0)
                .then(left.1.cmp(&right.1))
                .then(left_mode.cmp(right_mode))
        })
        .map(|(mode, _)| mode.to_string())
        .unwrap_or_else(|| "unknown".to_string())
}
