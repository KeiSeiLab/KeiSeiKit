//! `Client::tags()` against a wiremock /api/tags fixture.

use kei_llm_ollama::Client;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn fixture() -> serde_json::Value {
    serde_json::json!({
        "models": [
            {
                "name": "qwen3:4b",
                "model": "qwen3:4b",
                "modified_at": "2025-10-06T18:37:52Z",
                "size": 1234,
                "digest": "deadbeef"
            },
            {
                "name": "tiny:latest",
                "model": "tiny:latest",
                "modified_at": "2025-04-01T00:00:00Z",
                "size": 7,
                "digest": "abcd"
            }
        ]
    })
}

#[tokio::test]
async fn tags_returns_two_models() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/tags"))
        .respond_with(ResponseTemplate::new(200).set_body_json(fixture()))
        .mount(&server)
        .await;

    let client = Client::new(server.uri());
    let resp = client.tags().await.expect("tags ok");
    assert_eq!(resp.models.len(), 2);
    assert_eq!(resp.models[0].name, "qwen3:4b");
    assert_eq!(resp.models[1].name, "tiny:latest");
    assert_eq!(resp.models[0].size, 1234);
}
