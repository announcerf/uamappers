#[derive(Clone, Debug)]
pub struct WorkerConfig {
    pub beatmapsets_page_size: usize,
    pub progress_log_every: u64,
    pub database_url: String,
    pub enrich_beatmapsets: bool,
    pub enrich_users: bool,
    pub discovery_oldest_first: bool,
    pub osu_client_id: u64,
    pub osu_client_secret: String,
    pub page_delay_ms: u64,
    pub max_pages: Option<u32>,
    pub batch_size: usize,
    pub force_rescan: bool,
    pub resume_from_checkpoint: bool,
    pub run_discovery: bool,
}

impl WorkerConfig {
    pub fn load() -> Result<Self, String> {
        let beatmapsets_page_size = std::env::var("ENRICH_BEATMAPSETS_PAGE_SIZE")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or(50);

        let progress_log_every = std::env::var("WORKER_PROGRESS_EVERY")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(25);

        let database_url = uamappers_api::shared::db_url::build_database_url();

        let enrich_beatmapsets = std::env::var("ENRICH_BEATMAPSETS")
            .ok()
            .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
            .unwrap_or(true);

        let enrich_users = std::env::var("ENRICH_USERS")
            .ok()
            .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
            .unwrap_or(true);

        let discovery_oldest_first = std::env::var("SCAN_OLDEST_FIRST")
            .ok()
            .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
            .unwrap_or(false);

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

        let force_rescan = std::env::var("SCAN_FORCE_RESCAN")
            .ok()
            .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
            .unwrap_or(false);

        let resume_from_checkpoint = std::env::var("SCAN_RESUME_FROM_CHECKPOINT")
            .ok()
            .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
            .unwrap_or(true);

        let run_discovery = std::env::var("RUN_DISCOVERY")
            .ok()
            .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
            .unwrap_or(true);

        Ok(Self {
            beatmapsets_page_size,
            progress_log_every,
            database_url,
            enrich_beatmapsets,
            enrich_users,
            discovery_oldest_first,
            osu_client_id,
            osu_client_secret,
            page_delay_ms,
            max_pages,
            batch_size,
            force_rescan,
            resume_from_checkpoint,
            run_discovery,
        })
    }
}
