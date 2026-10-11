// src/handlers/account_handler.rs
use axum::{extract::{Path, State}, Json};
use crate::models::account::AccountResponse;

pub async fn get_account(
    State(db): State<AppState>,
    Path(id): Path<u32>,
) -> Json<AccountResponse> {
    // Lógica para obtener la cuenta
    Json(AccountResponse { /* ... */ })
}
