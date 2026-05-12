// SPDX-License-Identifier: Apache-2.0
//! kei-buddy binary — 4 subcommands: serve / migrate / webhook-set / webhook-delete.

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "kei-buddy",
    about = "KeiBuddy personal-assistant Telegram bot",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Start the Telegram webhook HTTP listener.
    Serve,
    /// Apply the SQLite schema (idempotent). Useful before first run.
    Migrate,
    /// Register a webhook URL with Telegram.
    WebhookSet {
        /// Public HTTPS URL for the /webhook route.
        url: String,
    },
    /// Delete the registered Telegram webhook (revert to polling).
    WebhookDelete,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Serve => cmd_serve().await,
        Command::Migrate => cmd_migrate(),
        Command::WebhookSet { url } => cmd_webhook_set(url).await,
        Command::WebhookDelete => cmd_webhook_delete().await,
    }
}

#[cfg(feature = "serve")]
async fn cmd_serve() -> anyhow::Result<()> {
    use kei_buddy::serve::{run_serve, ServeConfig};
    let cfg = ServeConfig {
        port: port_from_env(),
        db_path: db_path_from_env(),
        bot_token: require_env("TELEGRAM_BOT_TOKEN")?,
        webhook_secret: require_env("TELEGRAM_WEBHOOK_SECRET")?,
        allowed_chat_ids: allowed_chat_ids_from_env(),
        llm_proxy_url: std::env::var("KEI_BUDDY_LLM_PROXY")
            .ok()
            .or_else(|| Some("https://api.openai.com".to_string())),
        llm_api_key: std::env::var("KEI_BUDDY_LLM_KEY")
            .ok()
            .or_else(|| std::env::var("OPENAI_API_KEY").ok()),
        llm_model: std::env::var("KEI_BUDDY_LLM_MODEL").ok(),
        chat_log_db_path: chat_log_path_from_env(),
    };
    run_serve(cfg).await
}

/// Parse `KEI_BUDDY_ALLOWED_CHAT_IDS` CSV → Some(Vec<i64>); empty/missing → None.
fn allowed_chat_ids_from_env() -> Option<Vec<i64>> {
    let raw = std::env::var("KEI_BUDDY_ALLOWED_CHAT_IDS").ok()?;
    let list: Vec<i64> = raw
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();
    if list.is_empty() {
        None
    } else {
        Some(list)
    }
}

#[cfg(not(feature = "serve"))]
async fn cmd_serve() -> anyhow::Result<()> {
    anyhow::bail!("kei-buddy was compiled without the `serve` feature. Rebuild with --features serve.");
}

fn cmd_migrate() -> anyhow::Result<()> {
    let path = db_path_from_env();
    let _store = kei_buddy::store::SqliteBuddyStore::from_path(&path)?;
    let chat_log_path = chat_log_path_from_env();
    let _ = kei_buddy::ChatLog::from_path(&chat_log_path)?;
    init_log();
    tracing::info!(path = %path, chat_log_path = %chat_log_path, "schema applied");
    Ok(())
}

fn init_log() {
    #[cfg(feature = "serve")]
    {
        use tracing_subscriber::{fmt, EnvFilter};
        let _ = fmt().with_env_filter(EnvFilter::from_default_env()).try_init();
    }
}

#[cfg(feature = "serve")]
async fn cmd_webhook_set(url: String) -> anyhow::Result<()> {
    use kei_buddy::serve_telegram::set_webhook;
    let token = require_env("TELEGRAM_BOT_TOKEN")?;
    let secret = require_env("TELEGRAM_WEBHOOK_SECRET")?;
    let http = reqwest::Client::new();
    set_webhook(&token, &url, &secret, &http).await?;
    tracing::info!("webhook registered");
    Ok(())
}

#[cfg(not(feature = "serve"))]
async fn cmd_webhook_set(_url: String) -> anyhow::Result<()> {
    anyhow::bail!("compile with --features serve");
}

#[cfg(feature = "serve")]
async fn cmd_webhook_delete() -> anyhow::Result<()> {
    use kei_buddy::serve_telegram::delete_webhook;
    let token = require_env("TELEGRAM_BOT_TOKEN")?;
    let http = reqwest::Client::new();
    delete_webhook(&token, &http).await?;
    tracing::info!("webhook deleted");
    Ok(())
}

#[cfg(not(feature = "serve"))]
async fn cmd_webhook_delete() -> anyhow::Result<()> {
    anyhow::bail!("compile with --features serve");
}

fn require_env(name: &str) -> anyhow::Result<String> {
    std::env::var(name).map_err(|_| anyhow::anyhow!("env var {name} is required"))
}

fn port_from_env() -> u16 {
    std::env::var("KEI_BUDDY_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8080)
}

fn db_path_from_env() -> String {
    std::env::var("KEI_BUDDY_DB_PATH").unwrap_or_else(|_| "./kei-buddy.db".into())
}

fn chat_log_path_from_env() -> String {
    std::env::var("KEI_BUDDY_CHAT_LOG_PATH").unwrap_or_else(|_| "./kei-buddy-chat.db".into())
}
