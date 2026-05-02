// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Wiremock smoke tests for `kei-auth-google`. No live HTTP — every
//! assertion is local to the test process.

use kei_auth_google::{GoogleAuthClient, GoogleAuthProvider};
use kei_runtime_core::traits::auth::{AuthChallenge, AuthProvider};
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
async fn verify_end_to_end_builds_auth_session() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "access_token": "tok",
            "expires_in": 1800,
            "id_token": null
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/userinfo"))
        .and(header("authorization", "Bearer tok"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "sub": "999",
            "email": "bob@example.com",
            "name": "Bob"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = client_for(&server);
    let provider = GoogleAuthProvider::new(client, None).unwrap();
    let challenge = AuthChallenge::OAuthCode {
        provider: "google".into(),
        code: "code-xyz".into(),
        state: "csrf-state-xyz".into(),
        expected_state: "csrf-state-xyz".into(),
    };
    let session = provider.verify(&challenge).await.unwrap();
    assert_eq!(session.user_id, "bob@example.com");
    assert_eq!(session.dna.role(), "session");
    assert!(session.dna.caps().contains("UI"));
    assert_eq!(session.parent_dna.role(), "primitive");
    assert!(session.parent_dna.caps().contains("GO"));
    assert!(session.expires_unix_ms > 0);
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

#[tokio::test]
async fn issue_challenge_rejects_non_oauth() {
    let client = GoogleAuthClient::with_urls(
        "http://t/x", "http://u/x", "cid", "secret", "http://r/cb",
    ).unwrap();
    let provider = GoogleAuthProvider::new(client, None).unwrap();
    let challenge = AuthChallenge::MagicLink { email: "a@b.c".into() };
    assert!(provider.issue_challenge(&challenge).await.is_err());
}

#[tokio::test]
async fn verify_rejects_wrong_provider() {
    let client = GoogleAuthClient::with_urls(
        "http://t/x", "http://u/x", "cid", "secret", "http://r/cb",
    ).unwrap();
    let provider = GoogleAuthProvider::new(client, None).unwrap();
    let challenge = AuthChallenge::OAuthCode {
        provider: "github".into(),
        code: "x".into(),
        state: "y".into(),
        expected_state: "y".into(),
    };
    assert!(provider.verify(&challenge).await.is_err());
}

#[tokio::test]
async fn verify_rejects_csrf_state_mismatch() {
    let server = MockServer::start().await;
    let client = GoogleAuthClient::with_urls(
        format!("{}/token", server.uri()),
        format!("{}/userinfo", server.uri()),
        "cid", "secret", "https://example.com/cb",
    ).unwrap();
    let provider = GoogleAuthProvider::new(client, None).unwrap();
    let challenge = AuthChallenge::OAuthCode {
        provider: "google".into(),
        code: "code".into(),
        state: "got-this-state".into(),
        expected_state: "expected-state".into(),
    };
    let err = provider.verify(&challenge).await.unwrap_err();
    assert!(
        format!("{err}").contains("CSRF"),
        "expected CSRF error, got: {err}"
    );
}
