use chrono::{DateTime, Utc};
// use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct User {
  pub id: Uuid,
  pub email: String,
  pub name: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}
