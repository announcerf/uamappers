use rosu_v2::prelude::BeatmapsetSearchResult;

#[derive(Debug, Clone)]
pub struct CreatorRef {
    pub osu_user_id: u32,
    pub username: String,
}

pub fn collect_creators(result: &BeatmapsetSearchResult) -> Vec<CreatorRef> {
    let mut refs: Vec<CreatorRef> = Vec::new();
    for mapset in &result.mapsets {
        refs.push(CreatorRef {
            osu_user_id: mapset.creator_id,
            username: mapset.creator_name.to_string(),
        });
    }

    refs.sort_unstable_by_key(|r| r.osu_user_id);
    refs.dedup_by_key(|r| r.osu_user_id);
    refs
}
