use axum::{extract::State, Json};

use crate::{app::state::AppState, shared::errors::ApiError};

use super::dto::{IngestStatusDtoV1, ScanStateDtoV1};

#[utoipa::path(
    get,
    path = "/ingest/status",
    tag = "Ingest::Status",
    responses(
        (status = 200, description = "Current ingest status", body = IngestStatusDtoV1),
        (status = 500, description = "Internal error", body = crate::shared::errors::ErrorResponse)
    ),
    operation_id = "ingest_status_v1"
)]
pub async fn get_status(
    State(state): State<AppState>,
) -> Result<Json<IngestStatusDtoV1>, ApiError> {
    let rows = state.scan_state_repo.list_all().await.map_err(|err| {
        tracing::error!(error = ?err, "failed to load ingest status");
        ApiError::internal("ingest_status_failed", "Failed to load ingest status")
    })?;

    let states = rows
        .into_iter()
        .map(|row| ScanStateDtoV1 {
            name: row.name,
            cursor: row.cursor,
            last_success_at: row.last_success_at,
            last_error_at: row.last_error_at,
            retry_count: row.retry_count,
            next_retry_at: row.next_retry_at,
            updated_at: row.updated_at,
        })
        .collect();

    Ok(Json(IngestStatusDtoV1 { states }))
}
