//! main.rs — Ponto de entrada da aplicação.
//!
//! Aqui configuramos:
//! 1. Logging (tracing)
//! 2. Configuração (variáveis de ambiente)
//! 3. Conexão com o banco de dados
//! 4. Roteador Axum com todos os módulos
//! 5. Servidor HTTP

mod config;
mod db;
mod error;
mod middleware;
mod models;
mod modules;
mod state;

use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use config::Config;
use state::AppState;

#[tokio::main]
async fn main() {
    // ─── 1. Inicializa o sistema de logging ───────────────────────────────────
    // `RUST_LOG=debug cargo run` para ver logs detalhados.
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "first_rust_api=debug,tower_http=debug".into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // ─── 2. Carrega as configurações do .env ──────────────────────────────────
    let config = Config::from_env();
    tracing::info!("Configuração carregada com sucesso");

    // ─── 3. Conecta ao banco de dados ─────────────────────────────────────────
    // `PgPoolOptions` cria um pool de conexões reutilizáveis.
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .expect("Falha ao conectar ao banco de dados");

    // Executa as migrations automaticamente ao iniciar (cria tabelas se não existirem).
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Falha ao executar migrations");

    tracing::info!("Banco de dados conectado e migrations executadas");

    // ─── 4. Cria o estado compartilhado ──────────────────────────────────────
    let state = AppState::new(pool, config.clone());

    // ─── 5. Configura o roteador ──────────────────────────────────────────────
    let app = Router::new()
        // Rota de health check (Fase 1)
        .route("/health", get(health_check))
        // Módulo de autenticação: /auth/register e /auth/login
        .nest("/auth", modules::auth::router())
        // Módulo de usuários: /users/me e /users/:id
        .nest("/users", modules::users::router())
        // Adiciona middleware de logging de requests/responses
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // ─── 6. Inicia o servidor ─────────────────────────────────────────────────
    let addr: std::net::SocketAddr = config.server_addr.parse().expect("Endereço inválido");
    tracing::info!("Servidor rodando em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.expect("Falha ao iniciar listener");
    axum::serve(listener, app).await.expect("Falha no servidor");
}

/// Handler para GET /health
///
/// Retorna 200 OK com uma mensagem simples.
/// Útil para monitoramento e health checks de infraestrutura.
async fn health_check() -> &'static str {
    "OK"
}
