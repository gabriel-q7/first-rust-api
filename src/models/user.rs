//! models/user.rs — Estrutura que representa um usuário no banco de dados.
//!
//! Esta struct é usada pelo SQLx para mapear as linhas da tabela `users`.

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Representa um usuário completo conforme armazenado no banco.
///
/// `#[derive(sqlx::FromRow)]` permite que o SQLx preencha esta struct
/// automaticamente a partir dos resultados de uma query.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    /// Identificador único do usuário (UUID v4).
    pub id: Uuid,
    /// E-mail único do usuário.
    pub email: String,
    /// Hash Argon2 da senha (nunca expor em respostas!).
    #[serde(skip_serializing)]
    pub password_hash: String,
    /// Data/hora de criação do registro.
    pub created_at: NaiveDateTime,
    /// Data/hora da última atualização do registro.
    pub updated_at: NaiveDateTime,
}
