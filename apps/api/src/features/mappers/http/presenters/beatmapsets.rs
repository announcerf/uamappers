use crate::entities::beatmapset;
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

fn beatmapset_to_dto(model: beatmapset::Model) -> BeatmapsetDto {
    BeatmapsetDto {
        osu_beatmapset_id: model.osu_beatmapset_id,
        osu_last_updated: model.last_updated,
        cached_at: model.updated_at,
        raw: model.raw,
    }
}
