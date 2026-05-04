//! kei-comments — CLI dispatcher for the comment store.
//! Default DB: `~/.keisei/comments.sqlite`. Override via `--db <path>`.

use anyhow::Result;
use clap::{Parser, Subcommand};
use kei_comments::CommentStore;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "kei-comments", version, about = "Sovereign threaded comment store")]
struct Cli {
    /// Override DB path (default: ~/.keisei/comments.sqlite or $KEI_COMMENTS_DB).
    #[arg(long)]
    db: Option<PathBuf>,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Create the schema if missing.
    Migrate,
    /// Post a new comment.
    Post {
        #[arg(long)]
        page: String,
        #[arg(long)]
        author: String,
        #[arg(long)]
        body: String,
        #[arg(long)]
        parent: Option<String>,
    },
    /// List all comments for a page (JSON array on stdout).
    List {
        #[arg(long)]
        page: String,
    },
    /// Soft-delete a comment (only author may delete).
    Delete {
        #[arg(long)]
        id: String,
        #[arg(long)]
        author: String,
    },
    /// Toggle a reaction on. Idempotent.
    React {
        #[arg(long)]
        id: String,
        #[arg(long)]
        author: String,
        #[arg(long)]
        emoji: String,
    },
    /// Remove a reaction. Idempotent.
    Unreact {
        #[arg(long)]
        id: String,
        #[arg(long)]
        author: String,
        #[arg(long)]
        emoji: String,
    },
    /// Print reactions map (emoji → [authors]) for a comment as JSON.
    Reactions {
        #[arg(long)]
        id: String,
    },
}

fn db_path(cli_db: Option<PathBuf>) -> PathBuf {
    if let Some(p) = cli_db {
        return p;
    }
    if let Ok(env) = std::env::var("KEI_COMMENTS_DB") {
        return PathBuf::from(env);
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".keisei/comments.sqlite")
}

fn dispatch(store: &CommentStore, cmd: Cmd) -> Result<()> {
    match cmd {
        Cmd::Migrate => { store.migrate()?; println!("ok"); }
        Cmd::Post { page, author, body, parent } => {
            let id = store.post(&page, &author, &body, parent.as_deref())?;
            println!("{id}");
        }
        Cmd::List { page } => {
            println!("{}", serde_json::to_string(&store.list(&page)?)?);
        }
        Cmd::Delete { id, author } => {
            let ok = store.delete(&id, &author)?;
            println!("{}", if ok { "ok" } else { "denied" });
        }
        Cmd::React { id, author, emoji } => {
            store.react(&id, &author, &emoji)?; println!("ok");
        }
        Cmd::Unreact { id, author, emoji } => {
            store.unreact(&id, &author, &emoji)?; println!("ok");
        }
        Cmd::Reactions { id } => {
            println!("{}", serde_json::to_string(&store.reactions(&id)?)?);
        }
    }
    Ok(())
}

fn run(cli: Cli) -> Result<()> {
    let store = CommentStore::open(&db_path(cli.db))?;
    dispatch(&store, cli.cmd)
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(1)
        }
    }
}
