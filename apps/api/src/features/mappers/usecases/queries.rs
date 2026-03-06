use sea_orm::DbErr;

use crate::features::mappers::storage::{
    osu_user_beatmapset_repo::OsuUserBeatmapsetRepo, osu_user_repo::OsuUserRepo,
    ua_mapper_repo::UaMapperRepo,
};

use super::types::{BeatmapsetPage, MapperPage, MapperProfile, PageInput};

pub async fn list_mappers(
    ua_mappers_repo: &UaMapperRepo,
    page: PageInput,
) -> Result<MapperPage, DbErr> {
    let (items, total) = ua_mappers_repo.list(page.limit, page.offset).await?;

    Ok(MapperPage {
        items,
        limit: page.limit,
        offset: page.offset,
        total,
    })
}

pub async fn search_mappers(
    ua_mappers_repo: &UaMapperRepo,
    query: &str,
    page: PageInput,
) -> Result<MapperPage, DbErr> {
    let (items, total) = ua_mappers_repo
        .search(query, page.limit, page.offset)
        .await?;

    Ok(MapperPage {
        items,
        limit: page.limit,
        offset: page.offset,
        total,
    })
}

pub async fn get_mapper_profile_by_username(
    ua_mappers_repo: &UaMapperRepo,
    osu_users_repo: &OsuUserRepo,
    username: &str,
) -> Result<Option<MapperProfile>, DbErr> {
    let mapper = ua_mappers_repo.get_by_username(username).await?;

    load_mapper_profile(osu_users_repo, mapper).await
}

pub async fn get_mapper_profile_by_id(
    ua_mappers_repo: &UaMapperRepo,
    osu_users_repo: &OsuUserRepo,
    osu_user_id: i64,
) -> Result<Option<MapperProfile>, DbErr> {
    let mapper = ua_mappers_repo.get_by_osu_user_id(osu_user_id).await?;

    load_mapper_profile(osu_users_repo, mapper).await
}

pub async fn list_mapper_beatmapsets_by_username(
    ua_mappers_repo: &UaMapperRepo,
    osu_user_beatmapsets_repo: &OsuUserBeatmapsetRepo,
    username: &str,
    kind: &str,
    page: PageInput,
) -> Result<Option<BeatmapsetPage>, DbErr> {
    let mapper = ua_mappers_repo.get_by_username(username).await?;

    let Some(mapper) = mapper else {
        return Ok(None);
    };

    let beatmapsets =
        list_mapper_beatmapsets_by_id(osu_user_beatmapsets_repo, mapper.osu_user_id, kind, page)
            .await?;

    Ok(Some(beatmapsets))
}

pub async fn list_mapper_beatmapsets_by_id(
    osu_user_beatmapsets_repo: &OsuUserBeatmapsetRepo,
    osu_user_id: i64,
    kind: &str,
    page: PageInput,
) -> Result<BeatmapsetPage, DbErr> {
    let (items, total) = osu_user_beatmapsets_repo
        .list_beatmapsets(osu_user_id, kind, page.limit, page.offset)
        .await?;

    Ok(BeatmapsetPage {
        items,
        limit: page.limit,
        offset: page.offset,
        total,
    })
}

async fn load_mapper_profile(
    osu_users_repo: &OsuUserRepo,
    mapper: Option<crate::entities::ua_mapper::Model>,
) -> Result<Option<MapperProfile>, DbErr> {
    let Some(mapper) = mapper else {
        return Ok(None);
    };

    let user_row = osu_users_repo
        .get_by_osu_user_id(mapper.osu_user_id)
        .await?;
    let (user_raw, user_fetched_at) = match user_row {
        Some(row) => (Some(row.raw), Some(row.fetched_at)),
        None => (None, None),
    };

    Ok(Some(MapperProfile {
        mapper,
        user_fetched_at,
        user_raw,
    }))
}
