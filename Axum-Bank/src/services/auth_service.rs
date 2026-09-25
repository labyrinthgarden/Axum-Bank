use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::auth::{Claims, User};
use crate::state::AppState;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| AppError::Internal(format!("Hash error: {}", e)))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| AppError::Internal(format!("Invalid hash: {}", e)))?;

    match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(argon2::password_hash::Error::Password) => Ok(false),
        Err(e) => Err(AppError::Internal(format!("Verify error: {}", e))),
    }
}

pub fn generate_token(user_id: &str, secret: &str) -> Result<String, AppError> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| AppError::Internal(e.to_string()))?
        .as_secs() as usize
        + 3600; // 1 hora

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("JWT error: {}", e)))
}

pub async fn register_user(
    state: &AppState,
    username: String,
    email: String,
    password: String,
) -> Result<User, AppError> {
    let mut users = state.users.write().await;

    if users.contains_key(&username) {
        return Err(AppError::Conflict("Username already taken".into()));
    }

    let password_hash = hash_password(&password)?;
    let user = User {
        id: Uuid::new_v4().to_string(),
        username: username.clone(),
        email,
        password_hash,
    };

    users.insert(username, user.clone());
    Ok(user)
}

pub async fn authenticate(
    state: &AppState,
    username: &str,
    password: &str,
) -> Result<User, AppError> {
    let users = state.users.read().await;

    let user = users
        .get(username)
        .ok_or_else(|| AppError::Unauthorized("Invalid credentials".into()))?;

    if !verify_password(password, &user.password_hash)? {
        return Err(AppError::Unauthorized("Invalid credentials".into()));
    }

    Ok(user.clone())
}
