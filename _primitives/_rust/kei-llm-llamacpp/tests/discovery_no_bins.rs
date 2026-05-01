//! With both binaries absent, `discover` returns an all-None BinPaths
//! and `any_found` is false. The Runner is never called because
//! `locate()` fails first via PATH lookup.

mod common;

use kei_llm_llamacpp::discover;

#[tokio::test]
async fn discovery_no_bins_returns_empty_binpaths() {
    // Force PATH to a directory that almost certainly contains no
    // llama-cli / llama-server. Tempdir works because we create it
    // empty.
    let td = tempfile::tempdir().unwrap();
    // Restoring PATH on test exit is unnecessary — each tokio test
    // runs in the parent process but env is process-wide; we use a
    // small lock to serialize PATH-mutating tests.
    let _guard = path_lock();
    let original = std::env::var_os("PATH");
    std::env::set_var("PATH", td.path());
    std::env::remove_var("KEI_LLAMA_CPP_DIR");

    let runner = common::MockRunner::new();
    let bp = discover(&runner).await.unwrap();

    if let Some(p) = original {
        std::env::set_var("PATH", p);
    } else {
        std::env::remove_var("PATH");
    }

    assert!(bp.llama_cli.is_none(), "expected None, got {:?}", bp.llama_cli);
    assert!(bp.llama_server.is_none(), "expected None, got {:?}", bp.llama_server);
    assert!(bp.version.is_none());
    assert!(!bp.any_found(), "any_found should be false on empty discovery");
}

// Process-wide PATH mutex shared with the other discovery test.
fn path_lock() -> std::sync::MutexGuard<'static, ()> {
    use std::sync::{Mutex, OnceLock};
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(())).lock().unwrap_or_else(|e| e.into_inner())
}
