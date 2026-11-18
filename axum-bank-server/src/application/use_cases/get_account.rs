use std::sync::Arc;
use uuid::Uuid;

use crate::domain::account::Account;
use crate::domain::errors::DomainError;
use crate::domain::repositories::account_repository::AccountRepository;

pub struct GetAccountUseCase {
  repo: Arc<dyn AccountRepository>,
}

impl GetAccountUseCase {
  pub fn new(repo: Arc<dyn AccountRepository>) -> Self {
    Self { repo }
  }

  pub async fn execute(&self, id: Uuid) -> Result<Account, DomainError> {
    self.repo
      .find_by_id(id)
      .await?
      .ok_or_else(|| DomainError::NotFound(format!("account {}", id)))
  }
}
