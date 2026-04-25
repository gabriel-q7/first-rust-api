//! modules/users/service.rs — Regras de negócio do módulo de usuários.

use argon2::{password_hash::PasswordHash, password_hash::PasswordVerifier, Argon2};
use uuid::Uuid;

use crate::{db::user_repository, error::AppError, state::AppState};

use super::dto::{UpdateUserRequest, UserResponse};

/// Retorna os dados do usuário pelo ID.
/// Usado tanto em GET /users/me quanto em GET /users/:id.
pub async fn get_user_by_id(state: &AppState, id: Uuid) -> Result<UserResponse, AppError> {
    let user = user_repository::find_user_by_id(&state.db, id)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(UserResponse {
        id: user.id,
        email: user.email,
        name: user.name,
        created_at: user.created_at,
        updated_at: user.updated_at,
    })
}

/// Atualiza os dados do usuário.
/// Valida entrada e verifica senha atual se necessário.
pub async fn update_user(
    state: &AppState,
    user_id: Uuid,
    request: UpdateUserRequest,
) -> Result<UserResponse, AppError> {
    // Busca o usuário atual
    let current_user = user_repository::find_user_by_id(&state.db, user_id)
        .await?
        .ok_or(AppError::NotFound)?;

    // Valida campos opcionais
    if let Some(ref email) = request.email {
        validate_email(email)?;
    }

    if let Some(ref password) = request.password {
        validate_password(password)?;
        
        // Se está alterando senha, deve fornecer a senha atual
        if let Some(ref current_password) = request.current_password {
            // Verifica se a senha atual está correta
            let password_hash = PasswordHash::new(&current_user.password_hash)
                .map_err(|_| AppError::Validation("Hash de senha inválido".into()))?;
            
            let is_valid = Argon2::default()
                .verify_password(current_password.as_bytes(), &password_hash)
                .is_ok();
                
            if !is_valid {
                return Err(AppError::Validation("Senha atual incorreta".into()));
            }
        } else {
            return Err(AppError::Validation("Senha atual é obrigatória para alterar a senha".into()));
        }
    }

    // Atualiza no banco de dados
    let updated_user = user_repository::update_user(&state.db, user_id, &request).await?;

    Ok(UserResponse {
        id: updated_user.id,
        email: updated_user.email,
        name: updated_user.name,
        created_at: updated_user.created_at,
        updated_at: updated_user.updated_at,
    })
}

/// Valida formato do e-mail.
fn validate_email(email: &str) -> Result<(), AppError> {
    if !email.contains('@') || email.is_empty() {
        return Err(AppError::Validation("E-mail inválido".into()));
    }
    Ok(())
}

/// Valida se a senha tem ao menos 8 caracteres.
fn validate_password(password: &str) -> Result<(), AppError> {
    if password.len() < 8 {
        return Err(AppError::Validation("A senha deve ter pelo menos 8 caracteres".into()));
    }
    Ok(())
}
