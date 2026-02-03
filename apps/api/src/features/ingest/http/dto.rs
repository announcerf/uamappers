use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ScanStateDtoV1 {
    pub name: String,
    pub cursor: Option<String>,
    pub last_success_at: Option<DateTime<Utc>>,
    pub last_error_at: Option<DateTime<Utc>>,
    pub retry_count: i32,
    pub next_retry_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct IngestStatusDtoV1 {
    pub states: Vec<ScanStateDtoV1>,
}
