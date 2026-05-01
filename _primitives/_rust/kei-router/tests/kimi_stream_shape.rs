//! Kimi (Moonshot OpenAI-compatible) provider streaming wire-shape test.

use futures::StreamExt;
use kei_router::{KimiProvider, Message, Provider, StreamEvent};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn fake_sse_body() -> &'static str {
    "data: {\"choices\":[{\"delta\":{\"content\":\"Konichiwa\"}}]}\n\n\
     data: {\"choices\":[{\"delta\":{\"content\":\" from K2\"}}]}\n\n\
     data: [DONE]\n\n"
}

#[tokio::test]
async fn streams_token_then_done() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .and(header("authorization", "Bearer kimi-test-key"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "text/event-stream")
                .set_body_string(fake_sse_body()),
        )
        .mount(&server)
        .await;

    let endpoint = format!("{}/v1/chat/completions", server.uri());
    let p = KimiProvider::with_endpoint(
        "kimi-test-key".into(),
        "kimi-k2-thinking".into(),
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
    assert_eq!(tokens, vec!["Konichiwa".to_string(), " from K2".to_string()]);
    assert!(got_done);
}

#[test]
fn cost_constants_are_pinned() {
    let p = KimiProvider::with_endpoint(
        "k".into(),
        "kimi-k2-thinking".into(),
        "http://stub".into(),
    );
    assert_eq!(p.name(), "kimi");
    assert_eq!(p.cost_per_m_tok_input_cents(), 60);
    assert_eq!(p.cost_per_m_tok_output_cents(), 250);
}
