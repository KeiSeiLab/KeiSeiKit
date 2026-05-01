//! MockRunner returns fake llama-cli stdout (token answer + timing
//! footer in stderr). Asserts Response.text + tokens_per_sec parse out
//! of the combined buffer.

mod common;

use kei_llm_llamacpp::generate::generate;
use kei_llm_llamacpp::runner::RunOutput;
use kei_llm_llamacpp::GenerateOpts;

#[tokio::test]
async fn generate_parses_text_and_timings() {
    let td = tempfile::tempdir().unwrap();
    let model = td.path().join("dummy.gguf");
    std::fs::write(&model, b"x").unwrap();

    let runner = common::MockRunner::new();
    runner.push(common::Behaviour::Run(RunOutput {
        stdout: "Hello world\n".to_string(),
        stderr: "llama_perf_context_print: eval time = 1000.00 ms / 10 runs (extras)\n"
            .to_string(),
        code: 0,
    }));

    let opts = GenerateOpts { max_tokens: 10, temperature: Some(0.7) };
    let resp = generate(&runner, "llama-cli", &model, "say hi", &opts).await.unwrap();

    assert_eq!(resp.text, "Hello world");
    assert_eq!(resp.eval_tokens, 10);
    assert!((resp.eval_ms - 1000.0).abs() < 0.001);
    assert!((resp.tokens_per_sec - 10.0).abs() < 0.001);

    let args = runner.last_args.lock().unwrap().clone().unwrap();
    assert!(args.iter().any(|a| a == "-m"));
    assert!(args.iter().any(|a| a == "say hi"));
    assert!(args.iter().any(|a| a == "10"), "max_tokens should be in argv");
    assert!(args.iter().any(|a| a == "0.7"), "temperature should be in argv");
}
