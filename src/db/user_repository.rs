//! db/user_repository.rs — Acesso ao banco de dados para a entidade User.
//!
//! Todas as queries SQL ficam aqui, separadas da lógica de negócio (service).
//! Utilizamos `sqlx::query_as!` para queries com verificação em tempo de compilação,
//! e `sqlx::query_as` para queries dinâmicas (útil quando não há banco em CI).

use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::AppError, models::user::User};

/// Insere um novo usuário no banco e retorna o registro criado.
pub async fn create_user(
    pool: &PgPool,
    id: Uuid,
    email: &str,
    password_hash: &str,
) -> Result<User, AppError> {
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, email, password_hash, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(email)
    .bind(password_hash)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

/// Busca um usuário pelo e-mail. Retorna `None` se não encontrar.
pub async fn find_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, AppError> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, password_hash, created_at, updated_at
        FROM users
        WHERE email = $1
        "#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

/// Busca um usuário pelo ID. Retorna `None` se não encontrar.
pub async fn find_user_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, AppError> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, password_hash, created_at, updated_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}
