use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::models::User;

/// DTO de respuesta para crear un usuario.
/// En CQRS, el command devuelve solo el ID.
#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    pub user_id: Uuid,
}

/// DTO de respuesta para leer un usuario.
/// Este es el "read model" del lado de lectura.
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            email: u.email,
            name: u.name,
            created_at: u.created_at,
            updated_at: u.updated_at,
        }
    }
}
