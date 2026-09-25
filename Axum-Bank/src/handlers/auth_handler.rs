use axum::{extract::State, Json};

use crate::error::AppError;
use crate::models::auth::{
    AuthResponse, LoginRequest, RegisterRequest, UserResponse,
};
use crate::services::auth_service;
use crate::state::AppState;

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    if payload.username.trim().is_empty() || payload.password.len() < 6 {
        return Err(AppError::BadRequest(
            "Username required and password must be at least 6 characters".into(),
        ));
    }

    let user = auth_service::register_user(
        &state,
        payload.username.clone(),
        payload.email,
        payload.password,
    )
    .await?;

    let token = auth_service::generate_token(&user.id, &state.jwt_secret)?;

    Ok(Json(AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
        },
    }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = auth_service::authenticate(&state, &payload.username, &payload.password).await?;

    let token = auth_service::generate_token(&user.id, &state.jwt_secret)?;

    Ok(Json(AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
        },
    }))
}
