use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::domain::account::Account;

#[derive(Deserialize)]
pub struct CreateAccountRequest {
  pub owner_name: String,
}

#[derive(Serialize)]
pub struct AccountResponse {
  pub id: String,
  pub balance: i64,
  pub created_at: String,
}

impl From<Account> for AccountResponse {
  fn from(a: Account) -> Self {
    Self {
      id: a.id.to_string(),
      balance: a.balance,
      created_at: a.created_at.to_rfc3339(),
    }
  }
}

#[derive(Deserialize)]
pub struct DepositRequest {
  pub amount_cents: i64
}

#[derive(Deserialize)]
pub struct TransferRequest {
  pub amount_cents: i64,
  pub account_to_id: String
}

#[derive(Deserialize)]
pub struct WithdrawRequest {
  pub amount_cents: i64
}
