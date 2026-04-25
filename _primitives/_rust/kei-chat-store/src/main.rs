//! kei-chat-store CLI.

use clap::{Parser, Subcommand};
use kei_chat_store::search::search;
use kei_chat_store::sessions::{archive_session, save_message, start_session, ChatMessage};
use kei_chat_store::stats::stats;
use kei_chat_store::Store;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "kei-chat-store", version)]
struct Cli {
    #[arg(long)] db: Option<PathBuf>,
    #[command(subcommand)] cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Start { #[arg(long)] project: String,
            #[arg(long, default_value = "")] title: String,
            #[arg(long, default_value = "")] model: String },
    Save { #[arg(long)] session_id: String,
           #[arg(long)] role: String,
           content: String,
           #[arg(long, default_value_t = 0)] tokens_in: i64,
           #[arg(long, default_value_t = 0)] tokens_out: i64,
           #[arg(long, default_value_t = 0.0)] cost: f64 },
    Search { query: String, #[arg(long, default_value_t = 20)] limit: i64 },
    Archive { session_id: String },
    Stats,
}

fn db_path(o: Option<PathBuf>) -> PathBuf {
    if let Some(p) = o { return p; }
    if let Ok(e) = std::env::var("KEI_CHAT_DB") { return PathBuf::from(e); }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".claude/chat/chat.sqlite")
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let s = Store::open(&db_path(cli.db))?;
    match cli.cmd {
        Cmd::Start { project, title, model } => {
            println!("{}", start_session(&s, &project, &title, &model)?);
        }
        Cmd::Save { session_id, role, content, tokens_in, tokens_out, cost } => {
            let id = save_message(&s, &ChatMessage {
                session_id, role, content, tokens_in, tokens_out, cost,
                ..Default::default()
            })?;
            println!("{}", id);
        }
        Cmd::Search { query, limit } => {
            for m in search(&s, &query, limit)? {
                println!("{}\t{}\t{}", m.id, m.role, m.content);
            }
        }
        Cmd::Archive { session_id } => {
            archive_session(&s, &session_id)?;
            println!("archived {}", session_id);
        }
        Cmd::Stats => {
            let st = stats(&s)?;
            println!("{}", serde_json::to_string_pretty(&st)?);
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => { eprintln!("kei-chat-store: {e:#}"); ExitCode::from(1) }
    }
}
