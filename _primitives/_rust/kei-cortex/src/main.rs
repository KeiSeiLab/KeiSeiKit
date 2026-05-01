//! `kei-cortex` CLI — `serve` subcommand starts the daemon.
//!
//! Token is auto-generated on first launch if missing. The daemon binds to
//! `127.0.0.1:<port>` only; public binding is forbidden by design.

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use kei_cortex::{auth, build_router, AppConfig, AppState};
use tokio::net::TcpListener;

#[derive(Parser, Debug)]
#[command(name = "kei-cortex", about = "Local HTTP daemon exposing cortex state")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Start the daemon on 127.0.0.1.
    Serve(ServeArgs),
}

#[derive(clap::Args, Debug)]
struct ServeArgs {
    #[arg(long, default_value_t = kei_cortex::config::DEFAULT_PORT)]
    port: u16,

    #[arg(long, default_value_t = kei_cortex::config::DEFAULT_CORS_ORIGIN.to_string())]
    cors_origin: String,

    #[arg(long)]
    token_path: Option<PathBuf>,

    #[arg(long)]
    ledger_path: Option<PathBuf>,

    #[arg(long)]
    pet_root: Option<PathBuf>,

    #[arg(long)]
    memory_db: Option<PathBuf>,

    /// Directory containing bundled Cubism sample rigs (haru/, mao/, hiyori/,
    /// mark/). Used by the portrait-stylize endpoint as the clone source.
    #[arg(long)]
    live2d_samples_dir: Option<PathBuf>,

    /// Process working directory used to discover CLAUDE.md / AGENTS.md
    /// context files (walked upward). Defaults to `std::env::current_dir`.
    #[arg(long)]
    cwd: Option<PathBuf>,

    /// Project root used as the chroot for `/fs/list` + `/term` and as
    /// the search base for `/skill-name` resolution. Defaults to `cwd`.
    #[arg(long)]
    project_root: Option<PathBuf>,

    /// Default LLM provider used when the chat request omits `?provider=`.
    #[arg(long, default_value_t = kei_cortex::config::DEFAULT_PROVIDER.to_string())]
    default_provider: String,

    /// SQLite database for per-turn token-event telemetry. Defaults to
    /// `~/.keisei/token-events.sqlite`. Override the path via this flag
    /// for nightly sleep-report integration.
    #[arg(long)]
    token_tracker_db: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Serve(args) => serve(args).await,
    }
}

async fn serve(args: ServeArgs) -> Result<()> {
    let config = AppConfig::try_new(
        Some(args.port),
        Some(args.cors_origin),
        args.token_path,
        args.ledger_path,
        args.pet_root,
        args.memory_db,
        args.live2d_samples_dir,
        args.cwd,
        args.project_root,
        Some(args.default_provider),
        args.token_tracker_db,
    )
    .context("assemble config")?;
    warn_if_live2d_missing(&config.live2d_samples_dir);
    let token = load_or_bootstrap_token(&config.token_path)?;
    let state = AppState::new(config.clone(), token);
    let router = build_router(state);
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), config.port);
    let listener = TcpListener::bind(addr)
        .await
        .with_context(|| format!("bind {addr}"))?;
    eprintln!("kei-cortex listening on http://{addr}");
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("axum serve")?;
    Ok(())
}

/// Emit a warning if the live2d samples dir is missing. Not fatal — the
/// portrait-stylize endpoint is optional — but a clear heads-up beats
/// a cryptic 500 on the first upload.
fn warn_if_live2d_missing(dir: &std::path::Path) {
    if !dir.is_dir() {
        eprintln!(
            "warn: live2d samples dir not found at {dir:?} — portrait uploads will fail \
             with 500. Pass --live2d-samples-dir to override."
        );
    }
}

fn load_or_bootstrap_token(path: &std::path::Path) -> Result<String> {
    if path.exists() {
        Ok(auth::load_token(path).with_context(|| format!("load token from {path:?}"))?)
    } else {
        let token = auth::generate_token();
        auth::save_token(path, &token).with_context(|| format!("save token to {path:?}"))?;
        eprintln!("generated new bearer token at {path:?}");
        Ok(token)
    }
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}
