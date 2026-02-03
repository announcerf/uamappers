use axum::Router;

use crate::app::state::AppState;

use super::http::handlers;

pub fn router() -> Router<AppState> {
    Router::new().route("/status", axum::routing::get(handlers::get_status))
}
