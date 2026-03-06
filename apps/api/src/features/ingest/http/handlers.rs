use axum::{Json, extract::State};

use crate::{app::state::AppState, features::ingest::usecases, shared::errors::ApiError};

use super::dto::{IngestStatusDtoV1, ScanStateDtoV1};

pub async fn get_status(
    State(state): State<AppState>,
) -> Result<Json<IngestStatusDtoV1>, ApiError> {
    let rows = usecases::list_scan_states(&state.scan_state_repo)
        .await
        .map_err(|err| {
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
