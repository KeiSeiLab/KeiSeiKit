//! Non-streaming `/api/chat` against wiremock.

use kei_llm_ollama::{ChatReq, Client, Message};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn fixture() -> serde_json::Value {
    serde_json::json!({
        "model": "qwen3:4b",
        "created_at": "2025-04-01T00:00:00Z",
        "message": {"role": "assistant", "content": "Howdy"},
        "done": true,
        "eval_count": 2
    })
}

#[tokio::test]
async fn chat_decodes_assistant_message() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/chat"))
        .respond_with(ResponseTemplate::new(200).set_body_json(fixture()))
        .mount(&server)
        .await;

    let client = Client::new(server.uri());
    let req = ChatReq {
        model: "qwen3:4b".into(),
        messages: vec![Message {
            role: "user".into(),
            content: "hi".into(),
        }],
        stream: false,
        options: None,
    };
    let resp = client.chat(&req).await.expect("chat ok");
    assert!(resp.done);
    assert_eq!(resp.message.role, "assistant");
    assert_eq!(resp.message.content, "Howdy");
}
