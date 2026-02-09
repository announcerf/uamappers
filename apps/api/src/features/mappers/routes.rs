use axum::Router;

use crate::app::state::AppState;

use super::http::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("", axum::routing::get(handlers::list_mappers))
        .route("/search", axum::routing::get(handlers::search_mappers))
        .route(
            "/by-id/{osu_user_id}",
            axum::routing::get(handlers::get_mapper_by_id),
        )
        .route(
            "/by-id/{osu_user_id}/beatmapsets",
            axum::routing::get(handlers::list_mapper_beatmapsets_by_id),
        )
        .route(
            "/{user}/beatmapsets",
            axum::routing::get(handlers::list_mapper_beatmapsets),
        )
        .route("/{user}", axum::routing::get(handlers::get_mapper))
}
