//! modules/users/dto.rs — Data Transfer Objects para o módulo de usuários.

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Resposta com os dados públicos de um usuário.
/// Nunca inclui o hash da senha.
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Request para atualizar dados do usuário.
/// Todos os campos são opcionais para permitir atualizações parciais.
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    /// Novo e-mail (opcional).
    pub email: Option<String>,
    /// Novo nome (opcional).
    pub name: Option<String>,
    /// Nova senha (opcional).
    pub password: Option<String>,
    /// Senha atual para confirmação (obrigatória se password for fornecido).
    pub current_password: Option<String>,
}
