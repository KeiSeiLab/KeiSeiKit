// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! CVE-2023-7028 class regression tests for `GoogleAuthProvider`.
//!
//! Booking.com / Slack / GitLab were all hit by the same pattern: an
//! OIDC relying-party trusted `userinfo.email` without checking
//! `email_verified`, allowing a Workspace admin to mint accounts with
//! arbitrary unverified email aliases and sign in as any user.
//!
//! These tests ensure `verify()`:
//! 1. refuses `email_verified == false`
//! 2. refuses absent `email_verified`
//! 3. uses `sub` (not `email`) as `user_id`
//! 4. cross-checks `id_token.sub == userinfo.sub` when an `id_token`
//!    is returned, and rejects mismatch
//! 5. accepts the happy path when both are equal

use base64::Engine as _;
use kei_auth_google::{GoogleAuthClient, GoogleAuthProvider};
use kei_runtime_core::traits::auth::{AuthChallenge, AuthProvider};
use serde_json::json;
use wiremock::matchers::{method, path};
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

fn challenge() -> AuthChallenge {
    AuthChallenge::OAuthCode {
        provider: "google".into(),
        code: "c".into(),
        state: "s".into(),
        expected_state: "s".into(),
        code_verifier: None,
    }
}

fn make_jwt_with_sub(sub: &str) -> String {
    let header = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .encode(br#"{"alg":"RS256","typ":"JWT"}"#);
    let claims_json = format!(r#"{{"sub":"{sub}","aud":"client-id-xyz"}}"#);
    let claims = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .encode(claims_json.as_bytes());
    format!("{header}.{claims}.fake-sig-not-verified-yet")
}

async fn mock_token_no_id(server: &MockServer) {
    Mock::given(method("POST"))
        .and(path("/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "access_token": "tok",
            "expires_in": 1800,
            "id_token": null
        })))
        .mount(server)
        .await;
}

async fn mock_token_with_id(server: &MockServer, id_token: String) {
    Mock::given(method("POST"))
        .and(path("/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "access_token": "tok",
            "expires_in": 1800,
            "id_token": id_token
        })))
        .mount(server)
        .await;
}

async fn mock_userinfo(server: &MockServer, body: serde_json::Value) {
    Mock::given(method("GET"))
        .and(path("/userinfo"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .mount(server)
        .await;
}

#[tokio::test]
async fn verify_rejects_unverified_email() {
    let server = MockServer::start().await;
    mock_token_no_id(&server).await;
    mock_userinfo(&server, json!({
        "sub": "attacker-sub",
        "email": "victim@target.example",
        "email_verified": false,
        "name": "Attacker"
    })).await;
    let provider = GoogleAuthProvider::new(client_for(&server), None).unwrap();
    let err = provider.verify(&challenge()).await.unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("not verified") || msg.contains("email"),
        "expected email-not-verified error, got: {msg}"
    );
}

#[tokio::test]
async fn verify_rejects_missing_email_verified_field() {
    let server = MockServer::start().await;
    mock_token_no_id(&server).await;
    mock_userinfo(&server, json!({
        "sub": "abc",
        "email": "x@y.z",
        "name": "Default"
    })).await;
    let provider = GoogleAuthProvider::new(client_for(&server), None).unwrap();
    assert!(provider.verify(&challenge()).await.is_err());
}

#[tokio::test]
async fn verify_uses_sub_not_email_as_user_id() {
    let server = MockServer::start().await;
    mock_token_no_id(&server).await;
    mock_userinfo(&server, json!({
        "sub": "stable-google-account-id-12345",
        "email": "alice@example.com",
        "email_verified": true,
        "name": "Alice"
    })).await;
    let provider = GoogleAuthProvider::new(client_for(&server), None).unwrap();
    let session = provider.verify(&challenge()).await.unwrap();
    assert_eq!(session.user_id, "stable-google-account-id-12345");
    assert_ne!(session.user_id, "alice@example.com");
}

#[tokio::test]
async fn verify_rejects_id_token_sub_mismatch() {
    let server = MockServer::start().await;
    mock_token_with_id(&server, make_jwt_with_sub("ATTACKER-SUB")).await;
    mock_userinfo(&server, json!({
        "sub": "VICTIM-SUB",
        "email": "v@example.com",
        "email_verified": true,
        "name": "Victim"
    })).await;
    let provider = GoogleAuthProvider::new(client_for(&server), None).unwrap();
    let err = provider.verify(&challenge()).await.unwrap_err();
    assert!(format!("{err}").contains("sub"), "expected sub-mismatch error");
}

#[tokio::test]
async fn verify_accepts_matching_id_token_sub() {
    let server = MockServer::start().await;
    mock_token_with_id(&server, make_jwt_with_sub("happy-sub")).await;
    mock_userinfo(&server, json!({
        "sub": "happy-sub",
        "email": "h@e.io",
        "email_verified": true,
        "name": "Happy"
    })).await;
    let provider = GoogleAuthProvider::new(client_for(&server), None).unwrap();
    let session = provider.verify(&challenge()).await.unwrap();
    assert_eq!(session.user_id, "happy-sub");
}
