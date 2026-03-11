use uamappers_api::entities::{
    beatmap_profile, beatmapset_profile, mapper_profile, osu_user_beatmapset,
};
use uamappers_api::features::mappers::storage::mapper_stats_current_repo::NewMapperStatsCurrentRow;

use super::stats_helpers::{
    average_f32, count_status, dominant_mode, max_f32, min_f32, relevant_mapset_ids,
    unique_kind_count, weighted_rating,
};

pub fn build_mapper_stats_row(
    osu_user_id: i64,
    mapper_profile: Option<&mapper_profile::Model>,
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
    let avg_rating = average_f32(mapsets.iter().map(|row| row.rating));
    let weighted_rating = weighted_rating(&mapsets);
    let avg_stars = average_f32(beatmaps.iter().map(|row| row.stars));
    let min_stars = min_f32(beatmaps.iter().map(|row| row.stars));
    let max_stars = max_f32(beatmaps.iter().map(|row| row.stars));
    let avg_bpm = average_f32(beatmaps.iter().map(|row| row.bpm));
    let avg_length_seconds = average_f32(beatmaps.iter().map(|row| row.seconds_total as f32));
    let avg_ar = average_f32(beatmaps.iter().map(|row| row.ar));
    let avg_cs = average_f32(beatmaps.iter().map(|row| row.cs));
    let avg_od = average_f32(beatmaps.iter().map(|row| row.od));
    let avg_hp = average_f32(beatmaps.iter().map(|row| row.hp));
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
        avg_rating,
        weighted_rating,
        avg_stars,
        min_stars,
        max_stars,
        avg_bpm,
        avg_length_seconds,
        avg_ar,
        avg_cs,
        avg_od,
        avg_hp,
        first_submitted_date,
        first_ranked_date,
        last_mapset_updated_at,
        main_mode,
        mapping_followers: mapper_profile.map(|row| row.mapping_followers).unwrap_or(0),
        kudosu_total: mapper_profile.map(|row| row.kudosu_total).unwrap_or(0),
        has_ranked: ranked_mapsets > 0,
        has_loved: loved_mapsets > 0,
        has_guest: guest_mapsets > 0,
        has_nominated: nominated_mapsets > 0,
    }
}
