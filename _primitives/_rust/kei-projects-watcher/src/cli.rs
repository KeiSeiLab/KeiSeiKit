//! CLI subcommand dispatch — kept out of `main.rs` to honour the
//! Constructor-Pattern file-size budget.

use std::path::PathBuf;
use std::time::Duration;

use anyhow::{Context, Result};
use rusqlite::Connection;
use tokio::signal::unix::{signal, SignalKind};
use tracing::{error, info, warn};

use crate::watcher::Watcher;

/// Open the index DB and ensure its schema is migrated.
pub fn open_db(path: &PathBuf) -> Result<Connection> {
    if let Some(p) = path.parent() {
        std::fs::create_dir_all(p).ok();
    }
    let conn = Connection::open(path).with_context(|| format!("open sqlite {:?}", path))?;
    kei_projects_index::schema::init(&conn).context("schema::init failed")?;
    Ok(conn)
}

/// Daemon entry: initial rebuild, then watch loop until SIGINT/SIGTERM.
pub async fn cmd_run(db: PathBuf, root: PathBuf, debounce: Duration) -> Result<()> {
    let conn = open_db(&db)?;
    info!(?db, ?root, ?debounce, "kei-projects-watcher starting");
    if let Err(e) = kei_projects_index::index::rebuild_index_with_conn(&conn, &root) {
        warn!(error=?e, "initial rebuild_index failed; continuing live-watch");
    }
    let mut watcher = Watcher::new(root.clone(), debounce).context("watcher init")?;
    let events = watcher.events();
    watch_loop(&conn, events).await?;
    info!("kei-projects-watcher stopping");
    Ok(())
}

/// Receive debounced project paths and re-index until a signal arrives.
async fn watch_loop(
    conn: &Connection,
    mut events: tokio::sync::mpsc::Receiver<PathBuf>,
) -> Result<()> {
    let mut sigterm = signal(SignalKind::terminate()).context("SIGTERM handler")?;
    loop {
        tokio::select! {
            maybe = events.recv() => {
                let Some(project) = maybe else { break };
                match kei_projects_index::index::reindex_one(conn, &project) {
                    Ok(()) => info!(?project, "reindexed"),
                    Err(e) => error!(?project, error=?e, "reindex_one failed"),
                }
            }
            _ = tokio::signal::ctrl_c() => { info!("SIGINT"); break; }
            _ = sigterm.recv()         => { info!("SIGTERM"); break; }
        }
    }
    Ok(())
}

/// Print last-indexed-ts of each project as pretty JSON to stdout.
pub fn cmd_status(db: PathBuf) -> Result<()> {
    let conn = open_db(&db)?;
    let mut stmt = conn
        .prepare("SELECT path, last_indexed_ts FROM projects ORDER BY path")
        .context("prepare status query")?;
    let rows: Vec<serde_json::Value> = stmt
        .query_map([], |r| {
            Ok(serde_json::json!({
                "path": r.get::<_, String>(0)?,
                "last_indexed_ts": r.get::<_, i64>(1)?,
            }))
        })
        .context("status query")?
        .collect::<rusqlite::Result<_>>()?;
    println!("{}", serde_json::to_string_pretty(&rows)?);
    Ok(())
}
