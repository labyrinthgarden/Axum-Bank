use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct CreateUserRequest {
  pub email:String,
  pub name:String
}
