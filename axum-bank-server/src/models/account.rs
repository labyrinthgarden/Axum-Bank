#[derive(Debug, Clone, FromRow)]
pub struct Account {
  pub id: Uuid,
  pub account_number: String,
  pub currency: String,
  pub balance: Decimal,
  pub status: String,
  pub r#type: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}
