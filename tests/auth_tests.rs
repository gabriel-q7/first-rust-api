//! auth_tests.rs — Testes de integração para endpoints de autenticação.

mod common;

use common::{
    cleanup_database, create_test_app, execute_request,
    request_helpers::*,
    database::*,
};
use serde_json::json;

#[tokio::test]
async fn test_register_success() {
    let (app, pool, _) = create_test_app().await;
    cleanup_database(&pool).await;

    let request = post_json("/auth/register", &json!({
        "email": "test@example.com",
        "password": "12345678"
    }));

    let (status, response) = execute_request(&app, request).await;
    
    assert_eq!(status, 201);
    assert!(response["access_token"].is_string());
    assert_eq!(response["token_type"], "Bearer");
    assert!(user_exists(&pool, "test@example.com").await);
}

#[tokio::test]
async fn test_register_invalid_email() {
    let (app, pool, _) = create_test_app().await;
    cleanup_database(&pool).await;

    let request = post_json("/auth/register", &json!({
        "email": "invalid-email",
        "password": "12345678"
    }));

    let (status, response) = execute_request(&app, request).await;
    
    assert_eq!(status, 422);
    assert!(response["error"].as_str().unwrap().contains("E-mail inválido"));
}

#[tokio::test]
async fn test_register_short_password() {
    let (app, pool, _) = create_test_app().await;
    cleanup_database(&pool).await;

    let request = post_json("/auth/register", &json!({
        "email": "test@example.com",
        "password": "123"
    }));

    let (status, response) = execute_request(&app, request).await;
    
    assert_eq!(status, 422);
    assert!(response["error"].as_str().unwrap().contains("pelo menos 8 caracteres"));
}

#[tokio::test]
async fn test_register_duplicate_email() {
    let (app, pool, _) = create_test_app().await;
    cleanup_database(&pool).await;

    // First registration
    let request1 = post_json("/auth/register", &json!({
        "email": "test@example.com",
        "password": "12345678"
    }));
    let (status1, _) = execute_request(&app, request1).await;
    assert_eq!(status1, 201);

    // Duplicate registration
    let request2 = post_json("/auth/register", &json!({
        "email": "test@example.com",
        "password": "87654321"
    }));
    let (status2, response2) = execute_request(&app, request2).await;
    
    assert_eq!(status2, 409);
    assert!(response2["error"].as_str().unwrap().contains("E-mail já está em uso"));
}

#[tokio::test]
async fn test_login_success() {
    let (app, pool, _) = create_test_app().await;
    cleanup_database(&pool).await;

    // Register user first
    let register_request = post_json("/auth/register", &json!({
        "email": "test@example.com",
        "password": "12345678"
    }));
    execute_request(&app, register_request).await;

    // Login
    let login_request = post_json("/auth/login", &json!({
        "email": "test@example.com",
        "password": "12345678"
    }));

    let (status, response) = execute_request(&app, login_request).await;
    
    assert_eq!(status, 200);
    assert!(response["access_token"].is_string());
    assert_eq!(response["token_type"], "Bearer");
}

#[tokio::test]
async fn test_login_invalid_credentials() {
    let (app, pool, _) = create_test_app().await;
    cleanup_database(&pool).await;

    // Register user first
    let register_request = post_json("/auth/register", &json!({
        "email": "test@example.com",
        "password": "12345678"
    }));
    execute_request(&app, register_request).await;

    // Login with wrong password
    let login_request = post_json("/auth/login", &json!({
        "email": "test@example.com",
        "password": "wrongpassword"
    }));

    let (status, response) = execute_request(&app, login_request).await;
    
    assert_eq!(status, 401);
    assert!(response["error"].as_str().unwrap().contains("Credenciais inválidas"));
}

#[tokio::test]
async fn test_login_nonexistent_user() {
    let (app, pool, _) = create_test_app().await;
    cleanup_database(&pool).await;

    let login_request = post_json("/auth/login", &json!({
        "email": "nonexistent@example.com",
        "password": "12345678"
    }));

    let (status, response) = execute_request(&app, login_request).await;
    
    assert_eq!(status, 401);
    assert!(response["error"].as_str().unwrap().contains("Credenciais inválidas"));
}