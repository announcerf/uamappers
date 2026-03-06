use crate::shared::errors::{ErrorResponse, response::FieldError};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub code: String,
    pub message: String,
    pub fields: Option<Vec<FieldError>>,
    pub request_id: Option<String>,
}

impl ApiError {
    pub fn new(status: StatusCode, code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            status,
            code: code.into(),
            message: message.into(),
            fields: None,
            request_id: None,
        }
    }

    pub fn with_field(mut self, field: impl Into<String>, message: impl Into<String>) -> Self {
        let field_error = FieldError {
            field: field.into(),
            message: message.into(),
        };
        match &mut self.fields {
            Some(fields) => fields.push(field_error),
            None => self.fields = Some(vec![field_error]),
        }
        self
    }

    pub fn not_found(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, code, message)
    }

    pub fn bad_request(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, code, message)
    }

    pub fn internal(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, code, message)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let response = ErrorResponse {
            code: self.code,
            message: self.message,
            fields: self.fields,
            request_id: self.request_id,
        };

        (self.status, Json(response)).into_response()
    }
}
