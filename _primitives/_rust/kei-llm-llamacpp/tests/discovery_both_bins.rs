//! With both binaries present (via tempdir + KEI_LLAMA_CPP_DIR),
//! `discover` returns populated BinPaths. The Mock pretends `--version`
//! emitted "version 4203" so we get a parsed version too.

mod common;

use kei_llm_llamacpp::discover;
use kei_llm_llamacpp::runner::RunOutput;

#[tokio::test]
async fn discovery_both_bins_populates_binpaths() {
    let td = tempfile::tempdir().unwrap();
    let cli = td.path().join("llama-cli");
    let server = td.path().join("llama-server");
    std::fs::write(&cli, b"#!/bin/sh\n").unwrap();
    std::fs::write(&server, b"#!/bin/sh\n").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&cli, std::fs::Permissions::from_mode(0o755)).unwrap();
        std::fs::set_permissions(&server, std::fs::Permissions::from_mode(0o755)).unwrap();
    }

    let _guard = path_lock();
    let prev = std::env::var_os("KEI_LLAMA_CPP_DIR");
    std::env::set_var("KEI_LLAMA_CPP_DIR", td.path());

    let runner = common::MockRunner::new();
    runner.push(common::Behaviour::Run(RunOutput {
        stdout: "version 4203\n".into(),
        stderr: String::new(),
        code: 0,
    }));

    let bp = discover(&runner).await.unwrap();

    if let Some(p) = prev {
        std::env::set_var("KEI_LLAMA_CPP_DIR", p);
    } else {
        std::env::remove_var("KEI_LLAMA_CPP_DIR");
    }

    assert!(bp.llama_cli.is_some(), "llama-cli should be located");
    assert!(bp.llama_server.is_some(), "llama-server should be located");
    assert_eq!(bp.version.as_deref(), Some("4203"));
    assert!(bp.any_found());
}

fn path_lock() -> std::sync::MutexGuard<'static, ()> {
    use std::sync::{Mutex, OnceLock};
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(())).lock().unwrap_or_else(|e| e.into_inner())
}
