// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Wiremock smoke tests for `GoogleAuthProvider`. No live HTTP.

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
        code_verifier: None,
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
        code_verifier: None,
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
        code_verifier: None,
    };
    let err = provider.verify(&challenge).await.unwrap_err();
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
            "access_token": "tok-pkce",
            "expires_in": 900,
            "id_token": null
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/userinfo"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "sub": "pkce-sub",
            "email": "pkce@example.com",
            "name": "PKCE"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = client_for(&server);
    let provider = GoogleAuthProvider::new(client, None).unwrap();
    let challenge = AuthChallenge::OAuthCode {
        provider: "google".into(),
        code: "code-pkce".into(),
        state: "st".into(),
        expected_state: "st".into(),
        code_verifier: Some("my-pkce-verifier".into()),
    };
    let session = provider.verify(&challenge).await.unwrap();
    assert_eq!(session.user_id, "pkce@example.com");
}
