use axum::{extract::{Path, State},http::StatusCode,routing::{get, post},Json,Router,};
use uuid::Uuid;
use crate::{commands::{self, create_user::CreateUserCommand},error::AppError,
  http::{requests::CreateUserRequest,responses::{CreateUserResponse, UserResponse},},
  queries::{self, get_user::GetUserQuery},
  state::AppState,
};

pub fn router() -> Router<AppState> {
  Router::new()
  .route("/users", post(create_user_handler))
  .route("/users/{id}", get(get_user_handler))
}

/// Handler del command: crea un usuario.
async fn create_user_handler(State(state): State<AppState>,Json(req): Json<CreateUserRequest>,) -> Result<(StatusCode,Json<CreateUserResponse>),AppError> {
  // 1. Traducir DTO HTTP → Command
  let cmd = CreateUserCommand {
    email: req.email,
    name: req.name,
  };

  // 2. Ejecutar el command
  let result = commands::create_user::execute(&state, cmd).await?;

  // 3. Traducir resultado → DTO HTTP
  Ok((
    StatusCode::CREATED,
    Json(CreateUserResponse {
      user_id: result.user_id,
    }),
  ))
}

/// Handler de la query: obtiene un usuario.
async fn get_user_handler(State(state): State<AppState>,Path(user_id): Path<Uuid>,) -> Result<Json<UserResponse>,AppError> {
  let query = GetUserQuery { user_id };
  let user = queries::get_user::execute(&state, query).await?;
  Ok(Json(UserResponse::from(user)))
}
