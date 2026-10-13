use uuid::Uuid;
use serde::Deserialize;
use rust_decimal::Decimal;

#[derive(Debug,Deserialize)]
pub struct CreateUserRequest {
  pub email:String,
  pub name:String
}

#[derive(Debug,Deserialize)]
pub struct CreateAccountRequest {
  pub owner_id:Uuid,
  pub currency:String,
  pub account_type:String,
}

#[derive(Debug,Deserialize)]
pub struct DepositMoneyRequest {
  pub account_id:Uuid,
  pub amount:Decimal,
  pub description:Option<String>
}
