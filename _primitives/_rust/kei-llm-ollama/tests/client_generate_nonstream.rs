//! Non-streaming `/api/generate` against wiremock.

use kei_llm_ollama::{Client, GenerateReq};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn fixture() -> serde_json::Value {
    serde_json::json!({
        "model": "qwen3:4b",
        "created_at": "2025-04-01T00:00:00Z",
        "response": "Hello!",
        "done": true,
        "eval_count": 5,
        "eval_duration": 9999
    })
}

#[tokio::test]
async fn generate_decodes_response() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/generate"))
        .respond_with(ResponseTemplate::new(200).set_body_json(fixture()))
        .mount(&server)
        .await;

    let client = Client::new(server.uri());
    let req = GenerateReq {
        model: "qwen3:4b".into(),
        prompt: "hi".into(),
        stream: false,
        options: None,
    };
    let resp = client.generate(&req).await.expect("generate ok");
    assert!(resp.done);
    assert_eq!(resp.response, "Hello!");
    assert_eq!(resp.eval_count, Some(5));
}
