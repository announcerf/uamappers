use crate::shared::errors::ErrorResponse;

use super::HealthDtoV1;

#[utoipa::path(
    get,
    path = "/system/health",
    tag = "System::Health",
    summary = "Health check",
    responses(
        (status = 200, description = "Health check", body = HealthDtoV1),
        (status = 500, description = "Internal error", body = ErrorResponse)
    ),
    operation_id = "system_health_v1"
)]
pub async fn health() {}
