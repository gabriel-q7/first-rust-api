//! state.rs — Define o estado compartilhado da aplicação (AppState).
//!
//! O `AppState` é clonado e injetado em cada handler pelo Axum.
//! Ele carrega a conexão com o banco e as configurações.

use sqlx::PgPool;
use crate::config::Config;

/// Estado compartilhado entre todos os handlers da aplicação.
///
/// Derivamos `Clone` pois o Axum precisa clonar o estado para cada requisição.
/// Como `PgPool` já usa `Arc` internamente, clonar é barato.
#[derive(Clone)]
pub struct AppState {
    /// Pool de conexões com o PostgreSQL.
    pub db: PgPool,
    /// Configurações da aplicação (JWT secret, expiração, etc.).
    pub config: Config,
}

impl AppState {
    pub fn new(db: PgPool, config: Config) -> Self {
        Self { db, config }
    }
}
