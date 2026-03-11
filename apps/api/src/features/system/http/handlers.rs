use axum::Json;

use crate::shared::errors::ApiError;

use super::dto::HealthDto;

pub async fn health() -> Result<Json<HealthDto>, ApiError> {
    Ok(Json(HealthDto {
        status: "ok".to_string(),
    }))
}
