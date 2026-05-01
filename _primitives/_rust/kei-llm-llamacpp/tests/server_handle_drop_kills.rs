//! Spawn a mock-backed ServerHandle, drop it, assert the kill flag flips.
//! Mock-backed handles flip an `Arc<Mutex<bool>>` instead of sending a
//! signal; that lets us prove Drop fired without running a real child.

mod common;

use kei_llm_llamacpp::server::{start_server, ServerOpts};
use std::sync::{Arc, Mutex};

#[tokio::test]
async fn dropping_server_handle_invokes_kill() {
    let td = tempfile::tempdir().unwrap();
    let model = td.path().join("dummy.gguf");
    std::fs::write(&model, b"x").unwrap();

    let runner = common::MockRunner::new();
    let kill_flag: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    runner.push(common::Behaviour::ServerOk { pid: 4242, kill_flag: kill_flag.clone() });

    let opts = ServerOpts { host: "127.0.0.1".into(), port: 8765 };
    let handle = start_server(&runner, "llama-server", &model, &opts).await.unwrap();
    assert_eq!(handle.pid, 4242);
    assert_eq!(handle.port, 8765);
    assert!(!*kill_flag.lock().unwrap(), "kill flag should be false before drop");

    drop(handle);

    assert!(
        *kill_flag.lock().unwrap(),
        "kill flag must flip to true on Drop"
    );
}
