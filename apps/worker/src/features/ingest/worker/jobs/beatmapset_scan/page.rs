use rosu_v2::prelude::BeatmapsetSearchResult;

use super::accumulator::CollectedPage;

use super::Scanner;

impl Scanner {
    pub(crate) fn collect_page(result: &BeatmapsetSearchResult) -> CollectedPage {
        let mut collected = CollectedPage::new();

        for mapset in &result.mapsets {
            let creator_id = mapset.creator_id;
            let entry = collected.accumulators.entry(creator_id).or_default();
            entry.add_status(mapset.status);

            collected
                .mapsets_by_creator
                .entry(creator_id)
                .or_default()
                .push(mapset.clone());
        }

        collected
    }
}
