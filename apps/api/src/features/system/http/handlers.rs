use axum::Json;

use crate::shared::errors::ApiError;

use super::dto::HealthDtoV1;

pub async fn health() -> Result<Json<HealthDtoV1>, ApiError> {
    Ok(Json(HealthDtoV1 {
        status: "ok".to_string(),
    }))
}
