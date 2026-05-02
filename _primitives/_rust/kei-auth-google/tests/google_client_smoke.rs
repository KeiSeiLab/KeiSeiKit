// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Wiremock smoke tests for `GoogleAuthClient` HTTP layer. No live HTTP.

use kei_auth_google::GoogleAuthClient;
use serde_json::json;
use wiremock::matchers::{body_string_contains, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn client_for(server: &MockServer) -> GoogleAuthClient {
    GoogleAuthClient::with_urls(
        format!("{}/token", server.uri()),
        format!("{}/userinfo", server.uri()),
        "client-id-xyz",
        "client-secret-xyz",
        "https://example.com/cb",
    )
    .unwrap()
}

#[tokio::test]
async fn token_endpoint_200_returns_access_token() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/token"))
        .and(body_string_contains("grant_type=authorization_code"))
        .and(body_string_contains("code=abc123"))
        .and(body_string_contains("client_id=client-id-xyz"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "access_token": "ya29.a0AfH-test",
            "expires_in": 3600,
            "id_token": "eyJ.fake.jwt"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = client_for(&server);
    let token = client.exchange_code("abc123", None).await.unwrap();
    assert_eq!(token.access_token, "ya29.a0AfH-test");
    assert_eq!(token.expires_in, 3600);
    assert_eq!(token.id_token.as_deref(), Some("eyJ.fake.jwt"));
}

#[tokio::test]
async fn userinfo_200_returns_email_and_sub() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/userinfo"))
        .and(header("authorization", "Bearer ya29.a0AfH-test"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "sub": "1234567890",
            "email": "alice@example.com",
            "name": "Alice"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = client_for(&server);
    let info = client.userinfo("ya29.a0AfH-test").await.unwrap();
    assert_eq!(info.sub, "1234567890");
    assert_eq!(info.email, "alice@example.com");
    assert_eq!(info.name, "Alice");
}

#[tokio::test]
async fn exchange_code_400_returns_api_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/token"))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
            "error": "invalid_grant",
            "error_description": "Bad code"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = client_for(&server);
    let err = client.exchange_code("bad-code", None).await.unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("api"), "expected api variant, got {msg}");
    assert!(msg.contains("400"), "expected status 400 in message, got {msg}");
}
