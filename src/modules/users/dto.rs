//! modules/users/dto.rs — Data Transfer Objects para o módulo de usuários.

use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

/// Resposta com os dados públicos de um usuário.
/// Nunca inclui o hash da senha.
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
