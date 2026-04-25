//! db/user_repository.rs — Acesso ao banco de dados para a entidade User.
//!
//! Todas as queries SQL ficam aqui, separadas da lógica de negócio (service).
//! Utilizamos `sqlx::query_as!` para queries com verificação em tempo de compilação,
//! e `sqlx::query_as` para queries dinâmicas (útil quando não há banco em CI).

use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::AppError, models::user::User, modules::users::dto::UpdateUserRequest};

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
        RETURNING id, email, name, password_hash, created_at, updated_at
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
        SELECT id, email, name, password_hash, created_at, updated_at
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
        SELECT id, email, name, password_hash, created_at, updated_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

/// Atualiza dados do usuário no banco.
/// Permite atualizações seletivas - apenas campos fornecidos são atualizados.
pub async fn update_user(
    pool: &PgPool,
    user_id: Uuid,
    request: &UpdateUserRequest,
) -> Result<User, AppError> {
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };

    // Hash password if needed
    let password_hash = if let Some(ref password) = request.password {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| AppError::Validation("Erro ao gerar hash da senha".into()))?
            .to_string();
        Some(hash)
    } else {
        None
    };

    // Build dynamic update query
    let mut set_clauses = vec!["updated_at = NOW()".to_string()];
    let mut param_count = 1;

    if request.email.is_some() {
        set_clauses.push(format!("email = ${}", param_count));
        param_count += 1;
    }

    if request.name.is_some() {
        set_clauses.push(format!("name = ${}", param_count));
        param_count += 1;
    }

    if password_hash.is_some() {
        set_clauses.push(format!("password_hash = ${}", param_count));
        param_count += 1;
    }

    let query = format!(
        "UPDATE users SET {} WHERE id = ${} RETURNING id, email, name, password_hash, created_at, updated_at",
        set_clauses.join(", "),
        param_count
    );

    // Bind parameters dynamically
    let mut sql_query = sqlx::query_as::<_, User>(&query);
    
    if let Some(ref email) = request.email {
        sql_query = sql_query.bind(email);
    }
    
    if let Some(ref name) = request.name {
        sql_query = sql_query.bind(name);
    }
    
    if let Some(ref hash) = password_hash {
        sql_query = sql_query.bind(hash);
    }
    
    sql_query = sql_query.bind(user_id);

    let user = sql_query.fetch_one(pool).await?;

    Ok(user)
}
