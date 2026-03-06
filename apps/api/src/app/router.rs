use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    app::state::AppState,
    features::{ingest, mappers, system},
    infra::config::AppConfig,
    openapi::ApiDoc,
};

pub fn build(state: AppState, config: &AppConfig) -> Router {
    let api_router = Router::new()
        .nest("/mappers", mappers::routes::router())
        .nest("/ingest", ingest::routes::router())
        .nest("/system", system::routes::router())
        .with_state(state);

    match config.expose_openapi() {
        true => Router::new()
            .merge(api_router)
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi())),
        false => Router::new().merge(api_router),
    }
}
