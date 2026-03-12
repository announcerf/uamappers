use chrono_tz::Europe::Kyiv;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

struct KyivTimeMillis;

impl FormatTime for KyivTimeMillis {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        let ts = chrono::Utc::now()
            .with_timezone(&Kyiv)
            .format("%Y-%m-%d %H:%M:%S%.3f %Z")
            .to_string();
        w.write_str(&ts)
    }
}

pub fn init_tracing() {
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
                .with_timer(KyivTimeMillis)
                .with_target(false)
                .with_file(false)
                .with_line_number(false)
                .with_thread_names(false)
                .with_thread_ids(false)
                .with_ansi(true),
        )
        .init();
}
