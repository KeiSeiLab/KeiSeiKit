//! `is_running` returns `false` when nothing listens on the given URL.
//!
//! We bind a TCP listener on an ephemeral loopback port, drop it (releasing the
//! port), and point the client at that now-unbound URL. The connection refuses
//! immediately on every modern OS — we don't have to wait for any timeout.

use std::net::TcpListener;

use kei_llm_ollama::{is_running, Client};

fn unbound_url() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind ephemeral");
    let addr = listener.local_addr().expect("local_addr");
    drop(listener);
    format!("http://127.0.0.1:{}", addr.port())
}

#[tokio::test]
async fn is_running_false_when_nothing_listening() {
    let url = unbound_url();
    let client = Client::new(url);
    let alive = is_running(&client).await;
    assert!(!alive, "expected is_running == false on dead port");
}
