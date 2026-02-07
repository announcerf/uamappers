use rosu_v2::prelude::RankStatus;

use super::accumulator::MapperAccumulator;

#[test]
fn mapper_accumulator_counts_statuses() {
    let mut acc = MapperAccumulator::default();

    acc.add_status(RankStatus::Graveyard);
    acc.add_status(RankStatus::Pending);
    acc.add_status(RankStatus::WIP);
    acc.add_status(RankStatus::Loved);
    acc.add_status(RankStatus::Ranked);
    acc.add_status(RankStatus::Approved);
    acc.add_status(RankStatus::Qualified);

    assert_eq!(acc.count_total, 7);
    assert_eq!(acc.count_graveyard, 1);
    assert_eq!(acc.count_pending, 2);
    assert_eq!(acc.count_wip, 1);
    assert_eq!(acc.count_loved, 1);
    assert_eq!(acc.count_ranked, 1);
    assert_eq!(acc.count_approved, 1);
}
