//! modules/auth/dto.rs — Data Transfer Objects para o módulo de autenticação.
//!
//! DTOs (Data Transfer Objects) definem a estrutura dos dados que entram
//! e saem dos endpoints. Usamos `serde` para (de)serializar JSON.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

/// Payload recebido no endpoint POST /auth/register.
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    /// E-mail do usuário (deve ser válido e único).
    pub email: String,
    /// Senha em texto puro (será hashed antes de salvar).
    pub password: String,
}

/// Payload recebido no endpoint POST /auth/login.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Resposta retornada após login bem-sucedido.
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    /// Token JWT gerado.
    pub access_token: String,
    /// Tipo do token (sempre "Bearer").
    pub token_type: String,
}

/// Resposta retornada após registro bem-sucedido.
/// Inclui o token JWT para autenticação imediata.
#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    /// Token JWT gerado.
    pub access_token: String,
    /// Tipo do token (sempre "Bearer").
    pub token_type: String,
}
