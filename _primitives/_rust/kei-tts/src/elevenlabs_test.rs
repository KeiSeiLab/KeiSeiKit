// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! Wiremock tests for `ElevenLabsBackend`.
//!
//! Verifies request shape (path, header) and response byte parsing.

#![cfg(all(test, feature = "elevenlabs"))]

use wiremock::matchers::{header, method, path_regex};
use wiremock::{Mock, MockServer, ResponseTemplate};

use crate::elevenlabs::ElevenLabsBackend;
use crate::request::{AudioFormat, TtsRequest};
use crate::trait_def::TtsBackend;

#[tokio::test]
async fn elevenlabs_synth_ok() {
    let server = MockServer::start().await;
    let fake_audio = b"FAKE_AUDIO_BYTES".to_vec();

    Mock::given(method("POST"))
        .and(path_regex(r"/v1/text-to-speech/.+/stream"))
        .and(header("xi-api-key", "test-key"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_bytes(fake_audio.clone())
                .append_header("Content-Type", "audio/mpeg"),
        )
        .mount(&server)
        .await;

    let backend = ElevenLabsBackend::with_base_url(server.uri(), "test-key");
    let req = TtsRequest {
        text: "hello".into(),
        voice_id: Some("voice123".into()),
        language: None,
        format: AudioFormat::Mp3,
    };
    let resp = backend.synth(&req).await.expect("synth should succeed");
    assert_eq!(resp.audio_bytes, fake_audio);
    assert_eq!(resp.mime_type, "audio/mpeg");
}

#[tokio::test]
async fn elevenlabs_synth_http_error() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path_regex(r"/v1/text-to-speech/.+/stream"))
        .respond_with(ResponseTemplate::new(401).set_body_string("Unauthorized"))
        .mount(&server)
        .await;

    let backend = ElevenLabsBackend::with_base_url(server.uri(), "bad-key");
    let req = TtsRequest::new("hello");
    let err = backend.synth(&req).await.expect_err("should fail on 401");
    assert!(matches!(err, crate::TtsError::Http(_)));
}
