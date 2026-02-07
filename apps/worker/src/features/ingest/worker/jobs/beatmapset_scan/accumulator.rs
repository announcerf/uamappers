use std::collections::HashMap;

use rosu_v2::model::beatmap::BeatmapsetExtended;
use rosu_v2::prelude::RankStatus;

#[derive(Default, Debug, Clone)]
pub struct MapperAccumulator {
    pub count_graveyard: i32,
    pub count_pending: i32,
    pub count_wip: i32,
    pub count_loved: i32,
    pub count_ranked: i32,
    pub count_approved: i32,
    pub count_total: i32,
}

impl MapperAccumulator {
    pub fn add_status(&mut self, status: RankStatus) {
        self.count_total += 1;
        match status {
            RankStatus::Graveyard => self.count_graveyard += 1,
            RankStatus::Pending => self.count_pending += 1,
            RankStatus::WIP => self.count_wip += 1,
            RankStatus::Loved => self.count_loved += 1,
            RankStatus::Ranked => self.count_ranked += 1,
            RankStatus::Approved => self.count_approved += 1,
            RankStatus::Qualified => self.count_pending += 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CollectedPage {
    pub accumulators: HashMap<u32, MapperAccumulator>,
    pub mapsets_by_creator: HashMap<u32, Vec<BeatmapsetExtended>>,
}

impl CollectedPage {
    pub fn new() -> Self {
        Self {
            accumulators: HashMap::new(),
            mapsets_by_creator: HashMap::new(),
        }
    }
}
