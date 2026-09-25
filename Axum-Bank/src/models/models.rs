use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct Account {
    pub id: Uuid,
    pub owner_name: String,
    pub balance: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct LedgerEntry {
    pub id: Uuid,
    pub account_id: Uuid,
    pub kind: String,
    pub amount: Decimal,
    pub counterparty_account_id: Option<Uuid>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAccountRequest {
  pub owner_name: String,
  pub initial_balance: Option<Decimal>,
}

#[derive(Debug, Deserialize)]
pub struct BalanceMutationRequest {
    pub amount: Decimal,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TransferRequest {
    pub from_account_id: Uuid,
    pub to_account_id: Uuid,
    pub amount: Decimal,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AccountResponse {
    pub id: Uuid,
    pub owner_name: String,
    pub balance: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    pub id: Uuid,
    pub account_id: Uuid,
    pub kind: String,
    pub amount: String,
    pub counterparty_account_id: Option<Uuid>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct TransferResponse {
    pub from_account: AccountResponse,
    pub to_account: AccountResponse,
}

impl From<Account> for AccountResponse {
    fn from(value: Account) -> Self {
        Self {
            id: value.id,
            owner_name: value.owner_name,
            balance: value.balance.to_string(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<LedgerEntry> for TransactionResponse {
    fn from(value: LedgerEntry) -> Self {
        Self {
            id: value.id,
            account_id: value.account_id,
            kind: value.kind,
            amount: value.amount.to_string(),
            counterparty_account_id: value.counterparty_account_id,
            description: value.description,
            created_at: value.created_at,
        }
    }
}
