mod config;
mod error;
mod scanner;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use config::WorkerConfig;
use error::WorkerError;
use scanner::Scanner;

use rosu_v2::prelude::Osu;
use uamappers_api::{
    features::{ingest::storage::repo::ScanStateRepo, mappers::storage::repo::MapperRepo},
    infra::db,
};

#[tokio::main]
async fn main() -> Result<(), WorkerError> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = WorkerConfig::load().map_err(WorkerError::Config)?;

    let db = db::connect(&config.database_url).await?;
    let osu = Osu::new(config.osu_client_id, config.osu_client_secret.clone()).await?;

    let mappers_repo = MapperRepo::new(db.clone());
    let scan_state_repo = ScanStateRepo::new(db.clone());

    let scanner = Scanner::new(osu, config, mappers_repo, scan_state_repo);

    tracing::info!(job = "beatmapset_scan", "starting worker");
    scanner.run().await?;
    tracing::info!(job = "beatmapset_scan", "worker completed");

    Ok(())
}
