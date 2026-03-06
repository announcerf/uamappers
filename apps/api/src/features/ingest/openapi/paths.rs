use crate::shared::errors::ErrorResponse;

use super::IngestStatusDtoV1;

#[utoipa::path(
    get,
    path = "/ingest/status",
    tag = "Ingest::Status",
    summary = "Get ingest scan status",
    responses(
        (status = 200, description = "Current ingest status", body = IngestStatusDtoV1),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    operation_id = "ingest_status_v1"
)]
pub async fn get_status() {}
