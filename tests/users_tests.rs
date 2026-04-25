//! users_tests.rs — Testes de integração para endpoints de usuários.

mod common;

use common::{
    cleanup_database, create_test_app, execute_request,
    request_helpers::*,
    database::*,
};
use serde_json::json;
use uuid::Uuid;

async fn create_authenticated_user(app: &axum::Router) -> (Uuid, String) {
    // Register and login to get token
    let register_request = post_json("/auth/register", &json!({
        "email": "test@example.com",
        "password": "12345678"
    }));
    let (_, register_response) = execute_request(app, register_request).await;
    
    let token = extract_access_token(&register_response).to_string();
    
    // Get user ID by decoding the response or making a /users/me request
    let me_request = get_request_with_auth("/users/me", &token);
    let (_, me_response) = execute_request(app, me_request).await;
    let user_id = Uuid::parse_str(me_response["id"].as_str().unwrap()).unwrap();
    
    (user_id, token)
}

#[tokio::test]
async fn test_get_me_success() {
    let (app, pool, _) = create_test_app().await;
    cleanup_database(&pool).await;

    let (_, token) = create_authenticated_user(&app).await;

    let request = get_request_with_auth("/users/me", &token);
    let (status, response) = execute_request(&app, request).await;
    
    assert_eq!(status, 200);
    assert_eq!(response["email"], "test@example.com");
    assert!(response["id"].is_string());
    assert!(response["created_at"].is_string());
    assert!(response["updated_at"].is_string());
    assert!(response["name"].is_null()); // Initially null
}

#[tokio::test]
async fn test_get_me_unauthorized() {
    let (app, pool, _) = create_test_app().await;
    cleanup_database(&pool).await;

    let request = get_request("/users/me");
    let (status, response) = execute_request(&app, request).await;
    
    assert_eq!(status, 401);
    assert!(response["error"].as_str().unwrap().contains("Token não fornecido"));
}

#[tokio::test]
async fn test_get_me_invalid_token() {
    let (app, pool, _) = create_test_app().await;
    cleanup_database(&pool).await;

    let request = get_request_with_auth("/users/me", "invalid-token");
    let (status, response) = execute_request(&app, request).await;
    
    assert_eq!(status, 401);
    assert!(response["error"].is_string());
}

#[tokio::test]
async fn test_get_user_by_id() {
    let (app, pool, _) = create_test_app().await;
    cleanup_database(&pool).await;

    let (user_id, token) = create_authenticated_user(&app).await;

    let request = get_request_with_auth(&format!("/users/{}", user_id), &token);
    let (status, response) = execute_request(&app, request).await;
    
    assert_eq!(status, 200);
    assert_eq!(response["email"], "test@example.com");
    assert_eq!(response["id"], user_id.to_string());
}

#[tokio::test]
async fn test_get_user_not_found() {
    let (app, pool, _) = create_test_app().await;
    cleanup_database(&pool).await;

    let (_, token) = create_authenticated_user(&app).await;
    let nonexistent_id = Uuid::new_v4();

    let request = get_request_with_auth(&format!("/users/{}", nonexistent_id), &token);
    let (status, response) = execute_request(&app, request).await;
    
    assert_eq!(status, 404);
    assert!(response["error"].is_string());
}

#[tokio::test]
async fn test_update_user_name_success() {
    let (app, pool, _) = create_test_app().await;
    cleanup_database(&pool).await;

    let (user_id, token) = create_authenticated_user(&app).await;

    let request = put_json_with_auth("/users/me", &json!({
        "name": "João Silva"
    }), &token);
    let (status, response) = execute_request(&app, request).await;
    
    assert_eq!(status, 200);
    assert_eq!(response["name"], "João Silva");
    assert_eq!(response["email"], "test@example.com");
    
    // Verify in database
    let (email, name) = get_user_by_id(&pool, user_id).await.unwrap();
    assert_eq!(email, "test@example.com");
    assert_eq!(name, Some("João Silva".to_string()));
}

