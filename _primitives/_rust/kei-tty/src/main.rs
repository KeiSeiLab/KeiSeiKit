//! `kei-tty` binary entry point.
//!
//! Two subcommands:
//!
//! * `chat`  — interactive ratatui TUI (default mode for power users).
//! * `send`  — one-shot: read message from `--message` or stdin, stream
//!             response to stdout, exit. Pipe-friendly.
//!
//! Daemon URL is read from `KEI_DAEMON_URL` (default
//! `http://127.0.0.1:9797`). Bearer token is read from
//! `~/.keisei/cortex.token` (created by `keisei daemon init`); on first run
//! the file may not yet exist — we surface a clear error rather than
//! crashing.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use kei_tty::client::chat_stream;
use kei_tty::runner;
use kei_tty::types::ChatEvent;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, Read, Write};

/// Default cortex daemon URL when `KEI_DAEMON_URL` is unset.
const DEFAULT_URL: &str = "http://127.0.0.1:9797";

/// Default user_id for the cortex `pet` route — `keisei` daemon creates
/// this single user out of the box. Override with `--user-id` if needed.
const DEFAULT_USER_ID: &str = "default";

#[derive(Parser, Debug)]
#[command(name = "kei-tty", version, about = "Terminal UI client for kei-cortex")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Interactive TUI session (default mode).
    Chat {
        #[arg(long, default_value = DEFAULT_USER_ID)]
        user_id: String,
    },
    /// One-shot: send a single message and stream the reply to stdout.
    Send {
        /// Message body. If omitted, read from stdin.
        #[arg(long)]
        message: Option<String>,
        #[arg(long, default_value = DEFAULT_USER_ID)]
        user_id: String,
    },
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("kei-tty: {e:#}");
        std::process::exit(2);
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();
    let url = std::env::var("KEI_DAEMON_URL").unwrap_or_else(|_| DEFAULT_URL.into());
    let token = read_token()?;
    match cli.cmd {
        Cmd::Chat { user_id } => run_chat(url, token, user_id).await,
        Cmd::Send { message, user_id } => run_send(url, token, user_id, message).await,
    }
}

/// Read the bearer token from `~/.keisei/cortex.token`. The keisei daemon
/// writes this file with mode 0600 on first start.
fn read_token() -> Result<String> {
    let home = std::env::var("HOME").context("HOME env not set")?;
    let path = format!("{home}/.keisei/cortex.token");
    let raw = std::fs::read_to_string(&path)
        .with_context(|| format!("read {path} (start the daemon first?)"))?;
    Ok(raw.trim().to_string())
}

/// Enter the TUI: alternate screen + raw mode, run the event loop, then
/// always restore the terminal even on error.
async fn run_chat(url: String, token: String, user_id: String) -> Result<()> {
    enable_raw_mode().context("enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).context("enter alt screen")?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("init terminal")?;
    let res = runner::run(&mut terminal, url, token, user_id).await;
    let _ = disable_raw_mode();
    let _ = execute!(io::stdout(), LeaveAlternateScreen);
    res
}

/// One-shot mode: drains the SSE stream and prints token text to stdout.
async fn run_send(
    url: String,
    token: String,
    user_id: String,
    msg: Option<String>,
) -> Result<()> {
    let message = resolve_message(msg)?;
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let on_event = |ev: ChatEvent| emit_send_event(&mut handle, ev);
    chat_stream(&url, &token, &user_id, &message, None, on_event).await
}

/// Resolve `--message` or stdin into a non-empty string.
fn resolve_message(msg: Option<String>) -> Result<String> {
    let message = match msg {
        Some(m) => m,
        None => {
            let mut s = String::new();
            io::stdin().read_to_string(&mut s).context("read stdin")?;
            s.trim().to_string()
        }
    };
    if message.is_empty() {
        anyhow::bail!("empty message (pass --message or pipe via stdin)");
    }
    Ok(message)
}

/// Stream-event renderer for `send` mode (one event → stdout write).
fn emit_send_event<W: Write>(handle: &mut W, ev: ChatEvent) {
    match ev {
        ChatEvent::Token(t) => {
            let _ = handle.write_all(t.as_bytes());
            let _ = handle.flush();
        }
        ChatEvent::Error(m) => {
            let _ = writeln!(handle, "\n[error] {m}");
        }
        ChatEvent::Sentiment { tag, confidence } => {
            let _ = writeln!(handle, "\n[sentiment: {tag} ({:.0}%)]", confidence * 100.0);
        }
        _ => {}
    }
}
