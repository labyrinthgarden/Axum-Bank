use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::account::Account;
use crate::domain::errors::DomainError;

#[async_trait]
pub trait AccountRepository: Send + Sync {
  async fn save(&self,account:&Account) -> Result<Account,DomainError>;
  async fn find_by_id(&self,id:Uuid) -> Result<Option<Account>,DomainError>;
  async fn deposit(&self,id:Uuid,amount_cents:i64) -> Result<Account,DomainError>;
}
