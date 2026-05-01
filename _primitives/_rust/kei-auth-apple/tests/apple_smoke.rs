// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! `wiremock`-driven smoke tests for [`AppleAuthClient`] +
//! [`AppleAuthProvider`]. No live calls to appleid.apple.com.

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine as _;
use kei_auth_apple::{AppleAuthClient, AppleAuthProvider, Error};
use kei_runtime_core::HasDna;
use kei_runtime_core::traits::auth::{AuthChallenge, AuthProvider};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn b64(s: &str) -> String {
    URL_SAFE_NO_PAD.encode(s.as_bytes())
}

/// Forge a JWT-shaped string with header `{"alg":"ES256"}`, the supplied
/// JSON payload, and a placeholder signature. Signature is NOT validated
/// by this cube (see `jwt.rs` known-limitation note), so any non-empty
/// segment is accepted.
fn fake_id_token(payload_json: &str) -> String {
    let header = b64("{\"alg\":\"ES256\",\"kid\":\"TEST\"}");
    let payload = b64(payload_json);
    let sig = b64("placeholder-signature-bytes");
    format!("{header}.{payload}.{sig}")
}

fn token_response_body(id_token: &str) -> serde_json::Value {
    serde_json::json!({
        "access_token": "at-1234",
        "expires_in": 3600,
        "id_token": id_token,
        "refresh_token": "rt-5678",
        "token_type": "Bearer",
    })
}

#[tokio::test]
async fn token_endpoint_200_returns_token_response() {
    let server = MockServer::start().await;
    let id_token = fake_id_token(
        "{\"sub\":\"001234.abc\",\"email\":\"x@y.example\",\"exp\":9999999999,\"iss\":\"https://appleid.apple.com\"}",
    );
    Mock::given(method("POST"))
        .and(path("/auth/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(token_response_body(&id_token)))
        .mount(&server)
        .await;
    let token_url = format!("{}/auth/token", server.uri());
    let c = AppleAuthClient::with_url(token_url, "com.example.web", "JWT-CS", "https://app.example/cb").unwrap();
    let resp = c.exchange_code("auth-code-123").await.unwrap();
    assert_eq!(resp.access_token, "at-1234");
    assert_eq!(resp.expires_in, 3600);
    assert_eq!(resp.id_token, id_token);
    assert_eq!(resp.refresh_token.as_deref(), Some("rt-5678"));
}

#[tokio::test]
async fn provider_verify_end_to_end_returns_session_with_sub_user_id() {
    let server = MockServer::start().await;
    let id_token = fake_id_token(
        "{\"sub\":\"001999.zzz\",\"email\":\"relay@privaterelay.appleid.com\",\"exp\":9999999999,\"iss\":\"https://appleid.apple.com\"}",
    );
    Mock::given(method("POST"))
        .and(path("/auth/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(token_response_body(&id_token)))
        .mount(&server)
        .await;
    let token_url = format!("{}/auth/token", server.uri());
    let client = AppleAuthClient::with_url(token_url, "com.example.web", "JWT-CS", "https://app.example/cb").unwrap();
    let provider = AppleAuthProvider::new(client, None).unwrap();
    let challenge = AuthChallenge::OAuthCode {
        provider: "apple".into(),
        code: "auth-code-123".into(),
        state: "csrf-token".into(),
    };
    let session = provider.verify(&challenge).await.unwrap();
    assert_eq!(session.user_id, "001999.zzz");
    assert_eq!(session.parent_dna.as_str(), provider.dna().as_str());
    assert!(session.expires_unix_ms > 0);
    assert_eq!(provider.provider_name(), "apple");
    assert!(provider.is_passwordless());
}

#[tokio::test]
async fn token_endpoint_400_maps_to_api_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/auth/token"))
        .respond_with(
            ResponseTemplate::new(400)
                .set_body_string("{\"error\":\"invalid_grant\"}"),
        )
        .mount(&server)
        .await;
    let token_url = format!("{}/auth/token", server.uri());
    let c = AppleAuthClient::with_url(token_url, "com.example.web", "JWT-CS", "https://app.example/cb").unwrap();
    let err = c.exchange_code("bad-code").await.unwrap_err();
    assert!(matches!(err, Error::Api(_)), "expected Api(_), got {err:?}");
}

#[tokio::test]
async fn jwt_decode_rejects_malformed_id_token() {
    // Token endpoint returns a syntactically broken id_token (only two
    // segments). The client.exchange_code call succeeds (the wire-shape
    // is valid JSON), but provider.verify must fail at the JWT-decode
    // step.
    let server = MockServer::start().await;
    let bad_id_token = "header.payload"; // only two segments
    Mock::given(method("POST"))
        .and(path("/auth/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(token_response_body(bad_id_token)))
        .mount(&server)
        .await;
    let token_url = format!("{}/auth/token", server.uri());
    let client = AppleAuthClient::with_url(token_url, "com.example.web", "JWT-CS", "https://app.example/cb").unwrap();
    let provider = AppleAuthProvider::new(client, None).unwrap();
    let challenge = AuthChallenge::OAuthCode {
        provider: "apple".into(),
        code: "auth-code-123".into(),
        state: "csrf".into(),
    };
    let err = provider.verify(&challenge).await.unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("jwt decode") || msg.contains("missing"),
        "expected jwt-decode-related error, got: {msg}"
    );
}

#[tokio::test]
async fn provider_rejects_non_apple_oauth_code() {
    // No HTTP call should be made — error short-circuits before
    // exchange_code. Use a server with no mounts so any unexpected POST
    // would 404 and surface a different error class.
    let server = MockServer::start().await;
    let token_url = format!("{}/auth/token", server.uri());
    let client = AppleAuthClient::with_url(token_url, "com.example.web", "JWT-CS", "https://app.example/cb").unwrap();
    let provider = AppleAuthProvider::new(client, None).unwrap();
    let challenge = AuthChallenge::OAuthCode {
        provider: "github".into(),
        code: "x".into(),
        state: "y".into(),
    };
    let err = provider.verify(&challenge).await.unwrap_err();
    assert!(format!("{err}").contains("wrong provider"));
}
