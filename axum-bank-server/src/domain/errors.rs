#[derive(Debug,thiserror::Error)]
pub enum DomainError {
  #[error("validation error: {0}")]
  Validation(String),

  #[error("{0} not found")]
  NotFound(String),

  #[error("infrastructure error: {0}")]
  Infrastructure(String),

  #[error("insufficient funds")]
  InsufficientFunds
}
