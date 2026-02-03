use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthDtoV1 {
    pub status: String,
}
