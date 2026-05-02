use anyhow::{bail, Result};
use axum::{Router, routing::get};
use clap::Parser;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::broadcast;

use kei_graph_stream::{AliveState, tail, ws};

#[derive(Parser, Debug)]
#[command(name = "kei-graph-stream", about = "Stream agent events to browser via WebSocket")]
struct Cli {
    #[arg(long, env = "KEI_GRAPH_STREAM_BIND", default_value = "127.0.0.1:8201")]
    bind: SocketAddr,

    #[arg(long, env = "KEI_EVENTS_FILE")]
    events_file: Option<PathBuf>,

    /// Allow binding to a non-loopback address. Without this flag,
    /// kei-graph-stream refuses to start on a non-loopback bind address
    /// to prevent accidental exposure of the WebSocket endpoint.
    #[arg(long)]
    public_bind_i_accept_the_leak: bool,
}

fn default_events_file() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
    PathBuf::from(home).join(".claude/memory/agent-events.jsonl")
}

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var("KEI_GRAPH_STREAM_BYPASS").as_deref() == Ok("1") {
        eprintln!("[kei-graph-stream] bypass mode — exiting");
        return Ok(());
    }

    let cli = Cli::parse();

    if !cli.bind.ip().is_loopback() && !cli.public_bind_i_accept_the_leak {
        bail!(
            "kei-graph-stream: refusing to bind {}: non-loopback bind requires \
             explicit --public-bind-i-accept-the-leak flag",
            cli.bind
        );
    }

    let events_file = cli.events_file.unwrap_or_else(default_events_file);

    if let Some(parent) = events_file.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    if !events_file.exists() {
        tokio::fs::write(&events_file, b"").await?;
    }

    let (tx, _rx) = broadcast::channel::<String>(256);
    let tx = Arc::new(tx);
    let alive = Arc::new(AliveState::new());

    tokio::spawn(tail::run(events_file, Arc::clone(&tx), Arc::clone(&alive)));

    let app = build_router(Arc::clone(&tx), Arc::clone(&alive));
    let listener = tokio::net::TcpListener::bind(cli.bind).await?;
    eprintln!("[kei-graph-stream] listening on {}", cli.bind);
    axum::serve(listener, app).await?;
    Ok(())
}

fn build_router(
    tx: Arc<broadcast::Sender<String>>,
    alive: Arc<AliveState>,
) -> Router {
    Router::new()
        .route("/stream", get(ws::ws_handler))
        .route("/health", get(health_handler))
        .with_state((tx, alive))
}

async fn health_handler() -> &'static str {
    "kei-graph-stream alive\n"
}
