use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;
use rust_decimal::Decimal;

use crate::domain::account::Account;
use crate::domain::errors::DomainError;
use crate::domain::repositories::account_repository::AccountRepository;

pub struct CreateAccountUseCase {
  repo: Arc<dyn AccountRepository>,
}

impl CreateAccountUseCase {
  pub fn new(repo: Arc<dyn AccountRepository>) -> Self {
    Self { repo }
  }

  pub async fn execute(&self,owner_name: String) -> Result<Account,DomainError> {
    if owner_name.trim().is_empty() {
      return Err(DomainError::Validation("owner_name cannot be empty".into()));
    }
    let account = Account {
      id: Uuid::new_v4(),
      account_number: generate_account_number(),
      currency:"USD".to_string(),
      balance: 0,
      created_at: Utc::now(),
    };
    self.repo.save(&account).await
  }
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
