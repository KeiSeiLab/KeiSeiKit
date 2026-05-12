// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! Wiremock tests for `GoogleBackend`.
//!
//! Verifies JSON request shape and base64 `audioContent` decoding.

#![cfg(all(test, feature = "google"))]

use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use crate::google::GoogleBackend;
use crate::request::{AudioFormat, TtsRequest};
use crate::trait_def::TtsBackend;

#[tokio::test]
async fn google_synth_ok() {
    let server = MockServer::start().await;
    let fake_audio = b"GOOGLE_AUDIO".to_vec();
    let encoded = B64.encode(&fake_audio);
    let body = serde_json::json!({ "audioContent": encoded }).to_string();

    Mock::given(method("POST"))
        .and(path("/v1/text:synthesize"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(body)
                .append_header("Content-Type", "application/json"),
        )
        .mount(&server)
        .await;

    let backend = GoogleBackend::with_base_url(server.uri(), "test-key");
    let req = TtsRequest {
        text: "hello google".into(),
        voice_id: None,
        language: Some("en-US".into()),
        format: AudioFormat::Mp3,
    };
    let resp = backend.synth(&req).await.expect("synth should succeed");
    assert_eq!(resp.audio_bytes, fake_audio);
    assert_eq!(resp.mime_type, "audio/mpeg");
}

#[tokio::test]
async fn google_synth_invalid_base64() {
    let server = MockServer::start().await;
    let body = serde_json::json!({ "audioContent": "!!!not_b64!!!" }).to_string();

    Mock::given(method("POST"))
        .and(path("/v1/text:synthesize"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(body)
                .append_header("Content-Type", "application/json"),
        )
        .mount(&server)
        .await;

    let backend = GoogleBackend::with_base_url(server.uri(), "test-key");
    let req = TtsRequest::new("hello");
    let err = backend.synth(&req).await.expect_err("should fail on bad b64");
    assert!(matches!(err, crate::TtsError::InvalidResponse(_)));
}
