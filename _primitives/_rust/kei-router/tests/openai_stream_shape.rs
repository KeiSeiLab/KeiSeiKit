//! OpenAI provider streaming wire-shape test.

use futures::StreamExt;
use kei_router::{Message, OpenAiProvider, Provider, StreamEvent};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn fake_sse_body() -> &'static str {
    "data: {\"choices\":[{\"delta\":{\"content\":\"Hi\"}}]}\n\n\
     data: {\"choices\":[{\"delta\":{\"content\":\" there\"}}]}\n\n\
     data: [DONE]\n\n"
}

#[tokio::test]
async fn streams_token_then_done() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .and(header("authorization", "Bearer test-key"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "text/event-stream")
                .set_body_string(fake_sse_body()),
        )
        .mount(&server)
        .await;

    let endpoint = format!("{}/v1/chat/completions", server.uri());
    let p = OpenAiProvider::with_endpoint("test-key".into(), "gpt-4o-mini".into(), endpoint);
    let messages = vec![Message { role: "user".into(), content: "hi".into() }];
    let mut stream = p.stream_message("system!", &messages, None).await.expect("open ok");

    let mut tokens = Vec::new();
    let mut got_done = false;
    while let Some(ev) = stream.next().await {
        match ev.expect("event ok") {
            StreamEvent::Token(t) => tokens.push(t),
            StreamEvent::Done => { got_done = true; break; }
            _ => {}
        }
    }
    assert_eq!(tokens, vec!["Hi".to_string(), " there".to_string()]);
    assert!(got_done, "expected [DONE] -> Done");
}

#[tokio::test]
async fn surfaces_429_as_rate_limit() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(429).set_body_string("limited"))
        .mount(&server)
        .await;

    let endpoint = format!("{}/v1/chat/completions", server.uri());
    let p = OpenAiProvider::with_endpoint("k".into(), "gpt-4o-mini".into(), endpoint);
    let err = p
        .stream_message("", &[Message { role: "user".into(), content: "x".into() }], None)
        .await
        .err()
        .expect("expected error");
    assert!(matches!(err, kei_router::LlmError::RateLimit(_)));
}

#[test]
fn cost_constants_are_pinned() {
    let p = OpenAiProvider::with_endpoint("k".into(), "gpt-4o-mini".into(), "http://stub".into());
    assert_eq!(p.name(), "openai");
    assert_eq!(p.cost_per_m_tok_input_cents(), 15);
    assert_eq!(p.cost_per_m_tok_output_cents(), 60);
}
