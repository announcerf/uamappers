use axum::Router;

use crate::app::state::AppState;

use super::http::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", axum::routing::get(handlers::list_mappers))
        .route("/search", axum::routing::get(handlers::search_mappers))
        .route("/:osu_user_id", axum::routing::get(handlers::get_mapper))
}
