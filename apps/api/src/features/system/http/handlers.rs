use axum::Json;

use crate::shared::errors::ApiError;

use super::dto::HealthDtoV1;

#[utoipa::path(
    get,
    path = "/system/health",
    tag = "System::Health",
    summary = "Health check",
    responses(
        (status = 200, description = "Health check", body = HealthDtoV1),
        (status = 500, description = "Internal error", body = crate::shared::errors::ErrorResponse)
    ),
    operation_id = "system_health_v1"
)]
pub async fn health() -> Result<Json<HealthDtoV1>, ApiError> {
    Ok(Json(HealthDtoV1 {
        status: "ok".to_string(),
    }))
}
