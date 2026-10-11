use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{error::AppError, state::AppState};

// Debe coincidir con NUMERIC(20, 4) de las columnas de dinero del esquema.
const MONEY_SCALE: u32 = 4;
const MAX_DEPOSIT_AMOUNT: i64 = 1_000_000;
// Máximo que admite NUMERIC(20, 4): 16 dígitos enteros.
const MAX_ACCOUNT_BALANCE: i64 = 9_999_999_999_999_999;
const MAX_DESCRIPTION_LENGTH: usize = 255;

pub struct DepositMoneyCommand {
  pub account_id: Uuid,
  pub amount: Decimal,
  pub description: Option<String>,
}

pub struct DepositMoneyResult {
  pub ledger_entry_id: Uuid,
}

#[derive(sqlx::FromRow)]
struct LockedAccount {
  balance: Decimal,
  currency: String,
  status: String,
}

pub async fn execute(state: &AppState, cmd: DepositMoneyCommand) -> Result<DepositMoneyResult, AppError> {
  if cmd.amount <= Decimal::ZERO {
    return Err(AppError::bad_request("amount must be greater than zero"));
  }
  if cmd.amount.round_dp(MONEY_SCALE) != cmd.amount {
    return Err(AppError::bad_request(format!(
      "amount must have at most {MONEY_SCALE} decimal places"
    )));
  }
  if cmd.amount > Decimal::from(MAX_DEPOSIT_AMOUNT) {
    return Err(AppError::bad_request("amount exceeds the maximum allowed per deposit"));
  }
  let description = validate_description(cmd.description)?;

  let mut tx = state.db.begin().await?;

  // FOR UPDATE bloquea la fila hasta el commit: dos depósitos concurrentes
  // a la misma cuenta se serializan y ninguno pisa el saldo del otro.
  let account = sqlx::query_as::<_, LockedAccount>(
    r#"SELECT balance, currency, status FROM accounts WHERE id = $1 FOR UPDATE"#,
  )
  .bind(cmd.account_id)
  .fetch_optional(&mut *tx)
  .await?
  .ok_or_else(|| AppError::not_found("account not found"))?;

  if account.status != "active" {
    return Err(AppError::conflict("account is not active"));
  }

  let new_balance = account
    .balance
    .checked_add(cmd.amount)
    .filter(|balance| *balance <= Decimal::from(MAX_ACCOUNT_BALANCE))
    .ok_or_else(|| AppError::conflict("deposit would exceed the maximum account balance"))?;

  sqlx::query(r#"UPDATE accounts SET balance = $2, updated_at = NOW() WHERE id = $1"#)
    .bind(cmd.account_id)
    .bind(new_balance)
    .execute(&mut *tx)
    .await?;

  let ledger_entry_id = Uuid::new_v4();
  let correlation_id = Uuid::new_v4();

  sqlx::query(
    r#"
    INSERT INTO ledger_entries
      (id, correlation_id, account_id, kind, amount, currency, running_balance, description)
    VALUES ($1, $2, $3, $4::transaction_kind, $5, $6, $7, $8)
    "#,
  )
  .bind(ledger_entry_id)
  .bind(correlation_id)
  .bind(cmd.account_id)
  .bind("deposit")
  .bind(cmd.amount)
  .bind(&account.currency)
  .bind(new_balance)
  .bind(description)
  .execute(&mut *tx)
  .await?;

  tx.commit().await?;

  Ok(DepositMoneyResult { ledger_entry_id })
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
