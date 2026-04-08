//! middleware/auth.rs — Extractor de autenticação JWT para o Axum.
//!
//! `AuthUser` é um extractor customizado que valida o token Bearer do header
//! `Authorization` e injeta o `user_id` do token no handler.
//!
//! # Como funciona um Extractor no Axum
//! Implementamos `FromRequestParts` para que o Axum chame automaticamente
//! `AuthUser::from_request_parts(...)` antes de invocar o handler.
//! Se a validação falhar, o handler nem é executado.

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, HeaderMap},
    RequestPartsExt,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::AppError, state::AppState};

/// Claims armazenadas dentro do token JWT.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// `sub` (subject): ID do usuário.
    pub sub: String,
    /// `exp` (expiration): timestamp Unix de expiração.
    pub exp: u64,
    /// `iat` (issued at): timestamp Unix de emissão.
    pub iat: u64,
}

/// Extractor que representa um usuário autenticado.
///
/// Pode ser adicionado como parâmetro em qualquer handler que precise de autenticação.
///
/// ```rust
/// async fn protected_handler(auth: AuthUser) -> String {
///     format!("Olá, usuário {}!", auth.user_id)
/// }
/// ```
#[derive(Debug)]
pub struct AuthUser {
    /// ID do usuário extraído do token JWT.
    pub user_id: Uuid,
}

/// Gera um token JWT para o usuário informado.
pub fn create_token(user_id: Uuid, secret: &str, expires_in: u64) -> Result<String, AppError> {
    use jsonwebtoken::{encode, EncodingKey, Header};
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| AppError::Internal(e.to_string()))?
        .as_secs();

    let claims = Claims {
        sub: user_id.to_string(),
        iat: now,
        exp: now + expires_in,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("Erro ao gerar token: {e}")))
}

/// Valida um token JWT e retorna os claims.
pub fn validate_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppError::InvalidToken)?;

    Ok(token_data.claims)
}

/// Extrai o token Bearer do header Authorization.
fn extract_bearer_token(headers: &HeaderMap) -> Option<&str> {
    let auth_header = headers.get(axum::http::header::AUTHORIZATION)?.to_str().ok()?;
    auth_header.strip_prefix("Bearer ")
}

/// Implementação do extractor customizado para o Axum.
///
/// O Axum chama este método automaticamente para cada handler que recebe `AuthUser`.
#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Extrai os headers da requisição.
        let headers = parts.extract::<HeaderMap>().await.unwrap_or_default();

        // Tenta encontrar o token no header Authorization.
        let token = extract_bearer_token(&headers).ok_or(AppError::InvalidToken)?;

        // Valida o token e extrai os claims.
        let claims = validate_token(token, &state.config.jwt_secret)?;

        // Converte o `sub` (string) de volta para UUID.
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::InvalidToken)?;

        Ok(AuthUser { user_id })
    }
}
