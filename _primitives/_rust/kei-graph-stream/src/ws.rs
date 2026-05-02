use axum::{
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
};
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::time::{Duration, interval};

use crate::auth::{extract_bearer, load_expected_token, tokens_match, validate_origin};
use crate::state::AliveState;

pub type AppState = (Arc<broadcast::Sender<String>>, Arc<AliveState>);

/// Axum handler: validates Origin + bearer before upgrading to WebSocket.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    headers: HeaderMap,
    State((tx, alive)): State<AppState>,
) -> Response {
    let origin = headers.get(header::ORIGIN).and_then(|v| v.to_str().ok());
    if let Err(e) = validate_origin(origin) {
        eprintln!("[kei-graph-stream] ws origin rejected: {e}");
        return (StatusCode::FORBIDDEN, "forbidden\n").into_response();
    }
    let proto = headers
        .get("sec-websocket-protocol")
        .and_then(|v| v.to_str().ok());
    if let Err(e) = check_bearer(proto) {
        eprintln!("[kei-graph-stream] ws auth rejected: {e}");
        return (StatusCode::UNAUTHORIZED, "unauthorized\n").into_response();
    }
    ws.protocols(["bearer"])
        .on_upgrade(move |socket| handle_socket(socket, tx, alive))
}

fn check_bearer(protocol: Option<&str>) -> Result<(), crate::auth::AuthError> {
    let expected = load_expected_token()?;
    let got = extract_bearer(protocol)?;
    if !tokens_match(&expected, got) {
        return Err(crate::auth::AuthError::BearerInvalid);
    }
    Ok(())
}

async fn handle_socket(
    mut socket: WebSocket,
    tx: Arc<broadcast::Sender<String>>,
    alive: Arc<AliveState>,
) {
    // 1. Send snapshot of currently alive agents.
    let snapshot = build_snapshot(&alive);
    if socket.send(Message::Text(snapshot)).await.is_err() {
        return;
    }

    // 2. Subscribe to broadcast AFTER snapshot to avoid missing events.
    let mut rx = tx.subscribe();
    let mut heartbeat = interval(Duration::from_secs(30));
    heartbeat.tick().await; // consume the immediate first tick

    loop {
        tokio::select! {
            // Broadcast event → forward to client.
            result = rx.recv() => {
                match result {
                    Ok(msg) => {
                        if socket.send(Message::Text(msg)).await.is_err() {
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        eprintln!("[ws] client lagged {n} messages");
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }

            // Heartbeat ping every 30s.
            _ = heartbeat.tick() => {
                let ping = r#"{"type":"ping"}"#.to_string();
                if socket.send(Message::Text(ping)).await.is_err() {
                    break;
                }
            }

            // Client message (pong or close).
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {} // ignore other client frames
                }
            }
        }
    }
}

fn build_snapshot(alive: &AliveState) -> String {
    let agents = alive.snapshot();
    serde_json::json!({
        "type": "snapshot",
        "alive": agents,
    })
    .to_string()
}
