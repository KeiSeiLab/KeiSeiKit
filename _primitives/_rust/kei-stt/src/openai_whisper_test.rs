// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//! Wiremock tests for `OpenAiWhisperBackend`.
//!
//! Verifies Bearer auth, multipart body, and verbose_json segment parsing.

#![cfg(all(test, feature = "openai-whisper"))]

use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use crate::openai_whisper::OpenAiWhisperBackend;
use crate::request::SttRequest;
use crate::trait_def::SttBackend;

fn verbose_json_body() -> serde_json::Value {
    serde_json::json!({
        "text": "hello openai",
        "language": "english",
        "segments": [
            {"start": 0.0, "end": 0.5, "text": "hello"},
            {"start": 0.5, "end": 1.2, "text": "openai"}
        ]
    })
}

#[tokio::test]
async fn openai_whisper_parses_segments() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/audio/transcriptions"))
        .and(header("authorization", "Bearer test-key"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(verbose_json_body()),
        )
        .mount(&server)
        .await;

    let backend = OpenAiWhisperBackend::with_base_url(server.uri(), "test-key");
    let req = SttRequest {
        audio_bytes: b"fake_audio".to_vec(),
        mime_type: "audio/wav".to_string(),
        language: None,
    };
    let resp = backend.transcribe(&req).await.expect("transcribe should succeed");
    assert_eq!(resp.text, "hello openai");
    assert_eq!(resp.segments.len(), 2);
    assert_eq!(resp.segments[0].start_ms, 0);
    assert_eq!(resp.segments[0].end_ms, 500);
    assert_eq!(resp.segments[1].start_ms, 500);
    assert_eq!(resp.segments[1].end_ms, 1200);
    assert_eq!(resp.language_detected.as_deref(), Some("english"));
}

#[tokio::test]
async fn openai_whisper_http_error() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/audio/transcriptions"))
        .respond_with(ResponseTemplate::new(429).set_body_string("Rate limited"))
        .mount(&server)
        .await;

    let backend = OpenAiWhisperBackend::with_base_url(server.uri(), "test-key");
    let req = SttRequest::new_wav(b"audio".to_vec());
    let err = backend.transcribe(&req).await.expect_err("should fail on 429");
    assert!(matches!(err, crate::SttError::Http(_)));
}
