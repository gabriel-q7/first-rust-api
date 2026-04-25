//! common/mod.rs — Utilitários comuns para testes de integração.

use axum::{body::Body, http::Request, response::Response};
use first_rust_api::{
    config::Config,
    state::AppState,
    modules::{auth, users},
};
use serde_json::Value;
use sqlx::{PgPool, Row};
use std::env;
use tower::{Service, ServiceExt};
use uuid::Uuid;

pub mod database;
pub mod request_helpers;

/// Cria uma instância de teste da aplicação com banco de dados de teste.
pub async fn create_test_app() -> (axum::Router, PgPool, Config) {
    // Setup test database URL
    let test_db_url = env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/test_first_rust_api".to_string());
    
    // Create database pool
    let pool = PgPool::connect(&test_db_url)
        .await
        .expect("Failed to connect to test database");
    
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations on test database");
    
    // Create test config
    let config = Config {
        database_url: test_db_url,
        jwt_secret: "test-secret-key".to_string(),
        jwt_expires_in: 86400,
        server_addr: "0.0.0.0:3000".to_string(),
    };
    
    // Create app state
    let state = AppState {
        db: pool.clone(),
        config: config.clone(),
    };
    
    // Create router
    let app = axum::Router::new()
        .route("/health", axum::routing::get(|| async { "OK" }))
        .nest("/auth", auth::router())
        .nest("/users", users::router())
        .with_state(state);
    
    (app, pool, config)
}

/// Executa uma request e retorna a resposta como JSON.
pub async fn execute_request(
    app: &axum::Router,
    request: Request<Body>,
) -> (u16, Value) {
    let mut service = app.clone();
    
    let response: Response = ServiceExt::<Request<Body>>::ready(&mut service)
        .await
        .expect("Service not ready")
        .call(request)
        .await
        .expect("Failed to execute request");
    
    let status = response.status().as_u16();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    
    let json: Value = if body.is_empty() {
        serde_json::json!({})
    } else {
        serde_json::from_slice(&body)
            .expect("Failed to parse response as JSON")
    };
    
    (status, json)
}

/// Limpa todos os dados do banco de teste.
pub async fn cleanup_database(pool: &PgPool) {
    sqlx::query("DELETE FROM users")
        .execute(pool)
        .await
        .expect("Failed to clean up test database");
}
