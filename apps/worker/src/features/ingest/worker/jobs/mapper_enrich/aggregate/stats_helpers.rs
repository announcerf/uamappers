use std::collections::{HashMap, HashSet};

use uamappers_api::entities::{beatmap_profile, beatmapset_profile, osu_user_beatmapset};
use uamappers_api::features::mappers::storage::codes::{kind_code, mode_str, status_code};

pub fn relevant_mapset_ids(relations: &[osu_user_beatmapset::Model]) -> HashSet<i64> {
    relations.iter().map(|row| row.osu_beatmapset_id).collect()
}

pub fn unique_kind_count(relations: &[osu_user_beatmapset::Model], kind: &str) -> i32 {
    let kind = kind_code(kind);

    relations
        .iter()
        .filter(|row| row.kind == kind)
        .map(|row| row.osu_beatmapset_id)
        .collect::<HashSet<_>>()
        .len() as i32
}

pub fn count_status(rows: &[&beatmapset_profile::Model], status: &str) -> i32 {
    let status = status_code(status);

    rows.iter().filter(|row| row.status == status).count() as i32
}

pub fn dominant_mode(rows: &[&beatmap_profile::Model]) -> i16 {
    let mut modes: HashMap<i16, (usize, i64)> = HashMap::new();

    for row in rows {
        let entry = modes.entry(row.mode).or_insert((0, 0));
        entry.0 += 1;
        entry.1 += row.playcount;
    }

    modes
        .into_iter()
        .max_by(|(left_mode, left), (right_mode, right)| {
            left.0
                .cmp(&right.0)
                .then(left.1.cmp(&right.1))
                .then(mode_str(*left_mode).cmp(mode_str(*right_mode)))
        })
        .map(|(mode, _)| mode)
        .unwrap_or_default()
}
