//! Discovery against an empty PATH yields BinPaths {None,None,None}.
//! `generate` against a non-existent model yields ModelNotFound.

mod common;

use kei_llm_llamacpp::error::Error;
use kei_llm_llamacpp::generate::generate;
use kei_llm_llamacpp::{discover, GenerateOpts};
use std::path::PathBuf;

#[tokio::test]
async fn discover_with_empty_path_returns_none() {
    let td = tempfile::tempdir().unwrap();
    let _guard = path_lock();
    let prev_path = std::env::var_os("PATH");
    let prev_dir = std::env::var_os("KEI_LLAMA_CPP_DIR");
    std::env::set_var("PATH", td.path());
    std::env::remove_var("KEI_LLAMA_CPP_DIR");

    let runner = common::MockRunner::new();
    let bp = discover(&runner).await.unwrap();

    if let Some(p) = prev_path { std::env::set_var("PATH", p); }
    if let Some(p) = prev_dir { std::env::set_var("KEI_LLAMA_CPP_DIR", p); }

    assert!(bp.llama_cli.is_none());
    assert!(bp.llama_server.is_none());
}

#[tokio::test]
async fn generate_with_missing_model_returns_model_not_found() {
    let bogus = PathBuf::from("/tmp/this-model-must-not-exist-kei.gguf");
    let runner = common::MockRunner::new();
    let err = generate(&runner, "llama-cli", &bogus, "p", &GenerateOpts::default())
        .await
        .unwrap_err();

    match err {
        Error::ModelNotFound { path } => assert_eq!(path, bogus),
        other => panic!("expected ModelNotFound, got {other:?}"),
    }
}

fn path_lock() -> std::sync::MutexGuard<'static, ()> {
    use std::sync::{Mutex, OnceLock};
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(())).lock().unwrap_or_else(|e| e.into_inner())
}
