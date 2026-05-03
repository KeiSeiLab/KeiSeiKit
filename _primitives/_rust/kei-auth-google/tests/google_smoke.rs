// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Wiremock smoke tests for `GoogleAuthProvider`. No live HTTP.
//!
//! CVE-2023-7028 class regressions live in
//! `tests/google_security_regression.rs`.

use kei_auth_google::{GoogleAuthClient, GoogleAuthProvider};
use kei_runtime_core::traits::auth::{AuthChallenge, AuthProvider, AuthSession};
use serde_json::{json, Value};
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

fn challenge(state: &str, code_verifier: Option<&str>) -> AuthChallenge {
    AuthChallenge::OAuthCode {
        provider: "google".into(),
        code: "code".into(),
        state: state.into(),
        expected_state: state.into(),
        code_verifier: code_verifier.map(str::to_string),
    }
}

async fn mock_token(server: &MockServer, body: Value) {
    Mock::given(method("POST"))
        .and(path("/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .expect(1)
        .mount(server)
        .await;
}

async fn mock_userinfo(server: &MockServer, body: Value) {
    Mock::given(method("GET"))
        .and(path("/userinfo"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .expect(1)
        .mount(server)
        .await;
}

async fn run_verify(
    server: &MockServer,
    state: &str,
    code_verifier: Option<&str>,
) -> kei_runtime_core::Result<AuthSession> {
    let provider = GoogleAuthProvider::new(client_for(server), None).unwrap();
    provider.verify(&challenge(state, code_verifier)).await
}

#[tokio::test]
async fn verify_end_to_end_builds_auth_session() {
    let server = MockServer::start().await;
    mock_token(&server, json!({
        "access_token": "tok", "expires_in": 1800, "id_token": null
    })).await;
    Mock::given(method("GET"))
        .and(path("/userinfo"))
        .and(header("authorization", "Bearer tok"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "sub": "999", "email": "bob@example.com",
            "email_verified": true, "name": "Bob"
        })))
        .expect(1)
        .mount(&server)
        .await;
    let session = run_verify(&server, "csrf-state-xyz", None).await.unwrap();
    // Post CVE-2023-7028 fix: user_id is the OIDC `sub`, not the email.
    assert_eq!(session.user_id, "999");
    assert_eq!(session.dna.role(), "session");
    assert!(session.dna.caps().contains("UI"));
    assert_eq!(session.parent_dna.role(), "primitive");
    assert!(session.parent_dna.caps().contains("GO"));
    assert!(session.expires_unix_ms > 0);
}

#[tokio::test]
async fn issue_challenge_rejects_non_oauth() {
    let client = GoogleAuthClient::with_urls(
        "http://t/x", "http://u/x", "cid", "secret", "http://r/cb",
    ).unwrap();
    let provider = GoogleAuthProvider::new(client, None).unwrap();
    let c = AuthChallenge::MagicLink { email: "a@b.c".into() };
    assert!(provider.issue_challenge(&c).await.is_err());
}

#[tokio::test]
async fn verify_rejects_wrong_provider() {
    let client = GoogleAuthClient::with_urls(
        "http://t/x", "http://u/x", "cid", "secret", "http://r/cb",
    ).unwrap();
    let provider = GoogleAuthProvider::new(client, None).unwrap();
    let c = AuthChallenge::OAuthCode {
        provider: "github".into(),
        code: "x".into(),
        state: "y".into(),
        expected_state: "y".into(),
        code_verifier: None,
    };
    assert!(provider.verify(&c).await.is_err());
}

#[tokio::test]
async fn verify_rejects_csrf_state_mismatch() {
    let server = MockServer::start().await;
    let provider = GoogleAuthProvider::new(client_for(&server), None).unwrap();
    let c = AuthChallenge::OAuthCode {
        provider: "google".into(),
        code: "code".into(),
        state: "got-this-state".into(),
        expected_state: "expected-state".into(),
        code_verifier: None,
    };
    let err = provider.verify(&c).await.unwrap_err();
    assert!(
        format!("{err}").contains("CSRF"),
        "expected CSRF error, got: {err}"
    );
}

#[tokio::test]
async fn verify_sends_code_verifier_when_challenge_carries_some() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/token"))
        .and(body_string_contains("code_verifier=my-pkce-verifier"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "access_token": "tok-pkce", "expires_in": 900, "id_token": null
        })))
        .expect(1)
        .mount(&server)
        .await;
    mock_userinfo(&server, json!({
        "sub": "pkce-sub", "email": "pkce@example.com",
        "email_verified": true, "name": "PKCE"
    })).await;
    let session = run_verify(&server, "st", Some("my-pkce-verifier")).await.unwrap();
    // Post-fix: user_id == OIDC `sub`, not email.
    assert_eq!(session.user_id, "pkce-sub");
}