#[tokio::test]
async fn test_update_user_email_success() {
    let (app, pool, _) = create_test_app().await;
    cleanup_database(&pool).await;

    let (user_id, token) = create_authenticated_user(&app).await;

    let request = put_json_with_auth("/users/me", &json!({
        "email": "newemail@example.com"
    }), &token);
    let (status, response) = execute_request(&app, request).await;
    
    assert_eq!(status, 200);
    assert_eq!(response["email"], "newemail@example.com");
    
    // Verify in database
    let (email, _) = get_user_by_id(&pool, user_id).await.unwrap();
    assert_eq!(email, "newemail@example.com");
}

#[tokio::test]
async fn test_update_user_password_success() {
    let (app, pool, _) = create_test_app().await;
    cleanup_database(&pool).await;

    let (_, token) = create_authenticated_user(&app).await;

    let request = put_json_with_auth("/users/me", &json!({
        "password": "newpassword123",
        "current_password": "12345678"
    }), &token);
    let (status, response) = execute_request(&app, request).await;
    
    assert_eq!(status, 200);
    
    // Test login with new password
    let login_request = post_json("/auth/login", &json!({
        "email": "test@example.com",
        "password": "newpassword123"
    }));
    let (login_status, _) = execute_request(&app, login_request).await;
    assert_eq!(login_status, 200);
}

#[tokio::test]
async fn test_update_user_password_wrong_current() {
    let (app, pool, _) = create_test_app().await;
    cleanup_database(&pool).await;

    let (_, token) = create_authenticated_user(&app).await;

    let request = put_json_with_auth("/users/me", &json!({
        "password": "newpassword123",
        "current_password": "wrongpassword"
    }), &token);
    let (status, response) = execute_request(&app, request).await;
    
    assert_eq!(status, 422);
    assert!(response["error"].as_str().unwrap().contains("Senha atual incorreta"));
}

#[tokio::test]
async fn test_update_user_password_missing_current() {
    let (app, pool, _) = create_test_app().await;
    cleanup_database(&pool).await;

    let (_, token) = create_authenticated_user(&app).await;

    let request = put_json_with_auth("/users/me", &json!({
        "password": "newpassword123"
    }), &token);
    let (status, response) = execute_request(&app, request).await;
    
    assert_eq!(status, 422);
    assert!(response["error"].as_str().unwrap().contains("Senha atual é obrigatória"));
}

#[tokio::test]
async fn test_update_user_multiple_fields() {
    let (app, pool, _) = create_test_app().await;
    cleanup_database(&pool).await;

    let (user_id, token) = create_authenticated_user(&app).await;

    let request = put_json_with_auth("/users/me", &json!({
        "name": "Maria Santos",
        "email": "maria@example.com"
    }), &token);
    let (status, response) = execute_request(&app, request).await;
    
    assert_eq!(status, 200);
    assert_eq!(response["name"], "Maria Santos");
    assert_eq!(response["email"], "maria@example.com");
    
    // Verify in database
    let (email, name) = get_user_by_id(&pool, user_id).await.unwrap();
    assert_eq!(email, "maria@example.com");
    assert_eq!(name, Some("Maria Santos".to_string()));
}

#[tokio::test]
async fn test_update_user_invalid_email() {
    let (app, pool, _) = create_test_app().await;
    cleanup_database(&pool).await;

    let (_, token) = create_authenticated_user(&app).await;

    let request = put_json_with_auth("/users/me", &json!({
        "email": "invalid-email"
    }), &token);
    let (status, response) = execute_request(&app, request).await;
    
    assert_eq!(status, 422);
    assert!(response["error"].as_str().unwrap().contains("E-mail inválido"));
}

#[tokio::test]
async fn test_update_user_unauthorized() {
    let (app, pool, _) = create_test_app().await;
    cleanup_database(&pool).await;

    let request = put_json_with_auth("/users/me", &json!({
        "name": "Unauthorized User"
    }), "invalid-token");
    let (status, response) = execute_request(&app, request).await;
    
    assert_eq!(status, 401);
    assert!(response["error"].is_string());
}