//! modules/users/handler.rs — Handlers HTTP para o módulo de usuários.

use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::{error::AppError, middleware::auth::AuthUser, state::AppState};

use super::{dto::UserResponse, service};

/// Handler para GET /users/me
///
/// Rota protegida — requer token JWT válido.
/// O `AuthUser` extractor valida automaticamente o token e injeta o `user_id`.
pub async fn get_me(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<UserResponse>, AppError> {
    let user = service::get_user_by_id(&state, auth.user_id).await?;
    Ok(Json(user))
}

/// Handler para GET /users/:id
///
/// Rota protegida — requer token JWT válido.
/// Retorna os dados de qualquer usuário pelo ID.
pub async fn get_user(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, AppError> {
    let user = service::get_user_by_id(&state, id).await?;
    Ok(Json(user))
}
