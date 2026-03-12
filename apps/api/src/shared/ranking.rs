pub fn rank_delta(current_rank: i32, previous_rank: Option<i32>) -> i32 {
    previous_rank.map(|prev| prev - current_rank).unwrap_or(0)
}
