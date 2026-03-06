use uamappers_api::features::mappers::http::handlers::common::{
    DEFAULT_LIMIT, MAX_LIMIT, clamp_limit,
};

#[test]
fn clamp_limit_defaults_to_default_limit() {
    assert_eq!(clamp_limit(None), DEFAULT_LIMIT);
}

#[test]
fn clamp_limit_clamps_low_values_to_1() {
    assert_eq!(clamp_limit(Some(0)), 1);
}

#[test]
fn clamp_limit_clamps_high_values_to_max_limit() {
    assert_eq!(clamp_limit(Some(MAX_LIMIT + 1)), MAX_LIMIT);
}
