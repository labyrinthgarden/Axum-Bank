use chrono::{DateTime,Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug,Clone)]
pub struct Account{
  pub id:Uuid,
  pub account_number:String,
  pub currency:String,
  pub balance:i64,
  pub created_at:DateTime<Utc>
}
