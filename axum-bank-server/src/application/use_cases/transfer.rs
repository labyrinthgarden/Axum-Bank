use std::sync::Arc;
use uuid::Uuid;

use crate::domain::account::Account;
use crate::domain::errors::DomainError;
use crate::domain::repositories::account_repository::AccountRepository;

pub struct TransferUseCase {
  repo: Arc<dyn AccountRepository>
}

impl TransferUseCase {
  pub fn new(repo: Arc<dyn AccountRepository>) -> Self {
    Self {repo}
  }

  pub async fn execute(&self,from_account_id:Uuid,to_account_id:Uuid,amount_cents:i64) -> Result<Account,DomainError> {
    if amount_cents <= 0{
      return Err(DomainError::Validation(
        "transfer amount must be greater than zero.".into()
      ));
    }

    self.repo.transfer(from_account_id,to_account_id,amount_cents).await
  }
}
