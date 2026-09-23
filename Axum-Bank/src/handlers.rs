use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use rust_decimal::Decimal;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::{
    AppState,
    error::AppError,
    models::{
        Account, AccountResponse, BalanceMutationRequest, CreateAccountRequest, LedgerEntry,
        TransactionResponse, TransferRequest, TransferResponse,
    },
};

const MAX_DESCRIPTION_LENGTH: usize = 255;
const MAX_OWNER_NAME_LENGTH: usize = 120;
const TX_KIND_DEPOSIT: &str = "deposit";
const TX_KIND_WITHDRAWAL: &str = "withdrawal";
const TX_KIND_TRANSFER_IN: &str = "transfer_in";
const TX_KIND_TRANSFER_OUT: &str = "transfer_out";
const TX_KIND_INITIAL_DEPOSIT: &str = "initial_deposit";

#[derive(serde::Serialize)]
pub struct HealthResponse {
    status: &'static str,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

pub async fn list_accounts(
    State(state): State<AppState>,
) -> Result<Json<Vec<AccountResponse>>, AppError> {
    let rows = sqlx::query_as::<_, Account>(
        r#"
        SELECT id, owner_name, balance, created_at, updated_at
        FROM accounts
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(rows.into_iter().map(AccountResponse::from).collect()))
}

pub async fn create_account(
    State(state): State<AppState>,
    Json(request): Json<CreateAccountRequest>,
) -> Result<(StatusCode, Json<AccountResponse>), AppError> {
    let owner_name = request.owner_name.trim();
    if owner_name.is_empty() {
        return Err(AppError::bad_request("owner_name is required"));
    }
    if owner_name.len() > MAX_OWNER_NAME_LENGTH {
        return Err(AppError::bad_request(format!(
            "owner_name must be at most {MAX_OWNER_NAME_LENGTH} characters"
        )));
    }

    let initial_balance = request.initial_balance.unwrap_or(Decimal::ZERO);
    if initial_balance < Decimal::ZERO {
        return Err(AppError::bad_request("initial_balance cannot be negative"));
    }

    let mut tx = state.db.begin().await?;

    let account_id = Uuid::new_v4();
    let account = sqlx::query_as::<_, Account>(
        r#"
        INSERT INTO accounts (id, owner_name, balance)
        VALUES ($1, $2, $3)
        RETURNING id, owner_name, balance, created_at, updated_at
        "#,
    )
    .bind(account_id)
    .bind(owner_name)
    .bind(initial_balance)
    .fetch_one(&mut *tx)
    .await?;

    if initial_balance > Decimal::ZERO {
        insert_transaction(
            &mut tx,
            account.id,
            TX_KIND_INITIAL_DEPOSIT,
            initial_balance,
            None,
            Some("Initial deposit".to_string()),
        )
        .await?;
    }

    tx.commit().await?;

    Ok((StatusCode::CREATED, Json(account.into())))
}

pub async fn deposit(
    State(state): State<AppState>,
    Path(account_id): Path<Uuid>,
    Json(request): Json<BalanceMutationRequest>,
) -> Result<Json<AccountResponse>, AppError> {
    validate_positive_amount(request.amount)?;
    let description = normalize_description(request.description)?;

    let mut tx = state.db.begin().await?;

    let account = lock_account(&mut tx, account_id).await?;
    let new_balance = account.balance + request.amount;
    let updated_account = update_balance(&mut tx, account_id, new_balance).await?;

    insert_transaction(
        &mut tx,
        account_id,
        TX_KIND_DEPOSIT,
        request.amount,
        None,
        description,
    )
    .await?;

    tx.commit().await?;

    Ok(Json(updated_account.into()))
}

pub async fn withdraw(
    State(state): State<AppState>,
    Path(account_id): Path<Uuid>,
    Json(request): Json<BalanceMutationRequest>,
) -> Result<Json<AccountResponse>, AppError> {
    validate_positive_amount(request.amount)?;
    let description = normalize_description(request.description)?;

    let mut tx = state.db.begin().await?;

    let account = lock_account(&mut tx, account_id).await?;
    if account.balance < request.amount {
        return Err(AppError::conflict("insufficient funds"));
    }

    let new_balance = account.balance - request.amount;
    let updated_account = update_balance(&mut tx, account_id, new_balance).await?;

    insert_transaction(
        &mut tx,
        account_id,
        TX_KIND_WITHDRAWAL,
        request.amount,
        None,
        description,
    )
    .await?;

    tx.commit().await?;

    Ok(Json(updated_account.into()))
}

pub async fn transfer(
    State(state): State<AppState>,
    Json(request): Json<TransferRequest>,
) -> Result<Json<TransferResponse>, AppError> {
    validate_positive_amount(request.amount)?;

    if request.from_account_id == request.to_account_id {
        return Err(AppError::bad_request(
            "from_account_id and to_account_id must be different",
        ));
    }

    let description = normalize_description(request.description)?;

    let mut tx = state.db.begin().await?;

    let (first_id, second_id) = if request.from_account_id < request.to_account_id {
        (request.from_account_id, request.to_account_id)
    } else {
        (request.to_account_id, request.from_account_id)
    };

    let first_account = lock_account(&mut tx, first_id).await?;
    let second_account = lock_account(&mut tx, second_id).await?;

    let (source_account, destination_account) = if first_account.id == request.from_account_id {
        (first_account, second_account)
    } else {
        (second_account, first_account)
    };

    if source_account.balance < request.amount {
        return Err(AppError::conflict("insufficient funds"));
    }

    let source_new_balance = source_account.balance - request.amount;
    let destination_new_balance = destination_account.balance + request.amount;

    let source_updated = update_balance(&mut tx, source_account.id, source_new_balance).await?;
    let destination_updated =
        update_balance(&mut tx, destination_account.id, destination_new_balance).await?;

    insert_transaction(
        &mut tx,
        source_account.id,
        TX_KIND_TRANSFER_OUT,
        request.amount,
        Some(destination_account.id),
        description.clone(),
    )
    .await?;

    insert_transaction(
        &mut tx,
        destination_account.id,
        TX_KIND_TRANSFER_IN,
        request.amount,
        Some(source_account.id),
        description,
    )
    .await?;

    tx.commit().await?;

    Ok(Json(TransferResponse {
        from_account: source_updated.into(),
        to_account: destination_updated.into(),
    }))
}

pub async fn list_transactions(
    State(state): State<AppState>,
    Path(account_id): Path<Uuid>,
) -> Result<Json<Vec<TransactionResponse>>, AppError> {
    ensure_account_exists(&state, account_id).await?;

    let rows = sqlx::query_as::<_, LedgerEntry>(
        r#"
        SELECT id, account_id, kind, amount, counterparty_account_id, description, created_at
        FROM transactions
        WHERE account_id = $1
        ORDER BY created_at DESC
        LIMIT 100
        "#,
    )
    .bind(account_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(
        rows.into_iter().map(TransactionResponse::from).collect(),
    ))
}

async fn ensure_account_exists(state: &AppState, account_id: Uuid) -> Result<(), AppError> {
    let exists: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM accounts WHERE id = $1")
        .bind(account_id)
        .fetch_optional(&state.db)
        .await?;

    if exists.is_none() {
        return Err(AppError::not_found("account not found"));
    }

    Ok(())
}

fn validate_positive_amount(amount: Decimal) -> Result<(), AppError> {
    if amount <= Decimal::ZERO {
        return Err(AppError::bad_request("amount must be greater than zero"));
    }
    Ok(())
}

fn normalize_description(description: Option<String>) -> Result<Option<String>, AppError> {
    let Some(text) = description else {
        return Ok(None);
    };

    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    if trimmed.len() > MAX_DESCRIPTION_LENGTH {
        return Err(AppError::bad_request(format!(
            "description must be at most {MAX_DESCRIPTION_LENGTH} characters"
        )));
    }

    Ok(Some(trimmed.to_string()))
}

async fn lock_account(
    tx: &mut Transaction<'_, Postgres>,
    account_id: Uuid,
) -> Result<Account, AppError> {
    let account = sqlx::query_as::<_, Account>(
        r#"
        SELECT id, owner_name, balance, created_at, updated_at
        FROM accounts
        WHERE id = $1
        FOR UPDATE
        "#,
    )
    .bind(account_id)
    .fetch_optional(&mut **tx)
    .await?;

    account.ok_or_else(|| AppError::not_found("account not found"))
}

async fn update_balance(
    tx: &mut Transaction<'_, Postgres>,
    account_id: Uuid,
    new_balance: Decimal,
) -> Result<Account, AppError> {
    let updated = sqlx::query_as::<_, Account>(
        r#"
        UPDATE accounts
        SET balance = $2,
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, owner_name, balance, created_at, updated_at
        "#,
    )
    .bind(account_id)
    .bind(new_balance)
    .fetch_one(&mut **tx)
    .await?;

    Ok(updated)
}

async fn insert_transaction(
    tx: &mut Transaction<'_, Postgres>,
    account_id: Uuid,
    kind: &str,
    amount: Decimal,
    counterparty_account_id: Option<Uuid>,
    description: Option<String>,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO transactions (id, account_id, kind, amount, counterparty_account_id, description)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(account_id)
    .bind(kind)
    .bind(amount)
    .bind(counterparty_account_id)
    .bind(description)
    .execute(&mut **tx)
    .await?;

    Ok(())
}
