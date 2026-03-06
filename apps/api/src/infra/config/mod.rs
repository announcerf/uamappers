#[derive(Clone, Debug)]
pub struct AppConfig {
    pub api_bind: String,
    pub database_url: String,
    pub env: AppEnv,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AppEnv {
    Dev,
    Prod,
}

impl AppConfig {
    pub fn load() -> Result<Self, String> {
        let api_bind = std::env::var("API_BIND").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
        let database_url = crate::shared::db_url::build_database_url()?;
        let env = AppEnv::load();

        Ok(Self {
            api_bind,
            database_url,
            env,
        })
    }

    pub fn expose_openapi(&self) -> bool {
        matches!(self.env, AppEnv::Dev)
    }
}

impl AppEnv {
    fn load() -> Self {
        match std::env::var("UAMAPPERS_ENV") {
            Ok(value) => match value.to_ascii_lowercase().as_str() {
                "prod" => Self::Prod,
                _ => Self::Dev,
            },
            Err(_) => Self::Dev,
        }
    }
}
