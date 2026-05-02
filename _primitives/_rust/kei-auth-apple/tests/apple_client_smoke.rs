// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! Wiremock smoke tests for `AppleAuthClient` HTTP layer. No live HTTP.

#[allow(dead_code)]
mod helpers;
use helpers::{sign_id_token, token_response_body};

use kei_auth_apple::{AppleAuthClient, Error};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn token_endpoint_200_returns_token_response() {
    let server = MockServer::start().await;
    let id_token = sign_id_token(
        r#"{"sub":"001234.abc","email":"x@y.example","iss":"https://appleid.apple.com","aud":"com.example.web"}"#,
    );
    Mock::given(method("POST"))
        .and(path("/auth/token"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(token_response_body(&id_token)),
        )
        .mount(&server)
        .await;
    let token_url = format!("{}/auth/token", server.uri());
    let c = AppleAuthClient::with_url(
        token_url, "com.example.web", "JWT-CS", "https://app.example/cb",
    )
    .unwrap();
    let resp = c.exchange_code("auth-code-123", None).await.unwrap();
    assert_eq!(resp.access_token, "at-1234");
    assert_eq!(resp.expires_in, 3600);
    assert_eq!(resp.id_token, id_token);
    assert_eq!(resp.refresh_token.as_deref(), Some("rt-5678"));
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
    let c = AppleAuthClient::with_url(
        token_url, "com.example.web", "JWT-CS", "https://app.example/cb",
    )
    .unwrap();
    let err = c.exchange_code("bad-code", None).await.unwrap_err();
    assert!(matches!(err, Error::Api(_)), "expected Api(_), got {err:?}");
}
