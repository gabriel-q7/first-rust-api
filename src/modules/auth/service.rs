//! Serviços de autenticação.

use crate::{db::user_repository, modules::auth::dto::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse}};
use crate::error::AppError;
use crate::state::AppState;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordHash, PasswordVerifier, SaltString},
    Argon2,
};
use uuid::Uuid;
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize};
use chrono::{Utc, Duration};

#[derive(Serialize)]
struct Claims {
    sub: String, // subject (user id)
    exp: i64,    // expiration
}

/// Valida se um e-mail tem formato válido.
fn is_valid_email(email: &str) -> bool {
    // Validação simples: contém @ e pelo menos um . após o @
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    let domain = parts[1];
    domain.contains('.') && domain.len() > 3
}

/// Registra um novo usuário.
///
/// # Fluxo:
/// 1. Valida se o e-mail já está em uso
/// 2. Aplica hash na senha
/// 3. Cria o usuário no banco
/// 4. Gera e retorna JWT token
///
/// # Erros:
/// - `BadRequest`: E-mail já está em uso
/// - `Internal`: Erro no banco de dados
pub async fn register(
    state: &AppState,
    req: RegisterRequest,
) -> Result<RegisterResponse, AppError> {
    // --- Validações ---
    // Valida formato do e-mail
    if !is_valid_email(&req.email) {
        return Err(AppError::Validation("E-mail inválido".to_string()));
    }
    
    // Valida tamanho mínimo da senha
    if req.password.len() < 8 {
        return Err(AppError::Validation("A senha deve ter pelo menos 8 caracteres".to_string()));
    }
    
    // Verifica se e-mail já está em uso
    if user_repository::find_user_by_email(&state.db, &req.email).await?.is_some() {
        return Err(AppError::EmailAlreadyExists);
    }

    // --- Hash da senha ---
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(req.password.as_bytes(), &salt)
        .map_err(|_| AppError::Internal("Failed to hash password".to_string()))?
        .to_string();

    // --- Persistência no banco ---
    let id = Uuid::new_v4();
    let user = user_repository::create_user(&state.db, id, &req.email, &password_hash).await?;

    // --- Geração do JWT ---
    let claims = Claims {
        sub: user.id.to_string(),
        exp: (Utc::now() + Duration::hours(24)).timestamp(),
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(b"secret"))
        .map_err(|_| AppError::Internal("Failed to generate token".to_string()))?;

    Ok(RegisterResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
    })
}

/// Realiza login do usuário.
///
/// # Fluxo:
/// 1. Busca o usuário pelo e-mail
/// 2. Verifica a senha
/// 3. Gera e retorna JWT token
///
/// # Erros:
/// - `BadRequest`: E-mail não encontrado ou senha incorreta
/// - `Internal`: Erro no banco de dados
pub async fn login(
    state: &AppState,
    req: LoginRequest,
) -> Result<LoginResponse, AppError> {
    // --- Busca do usuário ---
    let user = user_repository::find_user_by_email(&state.db, &req.email).await?
        .ok_or_else(|| AppError::Unauthorized)?;

    // --- Verificação da senha ---
    let password_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| AppError::Internal("Invalid password hash".to_string()))?;
    
    let password_valid = Argon2::default()
        .verify_password(req.password.as_bytes(), &password_hash)
        .is_ok();
    
    if !password_valid {
        return Err(AppError::Unauthorized);
    }

    // --- Geração do JWT ---
    let claims = Claims {
        sub: user.id.to_string(),
        exp: (Utc::now() + Duration::hours(24)).timestamp(),
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(b"secret"))
        .map_err(|_| AppError::Internal("Failed to generate token".to_string()))?;

    Ok(LoginResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
    })
}

