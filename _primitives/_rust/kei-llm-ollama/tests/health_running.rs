//! `is_running` returns `true` when /api/tags responds 200.

use kei_llm_ollama::{is_running, snapshot, Client};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn is_running_returns_true_for_live_daemon() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/tags"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"models": []})))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/api/version"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"version": "0.21.2"})))
        .mount(&server)
        .await;

    let client = Client::new(server.uri());
    assert!(is_running(&client).await, "expected is_running == true");

    let snap = snapshot(&client).await;
    assert!(snap.running);
    assert_eq!(snap.version.as_deref(), Some("0.21.2"));
    assert_eq!(snap.models_count, Some(0));
}
