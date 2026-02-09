use rosu_v2::prelude::Osu;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uamappers_api::{
    features::{
        ingest::storage::repo::ScanStateRepo,
        mappers::storage::{
            beatmapset_repo::BeatmapsetRepo, osu_user_beatmapset_repo::OsuUserBeatmapsetRepo,
            osu_user_repo::OsuUserRepo, ua_mapper_repo::UaMapperRepo,
        },
    },
    infra::db,
};

use crate::{
    features::ingest::osu_client::OsuClient,
    features::ingest::worker::jobs::{
        mapper_discovery::MapperDiscovery, mapper_enrich::MapperEnrich,
    },
    infra::config::WorkerConfig,
    shared::errors::WorkerError,
};

fn init_tracing() {
    let mut filter = match tracing_subscriber::EnvFilter::try_from_default_env() {
        Ok(filter) => filter,
        Err(_) => tracing_subscriber::EnvFilter::new("info"),
    };

    let rust_log = std::env::var("RUST_LOG").unwrap_or_default();
    if !rust_log.contains("sqlx=") {
        filter = filter.add_directive("sqlx=warn".parse().expect("sqlx filter directive"));
    }
    if !rust_log.contains("sea_orm=") {
        filter = filter.add_directive("sea_orm=warn".parse().expect("sea_orm filter directive"));
    }

    tracing_subscriber::registry()
        .with(filter)
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_target(false)
                .with_file(false)
                .with_line_number(false)
                .with_thread_names(false)
                .with_thread_ids(false)
                .with_ansi(false)
                .without_time(),
        )
        .init();
}

pub async fn run() -> Result<(), WorkerError> {
    init_tracing();

    let config = WorkerConfig::load().map_err(WorkerError::Config)?;

    let db = db::connect(&config.database_url).await?;
    let osu = Osu::new(config.osu_client_id, config.osu_client_secret.clone()).await?;
    let osu_client = OsuClient::new(osu);
    let osu_client_stats = osu_client.clone();
    osu_client.min_request_interval_ms();

    tracing::info!(
        "start d{} u{} b{}",
        u8::from(config.run_discovery),
        u8::from(config.enrich_users),
        u8::from(config.enrich_beatmapsets)
    );

    let ua_mappers_repo = UaMapperRepo::new(db.clone());
    let osu_users_repo = OsuUserRepo::new(db.clone());
    let beatmapsets_repo = BeatmapsetRepo::new(db.clone());
    let scan_state_repo = ScanStateRepo::new(db.clone());
    let osu_user_beatmapsets_repo = OsuUserBeatmapsetRepo::new(db.clone());

    let discovery = MapperDiscovery::new(
        osu_client.clone(),
        config.clone(),
        ua_mappers_repo.clone(),
        scan_state_repo.clone(),
    );

    let enrich = MapperEnrich::new(
        osu_client,
        config.clone(),
        ua_mappers_repo,
        osu_users_repo,
        beatmapsets_repo,
        osu_user_beatmapsets_repo,
        scan_state_repo,
    );

    let started_at = std::time::Instant::now();

    if config.run_discovery {
        discovery.run().await?;
    } else {
        tracing::info!("discovery disabled");
    }

    if config.enrich_users || config.enrich_beatmapsets {
        enrich.run().await?;
    } else {
        tracing::info!("enrich disabled");
    }

    let elapsed = started_at.elapsed();
    let _ = crate::shared::time::format_duration(elapsed);
    let throttle = osu_client_stats.throttle_snapshot().await;
    let _ = throttle.total_sleep_ms;
    let stats = osu_client_stats.stats_snapshot().await;
    tracing::info!(
        "done {}s req{} r{}",
        elapsed.as_secs(),
        throttle.acquires,
        stats.retries
    );

    Ok(())
}
