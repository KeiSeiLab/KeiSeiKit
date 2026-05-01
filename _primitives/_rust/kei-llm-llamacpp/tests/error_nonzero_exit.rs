//! MockRunner returns code=2 + stderr; generate maps it to
//! `Error::NonZeroExit { code: 2, stderr }`.

mod common;

use kei_llm_llamacpp::error::Error;
use kei_llm_llamacpp::generate::generate;
use kei_llm_llamacpp::runner::RunOutput;
use kei_llm_llamacpp::GenerateOpts;

#[tokio::test]
async fn nonzero_exit_maps_to_typed_error() {
    let td = tempfile::tempdir().unwrap();
    let model = td.path().join("dummy.gguf");
    std::fs::write(&model, b"x").unwrap();

    let runner = common::MockRunner::new();
    runner.push(common::Behaviour::Run(RunOutput {
        stdout: String::new(),
        stderr: "model: failed to load tensor 'output.weight'".into(),
        code: 2,
    }));

    let err = generate(&runner, "llama-cli", &model, "p", &GenerateOpts::default())
        .await
        .unwrap_err();

    match err {
        Error::NonZeroExit { code, stderr } => {
            assert_eq!(code, 2);
            assert!(stderr.contains("failed to load tensor"));
        }
        other => panic!("expected NonZeroExit, got {other:?}"),
    }
}

#[test]
fn exit_code_mapping_is_locked() {
    use std::path::PathBuf;
    assert_eq!(Error::BinaryNotFound { name: "x".into() }.exit_code(), 2);
    assert_eq!(Error::ModelNotFound { path: PathBuf::from("p") }.exit_code(), 2);
    assert_eq!(Error::NonZeroExit { code: 1, stderr: "".into() }.exit_code(), 3);
    assert_eq!(Error::Timeout.exit_code(), 4);
    assert_eq!(Error::InvalidHost { host: "0.0.0.0".into() }.exit_code(), 5);
    assert_eq!(Error::ParseFailed { reason: "x".into() }.exit_code(), 1);
}
