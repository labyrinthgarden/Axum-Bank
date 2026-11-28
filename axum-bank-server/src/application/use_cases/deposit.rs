use std::sync::Arc;
use uuid::Uuid;

use crate::domain::account::Account;
use crate::domain::errors::DomainError;
use crate::domain::repositories::account_repository::AccountRepository;

pub struct DepositUseCase{
  repo: Arc<dyn AccountRepository>
}

impl DepositUseCase {
  pub fn new(repo: Arc<dyn AccountRepository>) -> Self {
    Self {repo}
  }

  pub async fn execute(&self,id:Uuid,amount_cents:i64) -> Result<Account,DomainError> {
    if amount_cents <=1 {
      return Err(DomainError::Validation(
        "deposit amount must be greater than 1".into()
      ));
    }

    self.repo.deposit(id,amount_cents).await
  }
}
