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

  async fn transfer(&self,from_account_id: Uuid,to_account_id: Uuid,amount_cents: i64,) -> Result<Account, DomainError> {
    if amount_cents <= 0 {
      return Err(DomainError::Validation(
        "amount_cents must be > 0".into(),
      ));
    }
    if from_account_id == to_account_id {
      return Err(DomainError::Validation(
        "cannot transfer to the same account".into(),
      ));
    }

    let mut tx = self
      .pool
      .begin()
      .await
      .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

    let from = sqlx::query!(
      r#"
      UPDATE accounts
      SET balance = balance - $2
      WHERE id = $1 AND balance >= $2
      RETURNING id, account_number, currency, balance, created_at
      "#,
      from_account_id,
      amount_cents,
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

    let from = match from {
      Some(r) => r,
      None => {
        let exists: Option<(Uuid,)> =
          sqlx::query_as("SELECT id FROM accounts WHERE id = $1")
            .bind(from_account_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

        return Err(match exists {
          Some(_) => DomainError::InsufficientFunds,
          None => DomainError::NotFound(format!("account {from_account_id}")),
        });
      }
    };

    sqlx::query!(
      r#"
      UPDATE accounts
      SET balance = balance + $2
      WHERE id = $1
      "#,
      to_account_id,
      amount_cents,
    )
    .execute(&mut *tx)
    .await
    .map_err(|e| DomainError::Infrastructure(e.to_string()))?
    .rows_affected()
    .eq(&1)
    .then_some(())
    .ok_or_else(|| DomainError::NotFound(format!("account {to_account_id}")))?;

    tx.commit()
      .await
      .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

    Ok(Account {
      id: from.id,
      account_number: from.account_number,
      currency: from.currency,
      balance: from.balance,
      created_at: from.created_at,
    })
  }

  async fn withdraw(&self,id:Uuid,amount_cents:i64) -> Result<Account,DomainError> {
    let row=sqlx::query!(r#"
      UPDATE accounts
      SET balance = balance - $1
      WHERE id = $2
      RETURNING id, account_number, currency, balance, created_at
    "#,
    amount_cents,
    id
    )
    .fetch_optional(&self.pool)
    .await
    .map_err(|e| DomainError::Infrastructure(e.to_string()))?;

    row.map(|r| Account{
      id: r.id,
      account_number: r.account_number,
      currency: r.currency,
      balance: r.balance,
      created_at: r.created_at
    })
    .ok_or_else(|| DomainError::NotFound(format!("account {}",id)))
  }
}
