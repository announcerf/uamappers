pub const DEFAULT_LIMIT: u64 = 40;
pub const MAX_LIMIT: u64 = 200;

pub fn clamp_limit(limit: Option<u64>) -> u64 {
    let limit = limit.unwrap_or(DEFAULT_LIMIT);
    limit.clamp(1, MAX_LIMIT)
}
