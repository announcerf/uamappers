#[derive(Clone, Debug)]
pub struct WorkerConfig {
    pub database_url: String,
    pub osu_client_id: u64,
    pub osu_client_secret: String,
    pub page_delay_ms: u64,
    pub max_pages: Option<u32>,
    pub batch_size: usize,
}

impl WorkerConfig {
    pub fn load() -> Result<Self, String> {
        let database_url = std::env::var("DATABASE_URL")
            .ok()
            .filter(|value| !value.is_empty())
            .unwrap_or_else(uamappers_api::shared::db_url::build_database_url);

        let osu_client_id = std::env::var("OSU_CLIENT_ID")
            .map_err(|_| "OSU_CLIENT_ID is required".to_string())?
            .parse::<u64>()
            .map_err(|_| "OSU_CLIENT_ID must be an integer".to_string())?;

        let osu_client_secret = std::env::var("OSU_CLIENT_SECRET")
            .map_err(|_| "OSU_CLIENT_SECRET is required".to_string())?;

        let page_delay_ms = std::env::var("SCAN_PAGE_DELAY_MS")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(500);

        let max_pages = std::env::var("SCAN_MAX_PAGES")
            .ok()
            .and_then(|value| value.parse::<u32>().ok());

        let batch_size = std::env::var("SCAN_BATCH_SIZE")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or(50);

        Ok(Self {
            database_url,
            osu_client_id,
            osu_client_secret,
            page_delay_ms,
            max_pages,
            batch_size,
        })
    }
}
