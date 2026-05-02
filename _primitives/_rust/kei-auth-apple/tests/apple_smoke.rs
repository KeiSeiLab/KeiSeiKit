// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! Wiremock smoke tests for `AppleAuthProvider`. No live calls to appleid.apple.com.

mod helpers;
use helpers::{sign_id_token, token_response_body, TEST_JWKS_JSON};

use kei_auth_apple::{AppleAuthClient, AppleAuthProvider};
use kei_runtime_core::HasDna;
use kei_runtime_core::traits::auth::{AuthChallenge, AuthProvider};
use wiremock::matchers::{body_string_contains, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn provider_verify_end_to_end_returns_session_with_sub_user_id() {
    let server = MockServer::start().await;
    let id_token = sign_id_token(
        r#"{"sub":"001999.zzz","email":"relay@privaterelay.appleid.com","iss":"https://appleid.apple.com","aud":"com.example.web"}"#,
    );
    Mock::given(method("POST"))
        .and(path("/auth/token"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(token_response_body(&id_token)),
        )
        .mount(&server)
        .await;
    let token_url = format!("{}/auth/token", server.uri());
    let client = AppleAuthClient::with_url(
        token_url, "com.example.web", "JWT-CS", "https://app.example/cb",
    )
    .unwrap();
    let provider = AppleAuthProvider::new(client, TEST_JWKS_JSON, None).unwrap();
    let challenge = AuthChallenge::OAuthCode {
        provider: "apple".into(),
        code: "auth-code-123".into(),
        state: "csrf-token".into(),
        expected_state: "csrf-token".into(),
        code_verifier: None,
    };
    let session = provider.verify(&challenge).await.unwrap();
    assert_eq!(session.user_id, "001999.zzz");
    assert_eq!(session.parent_dna.as_str(), provider.dna().as_str());
    assert!(session.expires_unix_ms > 0);
    assert_eq!(provider.provider_name(), "apple");
    assert!(provider.is_passwordless());
}

#[tokio::test]
async fn provider_verify_csrf_mismatch_rejected() {
    let server = MockServer::start().await;
    let token_url = format!("{}/auth/token", server.uri());
    let client = AppleAuthClient::with_url(
        token_url, "com.example.web", "JWT-CS", "https://app.example/cb",
    )
    .unwrap();
    let provider = AppleAuthProvider::new(client, TEST_JWKS_JSON, None).unwrap();
    let challenge = AuthChallenge::OAuthCode {
        provider: "apple".into(),
        code: "code".into(),
        state: "DIFFERENT".into(),
        expected_state: "EXPECTED".into(),
        code_verifier: None,
    };
    let err = provider.verify(&challenge).await.unwrap_err();
    assert!(
        format!("{err}").contains("CSRF"),
        "expected CSRF error, got: {err}"
    );
}

#[tokio::test]
async fn jwt_decode_rejects_malformed_id_token() {
    let server = MockServer::start().await;
    let bad_id_token = "header.payload"; // only two segments
    Mock::given(method("POST"))
        .and(path("/auth/token"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(token_response_body(bad_id_token)),
        )
        .mount(&server)
        .await;
    let token_url = format!("{}/auth/token", server.uri());
    let client = AppleAuthClient::with_url(
        token_url, "com.example.web", "JWT-CS", "https://app.example/cb",
    )
    .unwrap();
    let provider = AppleAuthProvider::new(client, TEST_JWKS_JSON, None).unwrap();
    let challenge = AuthChallenge::OAuthCode {
        provider: "apple".into(),
        code: "auth-code-123".into(),
        state: "csrf".into(),
        expected_state: "csrf".into(),
        code_verifier: None,
    };
    let err = provider.verify(&challenge).await.unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("jwt") || msg.contains("missing") || msg.contains("verify"),
        "expected jwt-related error, got: {msg}"
    );
}

#[tokio::test]
async fn provider_rejects_non_apple_oauth_code() {
    let server = MockServer::start().await;
    let token_url = format!("{}/auth/token", server.uri());
    let client = AppleAuthClient::with_url(
        token_url, "com.example.web", "JWT-CS", "https://app.example/cb",
    )
    .unwrap();
    let provider = AppleAuthProvider::new(client, TEST_JWKS_JSON, None).unwrap();
    let challenge = AuthChallenge::OAuthCode {
        provider: "github".into(),
        code: "x".into(),
        state: "y".into(),
        expected_state: "y".into(),
        code_verifier: None,
    };
    let err = provider.verify(&challenge).await.unwrap_err();
    assert!(format!("{err}").contains("wrong provider"));
}

#[tokio::test]
async fn verify_sends_code_verifier_when_challenge_carries_some() {
    let server = MockServer::start().await;
    let id_token = sign_id_token(
        r#"{"sub":"pkce-sub","email":"pkce@apple.example","iss":"https://appleid.apple.com","aud":"com.example.web"}"#,
    );
    Mock::given(method("POST"))
        .and(path("/auth/token"))
        .and(body_string_contains("code_verifier=apple-pkce-verifier"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(token_response_body(&id_token)),
        )
        .expect(1)
        .mount(&server)
        .await;
    let token_url = format!("{}/auth/token", server.uri());
    let client = AppleAuthClient::with_url(
        token_url, "com.example.web", "JWT-CS", "https://app.example/cb",
    )
    .unwrap();
    let provider = AppleAuthProvider::new(client, TEST_JWKS_JSON, None).unwrap();
    let challenge = AuthChallenge::OAuthCode {
        provider: "apple".into(),
        code: "auth-code-pkce".into(),
        state: "st".into(),
        expected_state: "st".into(),
        code_verifier: Some("apple-pkce-verifier".into()),
    };
    let session = provider.verify(&challenge).await.unwrap();
    assert_eq!(session.user_id, "pkce-sub");
}
