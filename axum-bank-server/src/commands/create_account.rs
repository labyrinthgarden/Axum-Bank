use std::str::FromStr;

use uuid::Uuid;
use crate::{
  error::AppError,
  // models::Account,
  state::AppState
};

const MAX_ACCOUNT_NUMBER_ATTEMPTS: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountType {
  Checking,
  Savings,
  Investment,
  Credit,
}

impl AccountType {
  pub fn as_str(self) -> &'static str {
    match self {
      Self::Checking => "checking",
      Self::Savings => "savings",
      Self::Investment => "investment",
      Self::Credit => "credit",
    }
  }
}

impl FromStr for AccountType {
  type Err = AppError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "checking" => Ok(Self::Checking),
      "savings" => Ok(Self::Savings),
      "investment" => Ok(Self::Investment),
      "credit" => Ok(Self::Credit),
      other => Err(AppError::bad_request(format!("{other} is not a valid account type"))),
    }
  }
}

pub struct CreateAccountCommand{
  pub owner_id:Uuid,
  pub currency:String,
  pub account_type:AccountType
}

pub struct CreateAccountResult{
  pub account_number:String
}

pub async fn execute(state:&AppState,cmd:CreateAccountCommand)->Result<CreateAccountResult,AppError>{
  let currency=normalize_currency(&cmd.currency)?;

  let mut tx=state.db.begin().await?;

  let owner_exists=sqlx::query_scalar::<_,bool>(r#"
  SELECT EXISTS (SELECT 1 FROM users WHERE id=$1)"#)
  .bind(cmd.owner_id)
  .fetch_one(&mut *tx)
  .await?;
  if !owner_exists {
    return Err(AppError::not_found("owner not found"));
  }

  let account_id=Uuid::new_v4();
  let mut account_number=None;

  // ON CONFLICT DO NOTHING evita el error 23505, que abortaría la transacción.
  for _ in 0..MAX_ACCOUNT_NUMBER_ATTEMPTS {
    let candidate=generate_account_number();

    let inserted:Option<Uuid>=sqlx::query_scalar(r#"
    INSERT INTO accounts (id,account_number,currency,type) VALUES ($1,$2,$3,$4)
    ON CONFLICT (account_number) DO NOTHING
    RETURNING id"#)
    .bind(account_id)
    .bind(&candidate)
    .bind(&currency)
    .bind(cmd.account_type.as_str())
    .fetch_optional(&mut *tx)
    .await?;

    if inserted.is_some() {
      account_number=Some(candidate);
      break;
    }
  }

  let account_number=account_number
    .ok_or_else(|| AppError::Internal("could not allocate a unique account number".into()))?;

  sqlx::query(r#"INSERT INTO account_holders (account_id,user_id) VALUES ($1,$2)"#)
  .bind(account_id)
  .bind(cmd.owner_id)
  .execute(&mut *tx)
  .await?;

  tx.commit().await?;

  Ok(CreateAccountResult {account_number})
}

fn normalize_currency(raw:&str)->Result<String,AppError>{
  let currency=raw.trim().to_ascii_uppercase();
  if currency.len()!=3 || !currency.bytes().all(|b| b.is_ascii_uppercase()) {
    return Err(AppError::bad_request("currency must be a 3-letter ISO 4217 code"));
  }
  Ok(currency)
}

fn luhn_check_digit(body: &str) -> u8 {
  let mut sum = 0u32;
  let mut double = true; // because we're computing check digit for the right side
  for ch in body.chars().rev() {
    let mut d = ch.to_digit(10).unwrap();
    if double {
      d *= 2;
      if d > 9 { d -= 9; }
    }
    sum += d;
    double = !double;
  }
  ((10 - (sum % 10)) % 10) as u8
}

fn generate_account_number() -> String {
  let n = Uuid::new_v4().as_u128() % 1_000_000_000_000u128; // 12 digits
  let body = format!("AB{:012}", n);
  let check = luhn_check_digit(&body[2..]); // run on numeric part
  format!("{body}{check}")
}
