//! Cross-cutting "what is alive right now" view.
//!
//! Constructor Pattern: pure read-side cube. Joins three sources for a
//! single dashboard:
//!   1. `blocks` table from kei-registry — atoms, skills, rules, hooks,
//!      primitives, plus path-atoms (atoms whose source file is
//!      `_blocks/path-*.md`).
//!   2. `agents` table from `~/.claude/agents/ledger.sqlite` if present —
//!      agent forks per RULE 0.12, with status (running / done / failed /
//!      merged / rejected).
//!   3. `git for-each-ref refs/heads` shell-out — local branches with
//!      `ahead`, `behind` and `dirty` flags relative to their upstream.
//!
//! No I/O beyond DB reads + one git invocation. No writes. The handler
//! formats the gathered struct into either an ASCII table (default) or
//! JSON (`--format json`).

use anyhow::{Context, Result};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::block::BlockType;

/// Aggregate snapshot returned by `compute_status`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Status {
    pub blocks_by_type: BTreeMap<String, u64>,
    pub path_atoms: Vec<PathAtomRow>,
    pub branches: Vec<BranchRow>,
    pub agents: Option<AgentSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathAtomRow {
    pub name: String,
    pub dna_prefix: String,
    pub body_sha8: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchRow {
    pub name: String,
    pub current: bool,
    pub upstream: Option<String>,
    pub ahead: u32,
    pub behind: u32,
    pub last_commit: String,
    /// Deterministic DNA-style identifier for the branch. Format
    /// `branch::git::<sha8(branch_name)>::<sha8(commit_sha)>`. Computed
    /// on-the-fly from `(name, last_commit)` so it survives without DB
    /// persistence — the underlying truth lives in `.git/refs/heads/<name>`.
    pub dna: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentSummary {
    pub running: u64,
    pub done: u64,
    pub failed: u64,
    pub merged: u64,
    pub rejected: u64,
}

/// Compute the full status snapshot. `git_repo` is the path to scan for
/// branches (typically the current working directory). `ledger_db` is
/// the optional path to `~/.claude/agents/ledger.sqlite`; if it doesn't
/// exist, `agents` field is `None`.
pub fn compute_status(
    conn: &Connection,
    git_repo: Option<&Path>,
    ledger_db: Option<&Path>,
) -> Result<Status> {
    let mut s = Status::default();
    s.blocks_by_type = block_counts(conn)?;
    s.path_atoms = path_atom_rows(conn)?;
    if let Some(repo) = git_repo {
        s.branches = git_branches(repo).unwrap_or_default();
    }
    if let Some(db) = ledger_db {
        if db.exists() {
            s.agents = ledger_agent_summary(db).ok();
        }
    }
    Ok(s)
}

fn block_counts(conn: &Connection) -> Result<BTreeMap<String, u64>> {
    let mut out = BTreeMap::new();
    for bt in BlockType::all() {
        let mut stmt = conn
            .prepare(
                "SELECT COUNT(*) FROM blocks \
                 WHERE block_type = ?1 AND superseded_by IS NULL",
            )
            .context("prepare block_counts")?;
        let n: i64 = stmt
            .query_row(rusqlite::params![bt.as_str()], |r| r.get(0))
            .context("query block_counts")?;
        out.insert(bt.as_str().to_string(), n as u64);
    }
    Ok(out)
}

fn path_atom_rows(conn: &Connection) -> Result<Vec<PathAtomRow>> {
    // Convention: path-atoms are atoms whose source file matches
    // `_blocks/path-<name>.md`. SQL LIKE keeps it server-side; the
    // resulting rows are sorted by name for stable output.
    let mut stmt = conn
        .prepare(
            "SELECT name, dna, body_sha FROM blocks \
             WHERE block_type = 'atom' \
             AND superseded_by IS NULL \
             AND path LIKE '%/_blocks/path-%.md' \
             ORDER BY name",
        )
        .context("prepare path_atom_rows")?;
    let rows = stmt
        .query_map([], |r| {
            let dna: String = r.get(1)?;
            let dna_prefix = dna_prefix(&dna);
            Ok(PathAtomRow {
                name: r.get(0)?,
                dna_prefix,
                body_sha8: r.get(2)?,
            })
        })?
        .filter_map(Result::ok)
        .collect();
    Ok(rows)
}

/// Compute a deterministic DNA-style identifier for a git branch. Mirrors
/// the kei-shared wire format `<role>::<caps>::<scope_sha8>::<body_sha8>`:
/// role is fixed `branch`, caps is fixed `git`, scope_sha is the first 8
/// hex chars of `sha256(branch_name)`, body_sha is the first 8 chars of
/// the commit SHA (which is itself a SHA-1 prefix). The pair is unique
/// per (name, head_commit) so the DNA changes on every commit, mirroring
/// the immutable-content invariant atoms have.
fn compute_branch_dna(name: &str, commit_sha: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(name.as_bytes());
    let name_sha = format!("{:x}", h.finalize());
    let scope8 = &name_sha[..8];
    let body8 = commit_sha
        .get(..8)
        .unwrap_or(commit_sha)
        .to_ascii_lowercase();
    format!("branch::git::{scope8}::{body8}")
}

/// Take the first three segments of a `<role>::<caps>::<scope_sha8>::...`
/// DNA so the displayed prefix is readable but identifying.
fn dna_prefix(dna: &str) -> String {
    let mut parts = dna.split("::").take(3).collect::<Vec<_>>();
    if parts.len() < 3 {
        return dna.to_string();
    }
    parts.push("…");
    parts.join("::")
}

fn git_branches(repo: &Path) -> Result<Vec<BranchRow>> {
    let current_branch = run_git(repo, &["rev-parse", "--abbrev-ref", "HEAD"]).ok();
    let out = run_git(
        repo,
        &[
            "for-each-ref",
            "--format=%(refname:short)\t%(upstream:short)\t%(upstream:track,nobracket)\t%(objectname:short)",
            "refs/heads",
        ],
    )?;
    let mut rows = Vec::new();
    for line in out.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 4 {
            continue;
        }
        let name = parts[0].to_string();
        let upstream = if parts[1].is_empty() {
            None
        } else {
            Some(parts[1].to_string())
        };
        let (ahead, behind) = parse_track(parts[2]);
        let last_commit = parts[3].to_string();
        let dna = compute_branch_dna(&name, &last_commit);
        rows.push(BranchRow {
            current: current_branch.as_deref() == Some(&name),
            name,
            upstream,
            ahead,
            behind,
            last_commit,
            dna,
        });
    }
    Ok(rows)
}

/// Parse `upstream:track,nobracket` output. Examples:
/// `""` (in sync), `"ahead 3"`, `"behind 1"`, `"ahead 3, behind 1"`,
/// `"gone"` (upstream deleted).
fn parse_track(s: &str) -> (u32, u32) {
    let mut ahead = 0u32;
    let mut behind = 0u32;
    for part in s.split(',') {
        let part = part.trim();
        if let Some(n) = part.strip_prefix("ahead ") {
            ahead = n.parse().unwrap_or(0);
        } else if let Some(n) = part.strip_prefix("behind ") {
            behind = n.parse().unwrap_or(0);
        }
    }
    (ahead, behind)
}

fn run_git(repo: &Path, args: &[&str]) -> Result<String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()
        .context("spawn git")?;
    if !out.status.success() {
        anyhow::bail!(
            "git {:?} failed: {}",
            args,
            String::from_utf8_lossy(&out.stderr)
        );
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim_end().to_string())
}

