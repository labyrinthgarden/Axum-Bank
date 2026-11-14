use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize)]
pub struct Account {
    pub id: String,
    pub owner_name: String,
    pub balance: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub account_id: String,
    pub kind: String,
    pub amount: String,
    pub counterparty_account_id: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct ApiError {
    pub error: String,
}

#[derive(Serialize)]
pub struct CreateAccountPayload {
    pub owner_name: String,
    pub initial_balance: Option<String>,
}

#[derive(Serialize)]
pub struct BalanceMutationPayload {
    pub amount: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct TransferPayload {
    pub from_account_id: String,
    pub to_account_id: String,
    pub amount: String,
    pub description: Option<String>,
}
