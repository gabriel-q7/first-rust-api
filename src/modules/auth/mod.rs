//! modules/auth/ — Módulo de autenticação.
//!
//! Responsável por registro e login de usuários.
//!
//! Estrutura:
//! - `dto`: tipos de entrada/saída (request/response)
//! - `service`: lógica de negócio
//! - `handler`: handlers HTTP (controllers)

pub mod dto;
pub mod handler;
pub mod service;

use axum::{routing::post, Router};
use crate::state::AppState;

/// Cria o roteador do módulo de autenticação.
///
/// Registra as rotas:
/// - POST /auth/register
/// - POST /auth/login
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(handler::register))
        .route("/login", post(handler::login))
}