fn ledger_agent_summary(db: &Path) -> Result<AgentSummary> {
    let conn = rusqlite::Connection::open_with_flags(
        db,
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
    )
    .context("open ledger DB read-only")?;
    let mut s = AgentSummary::default();
    for (status, slot) in [
        ("running", &mut s.running),
        ("done", &mut s.done),
        ("failed", &mut s.failed),
        ("merged", &mut s.merged),
        ("rejected", &mut s.rejected),
    ] {
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM agents WHERE status = ?1")?;
        let n: i64 = stmt.query_row(rusqlite::params![status], |r| r.get(0))?;
        *slot = n as u64;
    }
    Ok(s)
}

/// Render `Status` as a multi-section ASCII report.
pub fn render_ascii(s: &Status) -> String {
    let mut out = String::new();
    out.push_str("=== Substrate Status ===\n\n");
    out.push_str("[Blocks — active count by type]\n");
    for (k, v) in &s.blocks_by_type {
        out.push_str(&format!("  {:<14} {}\n", k, v));
    }
    out.push('\n');

    out.push_str(&format!("[Path Atoms — {}]\n", s.path_atoms.len()));
    for p in &s.path_atoms {
        out.push_str(&format!(
            "  {:<14} {:<28} body:{}\n",
            p.name, p.dna_prefix, p.body_sha8
        ));
    }
    if s.path_atoms.is_empty() {
        out.push_str("  (none registered)\n");
    }
    out.push('\n');

    out.push_str(&format!("[Local Branches — {}]\n", s.branches.len()));
    for b in &s.branches {
        let marker = if b.current { "*" } else { " " };
        let track = match (b.ahead, b.behind) {
            (0, 0) => "in sync".to_string(),
            (a, 0) => format!("ahead {a}"),
            (0, b_) => format!("behind {b_}"),
            (a, b_) => format!("ahead {a}, behind {b_}"),
        };
        let upstream = b.upstream.as_deref().unwrap_or("(none)");
        out.push_str(&format!(
            "  {} {:<40} → {:<25} {} @ {}  {}\n",
            marker, b.name, upstream, track, b.last_commit, dna_prefix(&b.dna)
        ));
    }
    out.push('\n');

    if let Some(a) = &s.agents {
        out.push_str("[Agent Forks — kei-ledger]\n");
        out.push_str(&format!(
            "  running:{}  done:{}  merged:{}  failed:{}  rejected:{}\n",
            a.running, a.done, a.merged, a.failed, a.rejected
        ));
    } else {
        out.push_str("[Agent Forks]\n  (no kei-ledger DB found)\n");
    }
    out.push('\n');
    out
}

