use crate::features::mappers::storage::codes::{genre_str, language_str, status_str};
use crate::features::mappers::usecases::BeatmapsetPage;

use super::super::dto::{BeatmapsetDto, BeatmapsetListResponse};

pub fn beatmapset_page_to_dto(page: BeatmapsetPage) -> BeatmapsetListResponse {
    let items = page.items.into_iter().map(beatmapset_to_dto).collect();

    BeatmapsetListResponse {
        items,
        limit: page.limit,
        offset: page.offset,
        total: page.total,
    }
}

fn beatmapset_to_dto(model: crate::features::mappers::usecases::BeatmapsetListItem) -> BeatmapsetDto {
    let creator_name = model
        .extra
        .as_ref()
        .map(|row| row.creator_name.clone())
        .unwrap_or_default();

    BeatmapsetDto {
        osu_beatmapset_id: model.profile.osu_beatmapset_id,
        creator_name,
        artist: model.profile.artist,
        artist_unicode: model.profile.artist_unicode,
        title: model.profile.title,
        title_unicode: model.profile.title_unicode,
        status: status_str(model.profile.status).to_string(),
        genre: genre_str(model.profile.genre).map(str::to_string),
        language: language_str(model.profile.language).map(str::to_string),
        source: model.profile.source,
        tags: model.profile.tags,
        cover_url: model.profile.cover_url,
        card_url: model.profile.card_url,
        submitted_date: model.profile.submitted_date,
        ranked_date: model.profile.ranked_date,
        osu_last_updated: model.profile.last_updated,
        playcount: model.profile.playcount,
        favourite_count: model.profile.favourite_count,
        rating: model.profile.rating,
        video: model.profile.video,
        storyboard: model.profile.storyboard,
        nsfw: model.profile.nsfw,
        spotlight: model.profile.spotlight,
        bpm: model.profile.bpm,
        cached_at: model.profile.cached_at,
    }
}
