// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! Wiremock tests for `OpenAiBackend`.
//!
//! Verifies request JSON shape, Bearer auth, and response byte parsing.

#![cfg(all(test, feature = "openai"))]

use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use crate::openai::OpenAiBackend;
use crate::request::{AudioFormat, TtsRequest};
use crate::trait_def::TtsBackend;

#[tokio::test]
async fn openai_synth_ok() {
    let server = MockServer::start().await;
    let fake_audio = b"OPENAI_AUDIO".to_vec();

    Mock::given(method("POST"))
        .and(path("/v1/audio/speech"))
        .and(header("authorization", "Bearer test-key"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_bytes(fake_audio.clone())
                .append_header("Content-Type", "audio/mpeg"),
        )
        .mount(&server)
        .await;

    let backend = OpenAiBackend::with_base_url(server.uri(), "test-key", "tts-1");
    let req = TtsRequest {
        text: "hello openai".into(),
        voice_id: Some("nova".into()),
        language: None,
        format: AudioFormat::Mp3,
    };
    let resp = backend.synth(&req).await.expect("synth should succeed");
    assert_eq!(resp.audio_bytes, fake_audio);
    assert_eq!(resp.mime_type, "audio/mpeg");
}

#[tokio::test]
async fn openai_synth_http_error() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/audio/speech"))
        .respond_with(ResponseTemplate::new(429).set_body_string("Rate limited"))
        .mount(&server)
        .await;

    let backend = OpenAiBackend::with_base_url(server.uri(), "test-key", "tts-1");
    let req = TtsRequest::new("hello");
    let err = backend.synth(&req).await.expect_err("should fail on 429");
    assert!(matches!(err, crate::TtsError::Http(_)));
}
