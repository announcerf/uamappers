use uamappers_api::entities::{beatmap_profile, beatmapset_profile, osu_user_beatmapset};
use uamappers_api::features::mappers::storage::osu_user_fingerprint::MapperFingerprint;
use uamappers_api::features::mappers::storage::mapper_stats_current_repo::NewMapperStatsCurrentRow;

use super::stats_helpers::{
    count_status, dominant_mode, relevant_mapset_ids, unique_kind_count,
};

pub fn build_mapper_stats_row(
    osu_user_id: i64,
    mapper_fingerprint: Option<&MapperFingerprint>,
    relations: &[osu_user_beatmapset::Model],
    beatmapsets: &[beatmapset_profile::Model],
    beatmaps: &[beatmap_profile::Model],
) -> NewMapperStatsCurrentRow {
    let relation_mapset_ids = relevant_mapset_ids(relations);
    let mapsets: Vec<&beatmapset_profile::Model> = beatmapsets
        .iter()
        .filter(|row| relation_mapset_ids.contains(&row.osu_beatmapset_id))
        .collect();
    let beatmaps: Vec<&beatmap_profile::Model> = beatmaps
        .iter()
        .filter(|row| relation_mapset_ids.contains(&row.osu_beatmapset_id))
        .collect();

    let ranked_mapsets = unique_kind_count(relations, "ranked");
    let guest_mapsets = unique_kind_count(relations, "guest");
    let nominated_mapsets = unique_kind_count(relations, "nominated");
    let total_mapsets = mapsets.len() as i32;
    let loved_mapsets = count_status(&mapsets, "loved");
    let graveyard_mapsets = count_status(&mapsets, "graveyard");
    let pending_mapsets = count_status(&mapsets, "pending")
        + count_status(&mapsets, "qualified")
        + count_status(&mapsets, "wip");
    let total_playcount = mapsets.iter().map(|row| row.playcount).sum();
    let first_submitted_date = mapsets.iter().filter_map(|row| row.submitted_date).min();
    let first_ranked_date = mapsets.iter().filter_map(|row| row.ranked_date).min();
    let last_mapset_updated_at = mapsets.iter().map(|row| row.last_updated).max();
    let main_mode = dominant_mode(&beatmaps);

    NewMapperStatsCurrentRow {
        osu_user_id,
        total_mapsets,
        ranked_mapsets,
        loved_mapsets,
        guest_mapsets,
        nominated_mapsets,
        graveyard_mapsets,
        pending_mapsets,
        total_playcount,
        first_submitted_date,
        first_ranked_date,
        last_mapset_updated_at,
        main_mode,
        mapping_followers: mapper_fingerprint
            .map(|row| row.mapping_followers)
            .unwrap_or(0),
        kudosu_available: mapper_fingerprint
            .map(|row| row.kudosu.available)
            .unwrap_or(0),
        kudosu_total: mapper_fingerprint
            .map(|row| row.kudosu.total)
            .unwrap_or(0),
    }
}
