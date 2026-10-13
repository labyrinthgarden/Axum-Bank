use uuid::Uuid;
use crate::{error::AppError,models::User,state::AppState};

pub struct CreateUserCommand {
  pub email:String,
  pub name:String
}

pub struct CreateUserResult {
  pub user_id: Uuid
}

pub async fn execute(state:&AppState,cmd:CreateUserCommand) -> Result<CreateUserResult,AppError> {
  let email = cmd.email.trim().to_lowercase();
  let name = cmd.name.trim();

  if email.is_empty() || !email.contains('@') {
    return Err(AppError::bad_request("invalid email"));
  }
  if name.is_empty() {
    return Err(AppError::bad_request("name is required"));
  }
  if name.len() > 120 {
    return Err(AppError::bad_request("name is too long"));
  }

  let user_id = Uuid::new_v4();

  sqlx::query(
    r#"INSERT INTO users (id,email,name) VALUES ($1,$2,$3)"#
  )
  .bind(user_id)
  .bind(&email)
  .bind(name)
  .execute(&state.db)
  .await?;

  Ok(CreateUserResult { user_id })
}
