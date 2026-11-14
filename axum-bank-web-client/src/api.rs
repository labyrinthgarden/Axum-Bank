use gloo_net::http::{Request, Response};
use serde::de::DeserializeOwned;

use crate::models::{
    Account, ApiError, BalanceMutationPayload, CreateAccountPayload, Transaction, TransferPayload,
};

const API_BASE: &str = "/api";

pub async fn api_list_accounts() -> Result<Vec<Account>, String> {
    let response = Request::get(&format!("{API_BASE}/accounts"))
        .send()
        .await
        .map_err(|err| err.to_string())?;

    parse_json_response(response).await
}

pub async fn api_list_transactions(account_id: &str) -> Result<Vec<Transaction>, String> {
    let response = Request::get(&format!("{API_BASE}/accounts/{account_id}/transactions"))
        .send()
        .await
        .map_err(|err| err.to_string())?;

    parse_json_response(response).await
}

pub async fn api_create_account(payload: CreateAccountPayload) -> Result<Account, String> {
    let response = Request::post(&format!("{API_BASE}/accounts"))
        .json(&payload)
        .map_err(|err| err.to_string())?
        .send()
        .await
        .map_err(|err| err.to_string())?;

    parse_json_response(response).await
}

pub async fn api_deposit(
    account_id: &str,
    payload: BalanceMutationPayload,
) -> Result<Account, String> {
    let response = Request::post(&format!("{API_BASE}/accounts/{account_id}/deposit"))
        .json(&payload)
        .map_err(|err| err.to_string())?
        .send()
        .await
        .map_err(|err| err.to_string())?;

    parse_json_response(response).await
}

pub async fn api_withdraw(
    account_id: &str,
    payload: BalanceMutationPayload,
) -> Result<Account, String> {
    let response = Request::post(&format!("{API_BASE}/accounts/{account_id}/withdraw"))
        .json(&payload)
        .map_err(|err| err.to_string())?
        .send()
        .await
        .map_err(|err| err.to_string())?;

    parse_json_response(response).await
}

pub async fn api_transfer(payload: TransferPayload) -> Result<serde_json::Value, String> {
    let response = Request::post(&format!("{API_BASE}/transfers"))
        .json(&payload)
        .map_err(|err| err.to_string())?
        .send()
        .await
        .map_err(|err| err.to_string())?;

    parse_json_response(response).await
}

async fn parse_json_response<T: DeserializeOwned>(response: Response) -> Result<T, String> {
    let status = response.status();
    let body = response.text().await.map_err(|err| err.to_string())?;

    if (200..300).contains(&status) {
        return serde_json::from_str::<T>(&body).map_err(|err| err.to_string());
    }

    if let Ok(error) = serde_json::from_str::<ApiError>(&body) {
        return Err(error.error);
    }

    Err(format!("Request failed with status {status}: {body}"))
}
