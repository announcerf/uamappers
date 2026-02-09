use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    app::openapi::ApiDoc,
    app::state::AppState,
    features::{ingest, mappers, system},
};

pub fn build(state: AppState) -> Router {
    let api_router = Router::new()
        .route(
            "/mappers",
            axum::routing::get(mappers::http::handlers::list_mappers),
        )
        .nest("/mappers", mappers::routes::router())
        .nest("/ingest", ingest::routes::router())
        .nest("/system", system::routes::router())
        .with_state(state);

    Router::new()
        .merge(api_router)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
