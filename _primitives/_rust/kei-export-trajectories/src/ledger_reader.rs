//! Read agent rows from `kei-ledger.sqlite`, optionally enrich with tool
//! events from `kei-memory.sqlite`, and read chatlog text from
//! `.claude/agents/<id>/chatlog.md`.
//!
//! Constructor Pattern: ledger query + chatlog file read here; memory
//! event queries live in the `memory_events` sibling cube. Each helper
//! <30 LOC; missing inputs degrade gracefully (missing chatlog = empty
//! string; missing memory DB = zero tool events).
//!
//! HERMES P0.2 deviation note: the spec said "extract tool events from
//! agent_runs.events JSON" — but the ledger schema has no events column.
//! Tool events live in the SIBLING `kei-memory.sqlite` `events` table
//! (rusqlite ATTACH avoided to keep the two stores independently
//! migratable). We open them as separate connections.

use crate::memory_events::query_tool_events;
use crate::tool_stats::ToolEvent;
use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use std::path::{Path, PathBuf};

/// One agent's complete trajectory record, ready to convert to ShareGPT.
#[derive(Debug, Clone)]
pub struct TrajectoryRecord {
    pub agent_id: String,
    pub branch: String,
    pub status: String,
    pub started_ts: i64,
    pub finished_ts: Option<i64>,
    pub summary: Option<String>,
    pub dna: Option<String>,
    pub spec_text: String,
    pub chatlog_text: String,
    pub tool_events: Vec<ToolEvent>,
}

impl TrajectoryRecord {
    /// `completed` per Hermes spec: terminal-state-with-success.
    /// `done` and `merged` count; `failed`/`running`/`rejected` do not.
    pub fn completed(&self) -> bool {
        matches!(self.status.as_str(), "done" | "merged")
    }
}

/// Reads a single ledger DB; optionally cross-references a memory DB and
/// a repo root for chatlog files. All paths are absolute.
#[derive(Debug, Clone)]
pub struct LedgerReader {
    pub ledger_path: PathBuf,
    pub memory_path: Option<PathBuf>,
    pub repo_root: Option<PathBuf>,
}

impl LedgerReader {
    /// Construct with explicit ledger path. Memory + repo are optional —
    /// `with_memory` / `with_repo_root` chain on top.
    pub fn new(ledger_path: impl Into<PathBuf>) -> Self {
        Self {
            ledger_path: ledger_path.into(),
            memory_path: None,
            repo_root: None,
        }
    }

    pub fn with_memory(mut self, p: impl Into<PathBuf>) -> Self {
        self.memory_path = Some(p.into());
        self
    }

    pub fn with_repo_root(mut self, p: impl Into<PathBuf>) -> Self {
        self.repo_root = Some(p.into());
        self
    }

    /// Materialize every agent in the ledger that has `started_ts >=
    /// from_ts`, in started_ts-ascending order (deterministic for tests).
    pub fn read_since(&self, from_ts: i64) -> Result<Vec<TrajectoryRecord>> {
        let conn = open_ledger(&self.ledger_path)?;
        let rows = query_agents(&conn, from_ts)?;
        let mem = self
            .memory_path
            .as_ref()
            .map(|p| Connection::open(p).with_context(|| format!("open memory at {}", p.display())))
            .transpose()?;
        let mut out = Vec::with_capacity(rows.len());
        for r in rows {
            out.push(self.hydrate(r, mem.as_ref())?);
        }
        Ok(out)
    }

    /// Just count rows >= from_ts; fast path for the `count` subcommand.
    pub fn count_since(&self, from_ts: i64) -> Result<u64> {
        let conn = open_ledger(&self.ledger_path)?;
        let n: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM agents WHERE started_ts >= ?1",
                params![from_ts],
                |r| r.get(0),
            )
            .context("count agents")?;
        Ok(n as u64)
    }

    fn hydrate(&self, raw: RawAgent, mem: Option<&Connection>) -> Result<TrajectoryRecord> {
        let chatlog_text = read_artefact(self.repo_root.as_deref(), &raw.id, "chatlog.md");
        let spec_text = read_artefact(self.repo_root.as_deref(), &raw.id, "spec.md");
        let tool_events = match mem {
            Some(c) => query_tool_events(c, &raw.id, raw.started_ts, raw.finished_ts)?,
            None => Vec::new(),
        };
        Ok(TrajectoryRecord {
            agent_id: raw.id,
            branch: raw.branch,
            status: raw.status,
            started_ts: raw.started_ts,
            finished_ts: raw.finished_ts,
            summary: raw.summary,
            dna: raw.dna,
            spec_text,
            chatlog_text,
            tool_events,
        })
    }
}

fn open_ledger(path: &Path) -> Result<Connection> {
    Connection::open(path).with_context(|| format!("open ledger at {}", path.display()))
}

/// Internal raw row from the ledger — only the columns we need.
#[derive(Debug, Clone)]
struct RawAgent {
    id: String,
    branch: String,
    status: String,
    started_ts: i64,
    finished_ts: Option<i64>,
    summary: Option<String>,
    dna: Option<String>,
}

fn query_agents(conn: &Connection, from_ts: i64) -> Result<Vec<RawAgent>> {
    let mut stmt = conn
        .prepare(
            "SELECT id, branch, status, started_ts, finished_ts, summary, dna
             FROM agents WHERE started_ts >= ?1 ORDER BY started_ts ASC",
        )
        .context("prepare agents query")?;
    let rows = stmt
        .query_map(params![from_ts], |r| {
            Ok(RawAgent {
                id: r.get(0)?,
                branch: r.get(1)?,
                status: r.get(2)?,
                started_ts: r.get(3)?,
                finished_ts: r.get(4)?,
                summary: r.get(5)?,
                dna: r.get(6)?,
            })
        })
        .context("query agents")?;
    let collected: rusqlite::Result<Vec<RawAgent>> = rows.collect();
    collected.context("collect agents")
}

/// Read `<repo>/.claude/agents/<id>/<filename>` if present; empty string
/// otherwise. Missing chatlog is normal (e.g. trivial agents per RULE
/// 0.12 exception list) — we never propagate the I/O error.
fn read_artefact(repo_root: Option<&Path>, agent_id: &str, name: &str) -> String {
    let Some(root) = repo_root else { return String::new() };
    let path = root.join(".claude").join("agents").join(agent_id).join(name);
    std::fs::read_to_string(&path).unwrap_or_default()
}
