//! modules/users/ — Módulo de gerenciamento de usuários.
//!
//! Responsável por consultar dados dos usuários autenticados.
//!
//! Estrutura:
//! - `dto`: tipos de resposta
//! - `service`: lógica de negócio
//! - `handler`: handlers HTTP

pub mod dto;
pub mod handler;
pub mod service;

use axum::{routing::{get, put}, Router};
use crate::state::AppState;

/// Cria o roteador do módulo de usuários.
///
/// Registra as rotas:
/// - GET /users/me  (protegida por JWT via AuthUser extractor)
/// - PUT /users/me  (protegida por JWT via AuthUser extractor)
/// - GET /users/:id (protegida por JWT via AuthUser extractor)
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/me", get(handler::get_me).put(handler::update_user))
        .route("/:id", get(handler::get_user))
}
