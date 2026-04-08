//! modules/auth/service.rs — Regras de negócio do módulo de autenticação.
//!
//! O service recebe dados já validados do handler, executa a lógica
//! (hash de senha, verificação, geração de JWT) e chama o repositório.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use uuid::Uuid;

use crate::{
    db::user_repository,
    error::AppError,
    middleware::auth::create_token,
    state::AppState,
};

use super::dto::{LoginResponse, RegisterRequest, RegisterResponse};

/// Registra um novo usuário.
///
/// # Fluxo:
/// 1. Valida e-mail e senha
/// 2. Gera hash Argon2 da senha
/// 3. Salva no banco
/// 4. Retorna os dados básicos (sem hash)
pub async fn register(state: &AppState, req: RegisterRequest) -> Result<RegisterResponse, AppError> {
    // --- Validação dos dados de entrada ---
    validate_email(&req.email)?;
    validate_password(&req.password)?;

    // --- Hash da senha com Argon2 ---
    // Argon2 é considerado o algoritmo mais seguro para hashing de senhas em 2024.
    // Geramos um salt aleatório diferente para cada usuário.
    let password_hash = hash_password(&req.password)?;

    // --- Persistência no banco ---
    let id = Uuid::new_v4();
    let user = user_repository::create_user(&state.db, id, &req.email, &password_hash).await?;

    Ok(RegisterResponse {
        id: user.id,
        email: user.email,
        created_at: user.created_at,
    })
}

/// Realiza login do usuário.
///
/// # Fluxo:
/// 1. Busca o usuário pelo e-mail
/// 2. Verifica a senha contra o hash
/// 3. Gera um JWT
/// 4. Retorna o token
pub async fn login(state: &AppState, email: &str, password: &str) -> Result<LoginResponse, AppError> {
    // --- Busca o usuário no banco ---
    let user = user_repository::find_user_by_email(&state.db, email)
        .await?
        .ok_or(AppError::Unauthorized)?;

    // --- Verifica a senha ---
    verify_password(password, &user.password_hash)?;

    // --- Gera o token JWT ---
    let token = create_token(user.id, &state.config.jwt_secret, state.config.jwt_expires_in)?;

    Ok(LoginResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
    })
}

// ─── Funções auxiliares privadas ─────────────────────────────────────────────

/// Valida o formato básico do e-mail.
fn validate_email(email: &str) -> Result<(), AppError> {
    let email = email.trim();
    if email.is_empty() {
        return Err(AppError::Validation("E-mail é obrigatório".to_string()));
    }
    // Validação simples: deve conter @ e pelo menos um ponto após o @.
    if !email.contains('@') || email.split('@').nth(1).map_or(true, |d| !d.contains('.')) {
        return Err(AppError::Validation("Formato de e-mail inválido".to_string()));
    }
    Ok(())
}

/// Valida o tamanho mínimo da senha.
fn validate_password(password: &str) -> Result<(), AppError> {
    if password.len() < 8 {
        return Err(AppError::Validation(
            "A senha deve ter no mínimo 8 caracteres".to_string(),
        ));
    }
    Ok(())
}

/// Gera o hash Argon2 de uma senha em texto puro.
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| AppError::Internal(format!("Erro ao gerar hash: {e}")))
}

/// Verifica se a senha corresponde ao hash armazenado.
pub fn verify_password(password: &str, hash: &str) -> Result<(), AppError> {
    let parsed_hash =
        PasswordHash::new(hash).map_err(|e| AppError::Internal(format!("Hash inválido: {e}")))?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Unauthorized)
}

// ─── Testes unitários ─────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email_valid() {
        assert!(validate_email("user@example.com").is_ok());
        assert!(validate_email("gabriel@email.com").is_ok());
    }

    #[test]
    fn test_validate_email_invalid() {
        assert!(validate_email("").is_err());
        assert!(validate_email("notanemail").is_err());
        assert!(validate_email("missing@dot").is_err());
    }

    #[test]
    fn test_validate_password_valid() {
        assert!(validate_password("12345678").is_ok());
        assert!(validate_password("strongpassword").is_ok());
    }

    #[test]
    fn test_validate_password_too_short() {
        assert!(validate_password("1234567").is_err());
        assert!(validate_password("").is_err());
    }

    #[test]
    fn test_hash_and_verify_password() {
        let password = "mypassword123";
        let hash = hash_password(password).expect("hash deve funcionar");
        assert!(verify_password(password, &hash).is_ok());
        assert!(verify_password("wrongpassword", &hash).is_err());
    }
}
