//! 404 from /api/generate maps to `ApiError::ModelNotFound`.

use kei_llm_ollama::{ApiError, Client, GenerateReq};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn generate_404_is_model_not_found() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/generate"))
        .respond_with(ResponseTemplate::new(404).set_body_string("model 'nope' not found"))
        .mount(&server)
        .await;

    let client = Client::new(server.uri());
    let req = GenerateReq {
        model: "nope".into(),
        prompt: "x".into(),
        stream: false,
        options: None,
    };
    let err = client.generate(&req).await.expect_err("404 must error");
    assert!(
        matches!(err, ApiError::ModelNotFound(_)),
        "expected ModelNotFound, got: {err:?}"
    );
    assert_eq!(err.exit_code(), 2);
}
