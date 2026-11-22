use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::account::Account;
use crate::domain::errors::DomainError;
use crate::domain::repositories::account_repository::AccountRepository;

pub struct PgAccountRepository {
  pool: PgPool,
}

impl PgAccountRepository {
  pub fn new(pool: PgPool) -> Self {
    Self { pool }
  }
}

#[async_trait]
impl AccountRepository for PgAccountRepository {
  async fn save(&self,account: &Account) -> Result<Account,DomainError> {
    let row = sqlx::query!(
      r#"
      INSERT INTO accounts (id, account_number, currency, balance, created_at)
      VALUES ($1, $2, $3, $4, $5)
      RETURNING id, account_number, currency, balance, created_at
      "#,
      account.id,
      account.account_number,
      account.currency,
      account.balance,
      account.created_at,
    )
    .fetch_one(&self.pool)
    .await
    .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

    Ok(Account {
      id: row.id,
      account_number: row.account_number,
      currency: row.currency,
      balance: row.balance,
      created_at: row.created_at,
    })
  }

  async fn find_by_id(&self,id: Uuid) -> Result<Option<Account>,DomainError> {
    let row = sqlx::query!(
      r#"
      SELECT id, account_number, currency, balance, created_at
      FROM accounts WHERE id = $1
      "#,
      id
    )
    .fetch_optional(&self.pool)
    .await
    .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

    Ok(row.map(|r| Account {
      id: r.id,
      account_number: r.account_number,
      currency: r.currency,
      balance: r.balance,
      created_at: r.created_at,
    }))
  }

  async fn deposit(&self,id:Uuid,amount_cents:i64) -> Result<Account,DomainError> {
    let row=sqlx::query!(
      r#"
      UPDATE accounts
      SET balance = balance + $1
      WHERE id = $2
      RETURNING id, account_number, currency, balance, created_at
      "#,
      amount_cents,
      id
    )
    .fetch_optional(&self.pool)
    .await
    .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

    row.map(|r| Account {
      id: r.id,
      account_number: r.account_number,
      currency: r.currency,
      balance: r.balance,
      created_at: r.created_at
    })
    .ok_or_else(|| DomainError::NotFound(format!("account {}", id)))
  }
}
