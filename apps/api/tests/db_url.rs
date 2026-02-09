use std::sync::{Mutex, OnceLock};

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn with_env_lock<T>(f: impl FnOnce() -> T) -> T {
    let lock = ENV_LOCK.get_or_init(|| Mutex::new(()));
    let _guard = lock.lock().expect("env lock poisoned");
    f()
}

fn set_var(name: &str, value: Option<&str>) {
    match value {
        Some(v) => unsafe {
            std::env::set_var(name, v);
        },
        None => unsafe {
            std::env::remove_var(name);
        },
    }
}

#[test]
fn build_database_url_uses_defaults_when_env_missing() {
    with_env_lock(|| {
        let prev = [
            ("POSTGRES_HOST", std::env::var("POSTGRES_HOST").ok()),
            ("POSTGRES_PORT", std::env::var("POSTGRES_PORT").ok()),
            ("POSTGRES_USER", std::env::var("POSTGRES_USER").ok()),
            ("POSTGRES_PASSWORD", std::env::var("POSTGRES_PASSWORD").ok()),
            ("POSTGRES_DB", std::env::var("POSTGRES_DB").ok()),
        ];

        set_var("POSTGRES_HOST", None);
        set_var("POSTGRES_PORT", None);
        set_var("POSTGRES_USER", None);
        set_var("POSTGRES_PASSWORD", None);
        set_var("POSTGRES_DB", None);

        let url = uamappers_api::shared::db_url::build_database_url();
        assert_eq!(url, "postgres://postgres:postgres@localhost:5432/uamappers");

        for (k, v) in prev {
            set_var(k, v.as_deref());
        }
    });
}

#[test]
fn build_database_url_uses_env_values() {
    with_env_lock(|| {
        let prev = [
            ("POSTGRES_HOST", std::env::var("POSTGRES_HOST").ok()),
            ("POSTGRES_PORT", std::env::var("POSTGRES_PORT").ok()),
            ("POSTGRES_USER", std::env::var("POSTGRES_USER").ok()),
            ("POSTGRES_PASSWORD", std::env::var("POSTGRES_PASSWORD").ok()),
            ("POSTGRES_DB", std::env::var("POSTGRES_DB").ok()),
        ];

        set_var("POSTGRES_HOST", Some("postgres"));
        set_var("POSTGRES_PORT", Some("6432"));
        set_var("POSTGRES_USER", Some("app"));
        set_var("POSTGRES_PASSWORD", Some("secret"));
        set_var("POSTGRES_DB", Some("db"));

        let url = uamappers_api::shared::db_url::build_database_url();
        assert_eq!(url, "postgres://app:secret@postgres:6432/db");

        for (k, v) in prev {
            set_var(k, v.as_deref());
        }
    });
}
