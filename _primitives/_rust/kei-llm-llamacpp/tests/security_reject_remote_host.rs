//! Security: server must reject any non-loopback host with an explicit
//! Error::InvalidHost. This crate is a daemon spawner — never a remote
//! exposure tool.

mod common;

use kei_llm_llamacpp::error::Error;
use kei_llm_llamacpp::server::{start_server, validate_host, ServerOpts};

#[tokio::test]
async fn server_rejects_zero_host() {
    let td = tempfile::tempdir().unwrap();
    let model = td.path().join("dummy.gguf");
    std::fs::write(&model, b"x").unwrap();

    let runner = common::MockRunner::new();
    let opts = ServerOpts { host: "0.0.0.0".into(), port: 8080 };
    let err = start_server(&runner, "llama-server", &model, &opts).await.unwrap_err();

    match err {
        Error::InvalidHost { host } => assert_eq!(host, "0.0.0.0"),
        other => panic!("expected InvalidHost, got {other:?}"),
    }
}

#[tokio::test]
async fn server_rejects_public_ip() {
    let td = tempfile::tempdir().unwrap();
    let model = td.path().join("dummy.gguf");
    std::fs::write(&model, b"x").unwrap();
    let runner = common::MockRunner::new();
    let opts = ServerOpts { host: "8.8.8.8".into(), port: 8080 };
    let err = start_server(&runner, "llama-server", &model, &opts).await.unwrap_err();
    assert!(matches!(err, Error::InvalidHost { .. }));
}

#[test]
fn validate_host_accepts_loopback_aliases() {
    validate_host("127.0.0.1").expect("127.0.0.1 must pass");
    validate_host("localhost").expect("localhost must pass");
    validate_host("::1").expect("::1 must pass");
    validate_host("LOCALHOST").expect("case-insensitive accept");
}

#[test]
fn validate_host_rejects_remote() {
    assert!(matches!(validate_host("0.0.0.0"), Err(Error::InvalidHost { .. })));
    assert!(matches!(validate_host("192.168.1.10"), Err(Error::InvalidHost { .. })));
    assert!(matches!(validate_host(""), Err(Error::InvalidHost { .. })));
}
