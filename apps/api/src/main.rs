use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use uamappers_api::{
    app::{router, state::AppState},
    infra::{config::AppConfig, db},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = AppConfig::load();
    let db = db::connect(&config.database_url).await?;
    let state = AppState::new(db);

    let app = router::build(state);
    let listener = tokio::net::TcpListener::bind(&config.api_bind).await?;

    tracing::info!(bind = %config.api_bind, "starting API server");

    axum::serve(listener, app).await?;

    Ok(())
}
