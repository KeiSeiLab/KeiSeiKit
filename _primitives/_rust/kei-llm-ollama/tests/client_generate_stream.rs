//! Streaming `/api/generate` (NDJSON) against wiremock.

use futures::StreamExt;
use kei_llm_ollama::{Client, GenerateReq};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn ndjson_body() -> &'static str {
    "{\"model\":\"qwen3:4b\",\"response\":\"Hel\",\"done\":false}\n\
     {\"model\":\"qwen3:4b\",\"response\":\"lo\",\"done\":false}\n\
     {\"model\":\"qwen3:4b\",\"response\":\"!\",\"done\":false}\n\
     {\"model\":\"qwen3:4b\",\"response\":\"\",\"done\":true,\"eval_count\":3,\"eval_duration\":42}\n"
}

#[tokio::test]
async fn stream_yields_chunks_then_done() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/api/generate"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "application/x-ndjson")
                .set_body_string(ndjson_body()),
        )
        .mount(&server)
        .await;

    let client = Client::new(server.uri());
    let req = GenerateReq {
        model: "qwen3:4b".into(),
        prompt: "hi".into(),
        stream: true,
        options: None,
    };
    let mut stream = client.generate_stream(&req).await.expect("open stream");
    let mut deltas = Vec::new();
    let mut got_done = false;
    let mut last_eval = None;
    while let Some(chunk_res) = stream.next().await {
        let chunk = chunk_res.expect("chunk ok");
        if chunk.done {
            got_done = true;
            last_eval = chunk.eval_count;
            break;
        }
        deltas.push(chunk.delta);
    }
    assert_eq!(deltas, vec!["Hel".to_string(), "lo".into(), "!".into()]);
    assert!(got_done, "expected terminal done:true chunk");
    assert_eq!(last_eval, Some(3));
}
