use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

use crate::domain::errors::DomainError;

#[derive(Debug)]
pub struct AppError(pub DomainError);

impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    let (status, msg) = match &self.0 {
      DomainError::Validation(m) => (StatusCode::BAD_REQUEST, m.clone()),
      DomainError::NotFound(what) => (StatusCode::NOT_FOUND, format!("{} not found", what)),
      DomainError::Infrastructure(e) => {
        tracing::error!(error = %e, "infrastructure error");
        (StatusCode::INTERNAL_SERVER_ERROR, "internal server error".to_string())
      }
    };
    (status, Json(json!({ "error": msg }))).into_response()
  }
}

impl From<DomainError> for AppError {
  fn from(e: DomainError) -> Self {
    AppError(e)
  }
}
