use super::super::dto::{
    BeatmapDifficultyDetailDto, BeatmapDifficultyOverviewDto, BeatmapsetAnalyticsDto,
    BeatmapsetDetailsDto, BeatmapsetHeaderDto, BeatmapsetHeadlineStatsDto,
};
use super::charts::beatmapset_charts_to_dto;

pub fn beatmapset_details_to_dto(
    details: crate::features::beatmapsets::usecases::BeatmapsetDetails,
) -> BeatmapsetDetailsDto {
    let headline = build_headline_stats(&details.beatmapset, &details.beatmaps);
    let difficulty_overview = details
        .beatmaps
        .iter()
        .map(|map| BeatmapDifficultyOverviewDto {
            osu_beatmap_id: map.osu_beatmap_id,
            version: map.version.clone(),
            mode: map.mode.clone(),
            stars: map.stars,
            ar: map.ar,
            cs: map.cs,
            od: map.od,
            hp: map.hp,
            bpm: map.bpm,
            seconds_total: map.seconds_total,
            playcount: map.playcount,
            passcount: map.passcount,
            pass_rate: pass_rate(map.passcount, map.playcount),
        })
        .collect();
    let difficulty_details = details
        .beatmaps
        .iter()
        .map(|map| BeatmapDifficultyDetailDto {
            osu_beatmap_id: map.osu_beatmap_id,
            version: map.version.clone(),
            mode: map.mode.clone(),
            stars: map.stars,
            status: map.status.clone(),
            ar: map.ar,
            cs: map.cs,
            od: map.od,
            hp: map.hp,
            bpm: map.bpm,
            seconds_total: map.seconds_total,
            seconds_drain: map.seconds_drain,
            max_combo: map.max_combo,
            playcount: map.playcount,
            passcount: map.passcount,
            pass_rate: pass_rate(map.passcount, map.playcount),
            objects: map.count_circles + map.count_sliders + map.count_spinners,
            owners: map.owners_json.clone(),
            count_circles: map.count_circles,
            count_sliders: map.count_sliders,
            count_spinners: map.count_spinners,
            last_updated: map.last_updated,
        })
        .collect();

    BeatmapsetDetailsDto {
        beatmapset: BeatmapsetHeaderDto {
            osu_beatmapset_id: details.beatmapset.osu_beatmapset_id,
            title: details.beatmapset.title,
            title_unicode: details.beatmapset.title_unicode,
            artist: details.beatmapset.artist,
            artist_unicode: details.beatmapset.artist_unicode,
            creator_name: details.beatmapset.creator_name,
            status: details.beatmapset.status,
            genre: details.beatmapset.genre,
            language: details.beatmapset.language,
            source: details.beatmapset.source,
            tags: details.beatmapset.tags,
            cover_url: details.beatmapset.cover_url,
            card_url: details.beatmapset.card_url,
            preview_url: details.beatmapset.preview_url,
            submitted_date: details.beatmapset.submitted_date,
            ranked_date: details.beatmapset.ranked_date,
            last_updated: details.beatmapset.last_updated,
            nsfw: details.beatmapset.nsfw,
            video: details.beatmapset.video,
            storyboard: details.beatmapset.storyboard,
            spotlight: details.beatmapset.spotlight,
        },
        headline_stats: headline,
        difficulty_overview,
        difficulty_details,
        analytics: BeatmapsetAnalyticsDto {
            charts: beatmapset_charts_to_dto(
                crate::features::beatmapsets::usecases::BeatmapsetCharts {
                    osu_beatmapset_id: details.beatmapset.osu_beatmapset_id,
                    points: details.charts,
                },
            ),
        },
    }
}

fn build_headline_stats(
    beatmapset: &crate::entities::beatmapset_profile::Model,
    beatmaps: &[crate::entities::beatmap_profile::Model],
) -> BeatmapsetHeadlineStatsDto {
    let avg_pass_rate = if beatmaps.is_empty() {
        0.0
    } else {
        beatmaps
            .iter()
            .map(|map| pass_rate(map.passcount, map.playcount))
            .sum::<f32>()
            / beatmaps.len() as f32
    };
    let avg_stars = if beatmaps.is_empty() {
        0.0
    } else {
        beatmaps.iter().map(|map| map.stars).sum::<f32>() / beatmaps.len() as f32
    };

    BeatmapsetHeadlineStatsDto {
        playcount: beatmapset.playcount,
        favourite_count: beatmapset.favourite_count,
        rating: beatmapset.rating,
        fav_play_ratio: pass_rate(beatmapset.favourite_count, beatmapset.playcount),
        difficulty_count: beatmapset.difficulty_count,
        avg_pass_rate,
        avg_stars,
    }
}

fn pass_rate(passcount: i64, playcount: i64) -> f32 {
    if playcount == 0 {
        0.0
    } else {
        passcount as f32 / playcount as f32
    }
}
