use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
  Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
  BadRequest(String),
  NotFound(String),
  Conflict(String),
  Internal(String),
  Database(sqlx::Error),
}

impl AppError {
  pub fn bad_request(msg: impl Into<String>) -> Self {
    Self::BadRequest(msg.into())
  }
  pub fn not_found(msg: impl Into<String>) -> Self {
    Self::NotFound(msg.into())
  }
  pub fn conflict(msg: impl Into<String>) -> Self {
    Self::Conflict(msg.into())
  }
}

impl From<sqlx::Error> for AppError {
  fn from(err: sqlx::Error) -> Self {
    // Detecta violación de UNIQUE para devolver 409 en vez de 500
    if let sqlx::Error::Database(db_err) = &err {
      if db_err.code().as_deref() == Some("23505") {
        return Self::Conflict("resource already exists".into());
      }
    }
    Self::Database(err)
  }
}

impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    let (status, message) = match self {
      AppError::BadRequest(m) => (StatusCode::BAD_REQUEST, m),
      AppError::NotFound(m) => (StatusCode::NOT_FOUND, m),
      AppError::Conflict(m) => (StatusCode::CONFLICT, m),
      AppError::Internal(m) => (StatusCode::INTERNAL_SERVER_ERROR, m),
      AppError::Database(e) => {
        eprintln!("DB error: {:?}", e);
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          "internal server error".to_string(),
        )
      }
    };

    (status, Json(json!({ "error": message }))).into_response()
  }
}