/// Default ledger path: `$KEI_LEDGER_DB` or `~/.claude/agents/ledger.sqlite`.
pub fn default_ledger_path() -> PathBuf {
    if let Some(v) = std::env::var_os("KEI_LEDGER_DB") {
        return PathBuf::from(v);
    }
    let home = std::env::var_os("HOME").unwrap_or_default();
    PathBuf::from(home).join(".claude/agents/ledger.sqlite")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_track_in_sync() {
        assert_eq!(parse_track(""), (0, 0));
    }

    #[test]
    fn parse_track_ahead() {
        assert_eq!(parse_track("ahead 3"), (3, 0));
    }

    #[test]
    fn parse_track_behind() {
        assert_eq!(parse_track("behind 7"), (0, 7));
    }

    #[test]
    fn parse_track_both() {
        assert_eq!(parse_track("ahead 3, behind 1"), (3, 1));
    }

    #[test]
    fn parse_track_gone_treated_as_zero() {
        // Upstream deleted — git emits "gone"; we don't surface it as
        // ahead/behind, so callers see (0, 0). Acceptable for a status
        // dashboard; a future field could carry it explicitly.
        assert_eq!(parse_track("gone"), (0, 0));
    }

    #[test]
    fn dna_prefix_three_segments() {
        let dna = "atom::md::1a771d51::b8f9e85f-abc12345";
        assert_eq!(dna_prefix(dna), "atom::md::1a771d51::…");
    }

    #[test]
    fn branch_dna_is_deterministic_and_well_formed() {
        let dna = compute_branch_dna("feat/foo-bar", "3422bdca12d4567");
        assert!(dna.starts_with("branch::git::"));
        let parts: Vec<&str> = dna.split("::").collect();
        assert_eq!(parts.len(), 4);
        assert_eq!(parts[0], "branch");
        assert_eq!(parts[1], "git");
        assert_eq!(parts[2].len(), 8); // sha8 of branch name
        assert_eq!(parts[3], "3422bdca"); // first 8 of commit
        // determinism: same input → same DNA
        let dna2 = compute_branch_dna("feat/foo-bar", "3422bdca12d4567");
        assert_eq!(dna, dna2);
    }

    #[test]
    fn branch_dna_changes_on_commit() {
        let a = compute_branch_dna("main", "aaaaaaaa1111");
        let b = compute_branch_dna("main", "bbbbbbbb2222");
        assert_ne!(a, b, "DNA should change when commit changes");
    }

    #[test]
    fn branch_dna_changes_on_rename() {
        let a = compute_branch_dna("main", "deadbeef");
        let b = compute_branch_dna("trunk", "deadbeef");
        assert_ne!(a, b, "DNA should change when name changes");
    }

    #[test]
    fn render_ascii_empty_status_has_all_sections() {
        let s = Status::default();
        let out = render_ascii(&s);
        assert!(out.contains("Blocks"));
        assert!(out.contains("Path Atoms"));
        assert!(out.contains("Local Branches"));
        assert!(out.contains("Agent Forks"));
        assert!(out.contains("(none registered)"));
        assert!(out.contains("(no kei-ledger DB found)"));
    }
}
