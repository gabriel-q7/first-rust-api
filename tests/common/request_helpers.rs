//! request_helpers.rs — Funções auxiliares para construção de requests.

use axum::{body::Body, http::{HeaderValue, Request}};
use serde::Serialize;
use serde_json::Value;

/// Cria um request POST com JSON.
pub fn post_json<T: Serialize>(uri: &str, body: &T) -> Request<Body> {
    let json_body = serde_json::to_string(body).expect("Failed to serialize JSON");
    
    Request::builder()
        .method("POST")
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(json_body))
        .expect("Failed to build request")
}

/// Cria um request PUT com JSON.
pub fn put_json<T: Serialize>(uri: &str, body: &T) -> Request<Body> {
    let json_body = serde_json::to_string(body).expect("Failed to serialize JSON");
    
    Request::builder()
        .method("PUT")
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(json_body))
        .expect("Failed to build request")
}

/// Cria um request GET.
pub fn get_request(uri: &str) -> Request<Body> {
    Request::builder()
        .method("GET")
        .uri(uri)
        .body(Body::empty())
        .expect("Failed to build request")
}

/// Cria um request GET com token de autorização.
pub fn get_request_with_auth(uri: &str, token: &str) -> Request<Body> {
    Request::builder()
        .method("GET")
        .uri(uri)
        .header("authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .expect("Failed to build request")
}

/// Cria um request PUT com JSON e token de autorização.
pub fn put_json_with_auth<T: Serialize>(uri: &str, body: &T, token: &str) -> Request<Body> {
    let json_body = serde_json::to_string(body).expect("Failed to serialize JSON");
    
    Request::builder()
        .method("PUT")
        .uri(uri)
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer {}", token))
        .body(Body::from(json_body))
        .expect("Failed to build request")
}

/// Extrai o token de acesso de uma resposta de autenticação.
pub fn extract_access_token(response: &Value) -> &str {
    response["access_token"]
        .as_str()
        .expect("Failed to extract access_token from response")
}