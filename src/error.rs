//! error.rs — Centraliza os erros da aplicação.
//!
//! `AppError` é o tipo de erro global. Ele implementa `IntoResponse` do Axum,
//! permitindo que handlers retornem `Result<T, AppError>` diretamente.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

/// Enumeração de todos os erros possíveis na aplicação.
#[derive(Debug, Error)]
pub enum AppError {
    /// Erro interno do banco de dados.
    #[error("Erro no banco de dados: {0}")]
    Database(#[from] sqlx::Error),

    /// Recurso não encontrado.
    #[error("Não encontrado")]
    NotFound,

    /// Credenciais inválidas (e-mail ou senha incorretos).
    #[error("Credenciais inválidas")]
    Unauthorized,

    /// Token JWT inválido ou expirado.
    #[error("Token inválido ou expirado")]
    InvalidToken,

    /// Dados de entrada inválidos.
    #[error("Dados inválidos: {0}")]
    Validation(String),

    /// E-mail já cadastrado.
    #[error("E-mail já está em uso")]
    #[allow(dead_code)]
    EmailAlreadyExists,

    /// Erro interno do servidor (inesperado).
    #[error("Erro interno: {0}")]
    Internal(String),
}

/// Converte `AppError` em uma resposta HTTP com JSON e status code adequados.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Mapeamos cada variante para um status code e mensagem.
        let (status, message) = match &self {
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::InvalidToken => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg.clone()),
            AppError::EmailAlreadyExists => (StatusCode::CONFLICT, self.to_string()),
            AppError::Database(e) => {
                // Detecta violação de constraint UNIQUE (e-mail duplicado).
                if let sqlx::Error::Database(db_err) = e {
                    if db_err.is_unique_violation() {
                        return (
                            StatusCode::CONFLICT,
                            Json(json!({ "error": "E-mail já está em uso" })),
                        )
                            .into_response();
                    }
                }
                tracing::error!("Erro no banco de dados: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Erro interno".to_string())
            }
            AppError::Internal(msg) => {
                tracing::error!("Erro interno: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg.clone())
            }
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
