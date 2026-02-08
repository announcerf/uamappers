#[derive(Clone, Debug)]
pub struct AppConfig {
    pub api_bind: String,
    pub database_url: String,
}

impl AppConfig {
    pub fn load() -> Self {
        let api_bind = std::env::var("API_BIND").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
        let database_url = crate::shared::db_url::build_database_url();

        Self {
            api_bind,
            database_url,
        }
    }
}
