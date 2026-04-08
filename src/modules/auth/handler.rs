//! modules/auth/handler.rs — Handlers HTTP para o módulo de autenticação.
//!
//! Os handlers recebem a requisição, extraem os dados e delegam ao service.
//! Utilizamos `Json` do Axum para extrair o body JSON automaticamente.

use axum::{extract::State, http::StatusCode, Json};

use crate::{error::AppError, state::AppState};

use super::{
    dto::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse},
    service,
};

/// Handler para POST /auth/register
///
/// Recebe os dados de registro, cria o usuário e retorna os dados básicos.
pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<RegisterResponse>), AppError> {
    let response = service::register(&state, body).await?;
    // Retornamos 201 Created para indicar que um novo recurso foi criado.
    Ok((StatusCode::CREATED, Json(response)))
}

/// Handler para POST /auth/login
///
/// Verifica as credenciais e retorna um JWT se forem válidas.
pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let response = service::login(&state, &body.email, &body.password).await?;
    Ok(Json(response))
}
