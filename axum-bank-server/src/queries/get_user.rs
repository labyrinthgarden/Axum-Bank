use uuid::Uuid;
use crate::{error::AppError,models::User,state::AppState};

pub struct GetUserQuery {
  pub user_id: Uuid
}

pub async fn execute(state:&AppState,query:GetUserQuery) -> Result<User,AppError> {
  let user = sqlx::query_as::<_,User>(
    r#"SELECT * FROM users WHERE id=$1"#
  ).bind(query.user_id)
  .fetch_optional(&state.db)
  .await?
  .ok_or_else(|| AppError::not_found("user not found"))?;
  Ok(user)
}
