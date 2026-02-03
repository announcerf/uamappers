use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub fields: Option<Vec<FieldError>>,
    pub request_id: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            fields: None,
            request_id: None,
        }
    }
}
