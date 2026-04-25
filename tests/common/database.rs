//! database.rs — Utilitários para manipulação do banco de dados em testes.

use sqlx::PgPool;
use sqlx::Row;
use uuid::Uuid;

/// Cria um usuário de teste no banco.
pub async fn create_test_user(
    pool: &PgPool,
    email: &str,
    password_hash: &str,
    name: Option<&str>,
) -> Uuid {
    let id = Uuid::new_v4();
    
    sqlx::query(
        "INSERT INTO users (id, email, password_hash, name) VALUES ($1, $2, $3, $4)"
    )
    .bind(id)
    .bind(email)
    .bind(password_hash)
    .bind(name)
    .execute(pool)
    .await
    .expect("Failed to create test user");
    
    id
}

/// Verifica se um usuário existe no banco.
pub async fn user_exists(pool: &PgPool, email: &str) -> bool {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE email = $1")
        .bind(email)
        .fetch_one(pool)
        .await
        .expect("Failed to check user existence");
    
    count > 0
}

/// Busca dados de um usuário pelo ID.
pub async fn get_user_by_id(
    pool: &PgPool, 
    id: Uuid
) -> Option<(String, Option<String>)> { // (email, name)
    sqlx::query("SELECT email, name FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
        .expect("Failed to fetch user")
        .map(|row| (row.get(0), row.get(1)))
}