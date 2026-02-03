use axum::Router;

use crate::app::state::AppState;

use super::http::handlers;

pub fn router() -> Router<AppState> {
    Router::new().route("/health", axum::routing::get(handlers::health))
}
