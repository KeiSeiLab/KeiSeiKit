/// Integration smoke test: spins up a real kei-graph-stream server on a random port,
/// appends events to a temp JSONL file, and verifies WS snapshot + event frames.
use std::io::Write;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use serde_json::Value;
use tempfile::NamedTempFile;
use tokio::sync::broadcast;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures::StreamExt;

async fn start_server(events_path: std::path::PathBuf) -> SocketAddr {
    use axum::Router;
    use axum::routing::get;

    let (tx, _) = broadcast::channel::<String>(256);
    let tx = Arc::new(tx);
    let alive = Arc::new(kei_graph_stream::AliveState::new());

    tokio::spawn(kei_graph_stream::tail::run(
        events_path,
        Arc::clone(&tx),
        Arc::clone(&alive),
    ));

    let app = Router::new()
        .route("/stream", get(kei_graph_stream::ws::ws_handler))
        .route("/health", get(|| async { "kei-graph-stream alive\n" }))
        .with_state((tx, alive));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    // axum::serve returns IntoFuture; use `into_future()` to spawn.
    use std::future::IntoFuture;
    tokio::spawn(axum::serve(listener, app).into_future());
    addr
}

async fn recv_text(
    stream: &mut (impl StreamExt<
        Item = Result<Message, tokio_tungstenite::tungstenite::Error>,
    > + Unpin),
) -> Value {
    loop {
        if let Message::Text(t) = stream.next().await.unwrap().unwrap() {
            return serde_json::from_str(&t).unwrap();
        }
    }
}

#[tokio::test]
async fn smoke_snapshot_and_event() {
    let mut tmp = NamedTempFile::new().unwrap();
    let path = std::path::PathBuf::from(tmp.path());

    let addr = start_server(path.clone()).await;

    // Health check.
    let body = reqwest::get(format!("http://{addr}/health"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert_eq!(body, "kei-graph-stream alive\n");

    // Connect WS before any events — expect empty snapshot.
    let (mut ws1, _) = connect_async(format!("ws://{addr}/stream")).await.unwrap();
    let snap: Value = recv_text(&mut ws1).await;
    assert_eq!(snap["type"], "snapshot");
    assert!(snap["alive"].as_array().unwrap().is_empty());

    // Append a spawn event.
    writeln!(tmp, r#"{{"ts":"2026-05-02T13:00:00.000Z","event":"agent_spawn","id":"smoke1","subagent_type":"researcher","model":"sonnet","prompt_preview":"test"}}"#).unwrap();

    // Allow tail poll (200ms) + margin.
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Should receive an event frame on the existing connection.
    let frame: Value = recv_text(&mut ws1).await;
    assert_eq!(frame["type"], "event");
    assert_eq!(frame["data"]["event"], "agent_spawn");
    assert_eq!(frame["data"]["id"], "smoke1");

    // New client snapshot should contain smoke1.
    let (mut ws2, _) = connect_async(format!("ws://{addr}/stream")).await.unwrap();
    let snap2: Value = recv_text(&mut ws2).await;
    assert_eq!(snap2["type"], "snapshot");
    let alive2 = snap2["alive"].as_array().unwrap();
    assert_eq!(alive2.len(), 1);
    assert_eq!(alive2[0]["id"], "smoke1");

    // Append done event.
    writeln!(tmp, r#"{{"ts":"2026-05-02T13:00:01.000Z","event":"agent_done","id":"smoke1","outcome":"functional","duration_ms":1000}}"#).unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;

    // Third client: snapshot should now be empty.
    let (mut ws3, _) = connect_async(format!("ws://{addr}/stream")).await.unwrap();
    let snap3: Value = recv_text(&mut ws3).await;
    assert_eq!(snap3["type"], "snapshot");
    assert!(snap3["alive"].as_array().unwrap().is_empty());
}
