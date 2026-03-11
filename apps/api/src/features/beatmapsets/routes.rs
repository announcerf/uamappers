use axum::Router;

use crate::app::state::AppState;

use super::http::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/{osu_beatmapset_id}",
            axum::routing::get(handlers::get_beatmapset_details),
        )
        .route(
            "/{osu_beatmapset_id}/charts",
            axum::routing::get(handlers::get_beatmapset_charts),
        )
}
