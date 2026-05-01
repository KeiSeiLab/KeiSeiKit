//! Anthropic provider streaming wire-shape test.
//!
//! Stands up a wiremock fake of /v1/messages, replies with a hand-crafted SSE
//! body, asserts the StreamEvent sequence the parser produces.

use futures::StreamExt;
use kei_router::{AnthropicProvider, Message, Provider, StreamEvent};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn fake_sse_body() -> &'static str {
    "event: message_start\n\
     data: {\"type\":\"message_start\"}\n\n\
     event: content_block_delta\n\
     data: {\"type\":\"content_block_delta\",\"delta\":{\"type\":\"text_delta\",\"text\":\"Hello\"}}\n\n\
     event: content_block_delta\n\
     data: {\"type\":\"content_block_delta\",\"delta\":{\"type\":\"text_delta\",\"text\":\", world\"}}\n\n\
     event: message_stop\n\
     data: {\"type\":\"message_stop\"}\n\n"
}

#[tokio::test]
async fn streams_token_then_done() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/messages"))
        .and(header("x-api-key", "test-key"))
        .and(header("anthropic-version", "2023-06-01"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "text/event-stream")
                .set_body_string(fake_sse_body()),
        )
        .mount(&server)
        .await;

    let endpoint = format!("{}/v1/messages", server.uri());
    let p = AnthropicProvider::with_endpoint(
        "test-key".into(),
        "claude-haiku-4-5".into(),
        endpoint,
    );
    let messages = vec![Message { role: "user".into(), content: "hi".into() }];
    let mut stream = p.stream_message("be brief", &messages, None).await.expect("open ok");

    let mut tokens = Vec::new();
    let mut got_done = false;
    while let Some(ev) = stream.next().await {
        match ev.expect("event ok") {
            StreamEvent::Token(t) => tokens.push(t),
            StreamEvent::Done => { got_done = true; break; }
            _ => {}
        }
    }
    assert_eq!(tokens, vec!["Hello".to_string(), ", world".to_string()]);
    assert!(got_done, "expected message_stop -> Done");
}

#[tokio::test]
async fn surfaces_429_as_rate_limit() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/messages"))
        .respond_with(ResponseTemplate::new(429).set_body_string("rate limited"))
        .mount(&server)
        .await;

    let endpoint = format!("{}/v1/messages", server.uri());
    let p = AnthropicProvider::with_endpoint("k".into(), "claude-haiku-4-5".into(), endpoint);
    let err = p
        .stream_message("", &[Message { role: "user".into(), content: "x".into() }], None)
        .await
        .err()
        .expect("expected error");
    assert!(matches!(err, kei_router::LlmError::RateLimit(_)), "got {err:?}");
}

#[test]
fn cost_constants_are_pinned() {
    let p = AnthropicProvider::with_endpoint(
        "k".into(),
        "claude-haiku-4-5".into(),
        "http://stub".into(),
    );
    assert_eq!(p.name(), "anthropic");
    assert_eq!(p.cost_per_m_tok_input_cents(), 100);
    assert_eq!(p.cost_per_m_tok_output_cents(), 500);
}
