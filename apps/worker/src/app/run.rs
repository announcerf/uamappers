use crate::{
    app::bootstrap::{WorkerRuntime, build_runtime},
    app::tracing::init_tracing,
    infra::config::WorkerConfig,
    shared::{errors::WorkerError, time::format_duration},
};

pub async fn run() -> Result<(), WorkerError> {
    init_tracing();

    let config = WorkerConfig::load().map_err(WorkerError::Config)?;
    let runtime = build_runtime(config).await?;
    let started_at = std::time::Instant::now();

    log_start(&runtime);
    run_jobs(&runtime).await?;
    log_finish(&runtime, started_at).await;

    Ok(())
}

fn log_start(runtime: &WorkerRuntime) {
    runtime.osu_client_stats.min_request_interval_ms();

    tracing::info!(
        "start disc={} users={} beatmaps={}",
        if runtime.config.run_discovery {
            "on"
        } else {
            "off"
        },
        if runtime.config.enrich_users {
            "on"
        } else {
            "off"
        },
        if runtime.config.enrich_beatmapsets {
            "on"
        } else {
            "off"
        }
    );
}

async fn run_jobs(runtime: &WorkerRuntime) -> Result<(), WorkerError> {
    if runtime.config.run_discovery {
        runtime.discovery.run().await?;
    } else {
        tracing::info!("discovery disabled");
    }

    if runtime.config.enrich_users || runtime.config.enrich_beatmapsets {
        runtime.enrich.run().await?;
    } else {
        tracing::info!("enrich disabled");
    }

    Ok(())
}

async fn log_finish(runtime: &WorkerRuntime, started_at: std::time::Instant) {
    let elapsed = started_at.elapsed();
    let throttle = runtime.osu_client_stats.throttle_snapshot().await;
    let stats = runtime.osu_client_stats.stats_snapshot().await;

    tracing::info!(
        duration = %format_duration(elapsed),
        requests = throttle.acquires,
        throttle_sleep_ms = throttle.total_sleep_ms,
        retries = stats.retries,
        "done"
    );
}
