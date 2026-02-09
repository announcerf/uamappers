use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use chrono::Utc;
use rosu_v2::prelude::Osu;
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
    shared::time::format_duration,
};

fn init_tracing() {
    let mut filter = match tracing_subscriber::EnvFilter::try_from_default_env() {
        Ok(filter) => filter,
        Err(_) => tracing_subscriber::EnvFilter::new("info"),
    };

    let rust_log = std::env::var("RUST_LOG").unwrap_or_default();
    if !rust_log.contains("sqlx=") {
        filter = filter
            .add_directive("sqlx=warn".parse().expect("sqlx filter directive"));
    }
    if !rust_log.contains("sea_orm=") {
        filter = filter
            .add_directive("sea_orm=warn".parse().expect("sea_orm filter directive"));
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

    let run_id = format!("{}-{}", Utc::now().timestamp_millis(), std::process::id());
    let span = tracing::info_span!("worker_run", run_id = %run_id);
    let _enter = span.enter();

    let db = db::connect(&config.database_url).await?;
    let osu = Osu::new(config.osu_client_id, config.osu_client_secret.clone()).await?;
    let osu_client = OsuClient::new(osu);
    let osu_client_stats = osu_client.clone();

    tracing::info!(
        run_discovery = config.run_discovery,
        enrich_users = config.enrich_users,
        enrich_beatmapsets = config.enrich_beatmapsets,
        scan_page_delay_ms = config.page_delay_ms,
        progress_log_every = config.progress_log_every,
        osu_min_request_interval_ms = osu_client.min_request_interval_ms(),
        "worker config loaded"
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
        tracing::info!(job = "mapper_discovery", "starting worker (discovery)");
        discovery.run().await?;
        tracing::info!(job = "mapper_discovery", "worker completed (discovery)");
    } else {
        tracing::info!(
            job = "mapper_discovery",
            "discovery disabled (RUN_DISCOVERY=0)"
        );
    }

    if config.enrich_users || config.enrich_beatmapsets {
        tracing::info!(job = "mapper_enrich", "starting worker (enrich)");
        enrich.run().await?;
        tracing::info!(job = "mapper_enrich", "worker completed (enrich)");
    } else {
        tracing::info!(
            job = "mapper_enrich",
            "enrich disabled (ENRICH_USERS=0 and ENRICH_BEATMAPSETS=0)"
        );
    }

    let elapsed = started_at.elapsed();
    let throttle = osu_client_stats.throttle_snapshot().await;
    let stats = osu_client_stats.stats_snapshot().await;
    tracing::info!(
        elapsed_ms = elapsed.as_millis() as u64,
        elapsed = %format_duration(elapsed),
        osu_requests = throttle.acquires,
        osu_retries = stats.retries,
        osu_throttle_sleep_ms = throttle.total_sleep_ms,
        "worker finished"
    );

    Ok(())
}
