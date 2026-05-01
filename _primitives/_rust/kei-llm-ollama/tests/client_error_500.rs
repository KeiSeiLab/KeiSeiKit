//! 500 from /api/generate maps to `ApiError::HttpError { status: 500, .. }`.

use kei_llm_ollama::{ApiError, Client, GenerateReq};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn generate_500_is_http_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/generate"))
        .respond_with(ResponseTemplate::new(500).set_body_string("internal error"))
        .mount(&server)
        .await;

    let client = Client::new(server.uri());
    let req = GenerateReq {
        model: "qwen3:4b".into(),
        prompt: "x".into(),
        stream: false,
        options: None,
    };
    let err = client.generate(&req).await.expect_err("500 must error");
    match err {
        ApiError::HttpError { status, .. } => assert_eq!(status, 500),
        other => panic!("expected HttpError(500), got {other:?}"),
    }
}
