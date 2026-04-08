//! modules/users/service.rs — Regras de negócio do módulo de usuários.

use uuid::Uuid;

use crate::{db::user_repository, error::AppError, state::AppState};

use super::dto::UserResponse;

/// Retorna os dados do usuário pelo ID.
/// Usado tanto em GET /users/me quanto em GET /users/:id.
pub async fn get_user_by_id(state: &AppState, id: Uuid) -> Result<UserResponse, AppError> {
    let user = user_repository::find_user_by_id(&state.db, id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(UserResponse {
        id: user.id,
        email: user.email,
        created_at: user.created_at,
        updated_at: user.updated_at,
    })
}
