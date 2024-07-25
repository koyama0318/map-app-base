use crate::domain::domain_error::DomainError;
use axum::response::Response;
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    code: u16,
    message: String,
}

impl ErrorResponse {
    pub fn new(status: StatusCode, message: &str) -> Self {
        ErrorResponse {
            code: status.as_u16(),
            message: message.to_string(),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}

pub fn handle_error(e: DomainError) -> ErrorResponse {
    match e {
        DomainError::ValidationError => ErrorResponse::new(StatusCode::BAD_REQUEST, &e.to_string()),
    }
}
