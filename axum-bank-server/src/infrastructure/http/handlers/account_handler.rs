use axum::{extract::State, extract::Path, http::StatusCode, Json};
use uuid::Uuid;

use crate::app_state::AppState;
use crate::errors::app_error::AppError;
use crate::infrastructure::http::dto::account_dto::{AccountResponse, CreateAccountRequest, DepositRequest, TransferRequest};

pub async fn create_account(
  State(state): State<AppState>,
  Json(payload): Json<CreateAccountRequest>,
) -> Result<(StatusCode, Json<AccountResponse>), AppError> {
  let account = state.create_account.execute(payload.owner_name).await?;
  Ok((StatusCode::CREATED, Json(AccountResponse::from(account))))
}

pub async fn get_account(
  State(state): State<AppState>,
  Path(id): Path<Uuid>,
) -> Result<Json<AccountResponse>,AppError>{
  let account = state.get_account.execute(id).await?;
  Ok(Json(AccountResponse::from(account)))
}

pub async fn deposit(
  State(state): State<AppState>,
  Path(id): Path<Uuid>,
  Json(payload): Json<DepositRequest>
) -> Result<Json<AccountResponse>,AppError> {
  let account = state.deposit.execute(id,payload.amount_cents).await?;
  Ok(Json(AccountResponse::from(account)))
}

pub async fn transfer(
  State(state): State<AppState>,
  Path(id): Path<Uuid>,
  Json(payload): Json<TransferRequest>
) -> Result<Json<AccountResponse>, AppError> {
  let account = state.transfer.execute(id,payload.account_to_id,payload.amount_cents).await?;
  Ok(Json(AccountResponse::from(account)))
}
