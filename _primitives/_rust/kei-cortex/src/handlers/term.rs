//! `WS /api/v1/cortex/term?cwd=<path>` — bidirectional terminal pipe.
//!
//! Spawns the user's `$SHELL` in a PTY rooted at `cwd`. WS frames in feed
//! the PTY stdin; PTY stdout is forwarded as WS BINARY frames (xterm.js
//! `term.write(Uint8Array)` consumes them losslessly — UTF-8 multi-byte
//! sequences and ANSI escapes split across reads no longer get mangled).
//!
//! Path safety: `cwd` MUST resolve inside `state.config().project_root` —
//! mirrors the `/fs/list` chroot. Symlinks are not followed.
//!
//! Wave 44b fixes:
//! - F-HIGH-2: Origin header check before WS upgrade (CSWSH defence).
//! - F-MED-2: PTY child kill+wait via `PtyBag::Drop`.
//! - F-MED-5: `Message::Binary` instead of UTF-8 lossy text frames.
//! - MISS-2: PTY reader is `tokio::spawn_blocking` + cancel token, no
//!   leaked OS threads on disconnect.

use crate::error::AppError;
use crate::handlers::term_pty::{spawn_pty, PtyBag};
use crate::state::AppState;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Query, State};
use axum::http::{header, HeaderMap};
use axum::response::Response;
use futures::SinkExt;
use futures::StreamExt;
use serde::Deserialize;
use std::io::Write;
use std::path::{Path, PathBuf};
use tokio::sync::mpsc;

#[derive(Debug, Default, Deserialize)]
pub struct TermQuery {
    #[serde(default)]
    pub cwd: Option<String>,
}

/// WebSocket upgrade entry point. Validates `Origin` and `cwd` BEFORE
/// upgrading so the client gets a clean 4xx (rather than an immediate WS
/// close). Browser clients send the bearer token via
/// `Sec-WebSocket-Protocol: bearer,...` and require us to echo `bearer`
/// back so the handshake completes.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<TermQuery>,
) -> Result<Response, AppError> {
    validate_origin(&headers, state.config().cors_origin.as_str())?;
    let cwd = resolve_cwd(&state.config().project_root, q.cwd.as_deref())?;
    Ok(ws
        .protocols(["bearer"])
        .on_upgrade(move |socket| handle_socket(socket, cwd)))
}

/// Reject WebSocket upgrades whose `Origin` header is missing, literally
/// `null`, or does not exactly match the configured `cors_origin`.
///
/// Why: CORS preflight does NOT cover WS upgrades, so the browser CORS
/// policy alone won't stop a drive-by site from opening
/// `new WebSocket('ws://localhost:9797/...')`. Browsers DO always set
/// `Origin` on cross-origin WS requests though, so an exact-match check
/// is the standard CSWSH defence.
fn validate_origin(headers: &HeaderMap, allowed: &str) -> Result<(), AppError> {
    let origin = headers
        .get(header::ORIGIN)
        .ok_or_else(|| AppError::Forbidden)?
        .to_str()
        .map_err(|_| AppError::Forbidden)?;
    if origin == "null" || origin != allowed {
        return Err(AppError::Forbidden);
    }
    Ok(())
}

/// Resolve the requested cwd; defaults to project root when unset. Rejects
/// `..`, absolute paths outside root, and non-directories.
fn resolve_cwd(root: &Path, requested: Option<&str>) -> Result<PathBuf, AppError> {
    let req = requested.unwrap_or("");
    if req.contains("..") {
        return Err(AppError::BadRequest("cwd may not contain '..'".into()));
    }
    let candidate = if req.is_empty() {
        root.to_path_buf()
    } else if Path::new(req).is_absolute() {
        PathBuf::from(req)
    } else {
        root.join(req)
    };
    let canon = candidate
        .canonicalize()
        .map_err(|e| AppError::NotFound(format!("cwd not found: {e}")))?;
    let root_canon = root
        .canonicalize()
        .map_err(|_| AppError::Internal("project_root canonicalize".into()))?;
    if !canon.starts_with(&root_canon) {
        return Err(AppError::BadRequest("cwd escapes project_root".into()));
    }
    if !canon.is_dir() {
        return Err(AppError::BadRequest("cwd is not a directory".into()));
    }
    Ok(canon)
}

/// Per-connection driver: spawn shell + bridge bytes both ways. The
/// `PtyBag` returned by `spawn_pty` carries the child + reader cancel flag
/// + writer; dropping it at the end of this function tears everything down
/// (kill + wait + abort reader).
async fn handle_socket(socket: WebSocket, cwd: PathBuf) {
    let (mut ws_tx, ws_rx) = socket.split();
    let (out_tx, out_rx) = mpsc::channel::<Vec<u8>>(64);
    let bag = match spawn_pty(&cwd, out_tx) {
        Ok(p) => p,
        Err(e) => {
            let _ = ws_tx
                .send(Message::Text(format!("\r\n[term: spawn failed: {e}]\r\n")))
                .await;
            return;
        }
    };
    bridge_io(ws_tx, ws_rx, bag, out_rx).await;
    // bag drops here → reader cancel + child kill+wait, no zombies.
}

/// Bidirectional bridge: PTY-stdout → WS-out (binary frames) and WS-in →
/// PTY-stdin until either side closes. Returns when the loop exits.
async fn bridge_io(
    mut ws_tx: futures::stream::SplitSink<WebSocket, Message>,
    mut ws_rx: futures::stream::SplitStream<WebSocket>,
    mut bag: PtyBag,
    mut out_rx: mpsc::Receiver<Vec<u8>>,
) {
    loop {
        tokio::select! {
            maybe_chunk = out_rx.recv() => match maybe_chunk {
                Some(bytes) => {
                    if ws_tx.send(Message::Binary(bytes)).await.is_err() { break; }
                }
                None => break,
            },
            maybe_msg = ws_rx.next() => match maybe_msg {
                Some(Ok(Message::Text(t))) => { let _ = bag.writer.write_all(t.as_bytes()); }
                Some(Ok(Message::Binary(b))) => { let _ = bag.writer.write_all(&b); }
                Some(Ok(Message::Close(_))) | None => break,
                Some(Ok(_)) => continue,
                Some(Err(_)) => break,
            },
        }
    }
    // Drop bag explicitly to make teardown order obvious; the cancel-token
    // signal stops the reader, then kill+wait reaps the child shell.
    drop(bag);
}

#[cfg(test)]
#[path = "term_test.rs"]
mod tests;
