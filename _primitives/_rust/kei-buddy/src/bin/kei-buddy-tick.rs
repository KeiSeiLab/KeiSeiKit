// SPDX-License-Identifier: Apache-2.0
//! `kei-buddy-tick` — oneshot digest-generation binary.
//!
//! Meant for systemd timer / cron. Reads env, calls `run_tick`,
//! prints JSON report to stdout, exits 0.

use kei_buddy::{run_tick, TickConfig};

#[tokio::main]
async fn main() {
    init_log();
    let cfg = cfg_from_env();
    match run_tick(cfg).await {
        Ok(report) => {
            let out = serde_json::json!({
                "topics_processed": report.topics_processed,
                "digests_created":  report.digests_created,
                "errors":           report.errors,
            });
            println!("{}", out);
        }
        Err(e) => {
            let out = serde_json::json!({
                "fatal": e.to_string(),
                "topics_processed": 0,
                "digests_created":  0,
                "errors":           [e.to_string()],
            });
            // Print to stdout so callers can parse; fatal still exits 0.
            println!("{}", out);
        }
    }
}

fn cfg_from_env() -> TickConfig {
    TickConfig {
        buddy_db_path: env_or("KEI_BUDDY_DB_PATH", "./kei-buddy.db"),
        chat_log_db_path: env_or("KEI_BUDDY_CHAT_LOG_PATH", "./kei-buddy-chat.db"),
        topics_db_path: env_or("KEI_BUDDY_TOPICS_DB_PATH", "./kei-buddy-topics.db"),
        since_hours: env_i64("KEI_BUDDY_TICK_SINCE_HOURS", 24),
        max_messages_per_topic: env_i64("KEI_BUDDY_TICK_MAX_MESSAGES", 50),
        llm_proxy_url: std::env::var("KEI_BUDDY_LLM_PROXY").ok(),
        llm_api_key: std::env::var("KEI_BUDDY_LLM_KEY")
            .ok()
            .or_else(|| std::env::var("OPENAI_API_KEY").ok()),
        llm_model: std::env::var("KEI_BUDDY_LLM_MODEL").ok(),
    }
}

fn env_or(name: &str, default: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| default.to_string())
}

fn env_i64(name: &str, default: i64) -> i64 {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn init_log() {
    #[cfg(feature = "serve")]
    {
        use tracing_subscriber::{fmt, EnvFilter};
        let _ = fmt().with_env_filter(EnvFilter::from_default_env()).try_init();
    }
}
