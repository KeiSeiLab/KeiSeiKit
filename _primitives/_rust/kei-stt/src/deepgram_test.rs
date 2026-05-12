// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! Wiremock tests for `DeepgramBackend`.
//!
//! Verifies request headers, URL parameters, and response parsing.

#![cfg(all(test, feature = "deepgram"))]

use wiremock::matchers::{header, header_regex, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use crate::deepgram::DeepgramBackend;
use crate::request::SttRequest;
use crate::trait_def::SttBackend;

fn deepgram_response_body() -> serde_json::Value {
    serde_json::json!({
        "results": {
            "channels": [{
                "alternatives": [{
                    "transcript": "hello deepgram",
                    "words": [
                        {"word": "hello", "start": 0.1, "end": 0.5},
                        {"word": "deepgram", "start": 0.6, "end": 1.1}
                    ]
                }]
            }]
        }
    })
}

#[tokio::test]
async fn deepgram_parses_transcript() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/listen"))
        .and(header_regex("authorization", "Token .+"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(deepgram_response_body()),
        )
        .mount(&server)
        .await;

    let backend = DeepgramBackend::with_base_url(server.uri(), "test-key");
    let req = SttRequest {
        audio_bytes: b"fake_audio".to_vec(),
        mime_type: "audio/wav".to_string(),
        language: None,
    };
    let resp = backend.transcribe(&req).await.expect("transcribe should succeed");
    assert_eq!(resp.text, "hello deepgram");
    assert_eq!(resp.segments.len(), 2);
    assert_eq!(resp.segments[0].start_ms, 100);
    assert_eq!(resp.segments[1].end_ms, 1100);
    assert!(resp.language_detected.is_none());
}

#[tokio::test]
async fn deepgram_sends_auth_header() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/listen"))
        .and(header("authorization", "Token secret-key"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(deepgram_response_body()),
        )
        .mount(&server)
        .await;

    let backend = DeepgramBackend::with_base_url(server.uri(), "secret-key");
    let req = SttRequest::new_wav(b"audio".to_vec());
    backend.transcribe(&req).await.expect("auth header test should pass");
}

#[tokio::test]
async fn deepgram_http_error() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/listen"))
        .respond_with(ResponseTemplate::new(401).set_body_string("Unauthorized"))
        .mount(&server)
        .await;

    let backend = DeepgramBackend::with_base_url(server.uri(), "bad-key");
    let req = SttRequest::new_wav(b"audio".to_vec());
    let err = backend.transcribe(&req).await.expect_err("should fail on 401");
    assert!(matches!(err, crate::SttError::Http(_)));
}
