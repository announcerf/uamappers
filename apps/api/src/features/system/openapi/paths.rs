use crate::shared::errors::ErrorResponse;

use super::HealthDto;

#[utoipa::path(
    get,
    path = "/system/health",
    tag = "System::Health",
    summary = "Health check",
    responses(
        (status = 200, description = "Health check", body = HealthDto),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    operation_id = "system_health"
)]
pub async fn health() {}
