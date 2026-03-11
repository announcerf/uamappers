use crate::shared::errors::ErrorResponse;

use super::IngestStatusDto;

#[utoipa::path(
    get,
    path = "/ingest/status",
    tag = "Ingest::Status",
    summary = "Get ingest scan status",
    responses(
        (status = 200, description = "Current ingest status", body = IngestStatusDto),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    operation_id = "ingest_status"
)]
pub async fn get_status() {}
