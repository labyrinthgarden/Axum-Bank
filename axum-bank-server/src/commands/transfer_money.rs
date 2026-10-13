use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{error::AppError, state::AppState};

// Debe coincidir con NUMERIC(20, 4) de las columnas de dinero del esquema.
const MONEY_SCALE: u32 = 4;
const MAX_TRANSFER_AMOUNT: i64 = 1_000_000;
// Máximo que admite NUMERIC(20, 4): 16 dígitos enteros.
const MAX_ACCOUNT_BALANCE: i64 = 9_999_999_999_999_999;
const MAX_DESCRIPTION_LENGTH: usize = 255;

pub struct TransferMoneyCommand {
  pub from_account_id: Uuid,
  pub to_account_id: Uuid,
  pub amount: Decimal,
  pub description: Option<String>,
}

pub struct TransferMoneyResult {
  pub outgoing_ledger_entry_id: Uuid,
  pub incoming_ledger_entry_id: Uuid,
}

#[derive(sqlx::FromRow)]
struct LockedAccount {
  id: Uuid,
  balance: Decimal,
  currency: String,
  status: String,
}

pub async fn execute(state: &AppState, cmd: TransferMoneyCommand) -> Result<TransferMoneyResult, AppError> {
  if cmd.amount <= Decimal::ZERO {
    return Err(AppError::bad_request("amount must be greater than zero"));
  }
  if cmd.amount.round_dp(MONEY_SCALE) != cmd.amount {
    return Err(AppError::bad_request(format!(
      "amount must have at most {MONEY_SCALE} decimal places"
    )));
  }
  if cmd.amount > Decimal::from(MAX_TRANSFER_AMOUNT) {
    return Err(AppError::bad_request("amount exceeds the maximum allowed per transfer"));
  }
  if cmd.from_account_id == cmd.to_account_id {
    return Err(AppError::bad_request("source and destination accounts must be different"));
  }
  let description = validate_description(cmd.description)?;

  let mut tx = state.db.begin().await?;

  // ORDER BY id define el orden de bloqueo: dos transferencias cruzadas (A→B y B→A)
  // bloquean las mismas filas en el mismo orden y no se interbloquean.
  let accounts = sqlx::query_as::<_, LockedAccount>(
    r#"SELECT id, balance, currency, status FROM accounts WHERE id = ANY($1) ORDER BY id FOR UPDATE"#,
  )
  .bind(vec![cmd.from_account_id, cmd.to_account_id])
  .fetch_all(&mut *tx)
  .await?;

  let source = accounts
    .iter()
    .find(|a| a.id == cmd.from_account_id)
    .ok_or_else(|| AppError::not_found("source account not found"))?;
  let destination = accounts
    .iter()
    .find(|a| a.id == cmd.to_account_id)
    .ok_or_else(|| AppError::not_found("destination account not found"))?;

  if source.status != "active" {
    return Err(AppError::conflict("source account is not active"));
  }
  if destination.status != "active" {
    return Err(AppError::conflict("destination account is not active"));
  }
  if source.currency != destination.currency {
    return Err(AppError::conflict("accounts must have the same currency"));
  }

  let new_source_balance = source
    .balance
    .checked_sub(cmd.amount)
    .filter(|balance| *balance >= Decimal::ZERO)
    .ok_or_else(|| AppError::conflict("insufficient funds"))?;

  let new_destination_balance = destination
    .balance
    .checked_add(cmd.amount)
    .filter(|balance| *balance <= Decimal::from(MAX_ACCOUNT_BALANCE))
    .ok_or_else(|| AppError::conflict("transfer would exceed the maximum account balance"))?;

  sqlx::query(r#"UPDATE accounts SET balance = $2, updated_at = NOW() WHERE id = $1"#)
    .bind(cmd.from_account_id)
    .bind(new_source_balance)
    .execute(&mut *tx)
    .await?;

  sqlx::query(r#"UPDATE accounts SET balance = $2, updated_at = NOW() WHERE id = $1"#)
    .bind(cmd.to_account_id)
    .bind(new_destination_balance)
    .execute(&mut *tx)
    .await?;

  // Una transferencia genera dos movimientos que comparten correlation_id
  // para poder reconstruir la operación completa.
  let correlation_id = Uuid::new_v4();
  let outgoing_ledger_entry_id = Uuid::new_v4();
  let incoming_ledger_entry_id = Uuid::new_v4();

  sqlx::query(
    r#"
    INSERT INTO ledger_entries
      (id, correlation_id, account_id, kind, amount, currency, running_balance, counterparty_account_id, description)
    VALUES ($1, $2, $3, $4::transaction_kind, $5, $6, $7, $8, $9)
    "#,
  )
  .bind(outgoing_ledger_entry_id)
  .bind(correlation_id)
  .bind(cmd.from_account_id)
  .bind("transfer_out")
  .bind(cmd.amount)
  .bind(&source.currency)
  .bind(new_source_balance)
  .bind(cmd.to_account_id)
  .bind(description.as_deref())
  .execute(&mut *tx)
  .await?;

  sqlx::query(
    r#"
    INSERT INTO ledger_entries
      (id, correlation_id, account_id, kind, amount, currency, running_balance, counterparty_account_id, description)
    VALUES ($1, $2, $3, $4::transaction_kind, $5, $6, $7, $8, $9)
    "#,
  )
  .bind(incoming_ledger_entry_id)
  .bind(correlation_id)
  .bind(cmd.to_account_id)
  .bind("transfer_in")
  .bind(cmd.amount)
  .bind(&destination.currency)
  .bind(new_destination_balance)
  .bind(cmd.from_account_id)
  .bind(description.as_deref())
  .execute(&mut *tx)
  .await?;

  tx.commit().await?;

  Ok(TransferMoneyResult {
    outgoing_ledger_entry_id,
    incoming_ledger_entry_id,
  })
}

fn validate_description(description: Option<String>) -> Result<Option<String>, AppError> {
  let Some(text) = description.map(|d| d.trim().to_owned()).filter(|d| !d.is_empty()) else {
    return Ok(None);
  };

  if text.chars().count() > MAX_DESCRIPTION_LENGTH {
    return Err(AppError::bad_request(format!(
      "description must be at most {MAX_DESCRIPTION_LENGTH} characters"
    )));
  }

  Ok(Some(text))
}
