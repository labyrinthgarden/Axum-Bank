use axum::{extract::State, http::StatusCode, Json};

use crate::app_state::AppState;
use crate::errors::app_error::AppError;
use crate::infrastructure::http::dto::account_dto::{AccountResponse, CreateAccountRequest};

pub async fn create_account(
  State(state): State<AppState>,
  Json(payload): Json<CreateAccountRequest>,
) -> Result<(StatusCode, Json<AccountResponse>), AppError> {
  let account = state.create_account.execute(payload.owner_name).await?;
  Ok((StatusCode::CREATED, Json(AccountResponse::from(account))))
}
