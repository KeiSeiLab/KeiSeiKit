//! MockRunner emits N lines as if llama-cli streamed N tokens.
//! Stream API yields N+1 chunks (N tokens, then a `done: true` marker).

mod common;

use kei_llm_llamacpp::stream::generate_stream;
use kei_llm_llamacpp::GenerateOpts;

#[tokio::test]
async fn generate_stream_yields_tokens_plus_done() {
    let td = tempfile::tempdir().unwrap();
    let model = td.path().join("dummy.gguf");
    std::fs::write(&model, b"x").unwrap();

    let runner = common::MockRunner::new();
    runner.push(common::Behaviour::Stream(vec![
        "Hello".into(),
        " ".into(),
        "world".into(),
        "llama_perf_context_print: footer ignored".into(),
    ]));

    let opts = GenerateOpts::default();
    let chunks = generate_stream(&runner, "llama-cli", &model, "p", &opts)
        .await
        .unwrap();

    // 3 token lines + footer dropped + 1 done marker = 4 chunks total.
    assert_eq!(chunks.len(), 4, "got {} chunks: {chunks:?}", chunks.len());
    assert_eq!(chunks[0].delta, "Hello");
    assert_eq!(chunks[0].tokens_so_far, 1);
    assert!(!chunks[0].done);
    assert_eq!(chunks[2].delta, "world");
    assert_eq!(chunks[2].tokens_so_far, 3);
    assert!(chunks[3].done, "last chunk must be done=true");
    assert_eq!(chunks[3].tokens_so_far, 3, "tokens_so_far frozen at last token count");
}
