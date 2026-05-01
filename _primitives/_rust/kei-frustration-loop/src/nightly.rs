//! Phase-0 nightly scan — walks new chatlogs since timestamp, classifies
//! each user line via regex categories (the regex SSoT in `categories.rs`),
//! and appends every hit to the shared queue for morning review.
//!
//! The per-user `Firmware` argument is the user's baseline language model;
//! it is a passthrough today and reserved for future likelihood-ratio
//! filtering once enough confirmed feedback exists. Carrying it in the
//! signature now keeps the call sites stable across the v2 → v3 cutover.
//!
//! Constructor Pattern: this cube only orchestrates existing primitives
//! (jsonl parser + categories regex). Hit emission goes through the
//! `feedback`/`persistence` cubes; this file owns no IO format.

use anyhow::{Context, Result};
use frustration_matrix::categories::{compile_all, CompiledCategory};
use frustration_matrix::firmware::Firmware;
use frustration_matrix::jsonl::{parse_user_lines, JsonlUserLine};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use walkdir::WalkDir;

/// Default minimum char length applied to user messages before any regex
/// match. Matches the existing `scan` subcommand to keep behaviour consistent.
pub const MIN_HIT_LEN: usize = 8;

/// One queued hit awaiting morning review.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueuedHit {
    /// Stable identifier — `<file_basename>:<line_no>:<ts>`.
    pub id: String,
    /// User identifier under which this hit was produced.
    pub user: String,
    /// Predicted category id (matches `categories.rs::Category.id`).
    pub category: String,
    /// Original message text.
    pub message: String,
    /// Source file (absolute path).
    pub source: String,
    /// 1-based line number inside the source file.
    pub line_no: usize,
    /// Unix-seconds timestamp when this hit was queued.
    pub ts: u64,
}

/// Aggregate scan result returned to the CLI / sleep-layer Phase 0.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScanReport {
    /// Total trace files visited (after `since_ts` filter).
    pub scanned: usize,
    /// Total hits emitted across all files.
    pub hits: usize,
    /// Per-category hit counts.
    pub by_category: HashMap<String, usize>,
}

/// Walk `traces_dir`, classify each user line in files modified strictly
/// after `since_ts`, and append every hit to `queue_path` as JSONL.
///
/// Returns the aggregate `ScanReport` for the run.
pub fn nightly_scan(
    traces_dir: &Path,
    firmware: &Firmware,
    user: &str,
    since_ts: u64,
    queue_path: &Path,
) -> Result<ScanReport> {
    let _ = firmware;
    let cats = compile_all();
    let files = collect_recent_traces(traces_dir, since_ts);
    let mut report = ScanReport {
        scanned: 0,
        hits: 0,
        by_category: HashMap::new(),
    };
    for path in files {
        report.scanned += 1;
        scan_one_file(&path, user, &cats, queue_path, &mut report)?;
    }
    Ok(report)
}

/// Walk `traces_dir` and return every `.jsonl` file with mtime strictly
/// greater than `since_ts`. Sorted alphabetically (deterministic).
fn collect_recent_traces(root: &Path, since_ts: u64) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = WalkDir::new(root)
        .follow_links(false)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|r| r.ok())
        .filter(|e| e.path().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|s| s.to_str())
                .is_some_and(|s| s.eq_ignore_ascii_case("jsonl"))
        })
        .filter(|e| file_mtime_secs(e.path()) > since_ts)
        .map(|e| e.into_path())
        .collect();
    out.sort();
    out
}

/// Parse one trace, run every user line through the regex categories,
/// append every hit to the queue, mutate the running report.
fn scan_one_file(
    path: &Path,
    user: &str,
    cats: &[CompiledCategory],
    queue: &Path,
    report: &mut ScanReport,
) -> Result<()> {
    let lines = parse_user_lines(path)?;
    for line in lines {
        let cat_id = match_category(&line.text, cats);
        let Some(cat) = cat_id else { continue };
        let hit = build_hit(&line, user, cat);
        append_queue(queue, &hit)?;
        report.hits += 1;
        *report.by_category.entry(cat.to_string()).or_insert(0) += 1;
    }
    Ok(())
}

/// Run regex categories against `text`. Returns the first category id whose
/// pattern set matches, or `None`. Order = `categories::CATEGORIES` order.
fn match_category<'a>(text: &str, cats: &'a [CompiledCategory]) -> Option<&'a str> {
    if text.chars().count() < MIN_HIT_LEN {
        return None;
    }
    for c in cats {
        if c.patterns.iter().any(|p| p.is_match(text)) {
            return Some(c.id);
        }
    }
    None
}

/// Construct a `QueuedHit` from one parsed user line + classifier verdict.
fn build_hit(line: &JsonlUserLine, user: &str, category: &str) -> QueuedHit {
    let ts = now_secs();
    let basename = line
        .file
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");
    QueuedHit {
        id: format!("{basename}:{line_no}:{ts}", line_no = line.line_no),
        user: user.to_string(),
        category: category.to_string(),
        message: line.text.clone(),
        source: line.file.display().to_string(),
        line_no: line.line_no,
        ts,
    }
}

/// Append one hit as JSONL on `queue` with O_APPEND.
fn append_queue(queue: &Path, hit: &QueuedHit) -> Result<()> {
    if let Some(parent) = queue.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .with_context(|| format!("mkdir {}", parent.display()))?;
        }
    }
    let mut line = serde_json::to_string(hit).context("serialise queued hit")?;
    line.push('\n');
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(queue)
        .with_context(|| format!("open append {}", queue.display()))?;
    f.write_all(line.as_bytes())
        .with_context(|| format!("append {}", queue.display()))?;
    Ok(())
}

/// File mtime in Unix seconds. 0 on any FS error (caller treats 0 as
/// "always include" — unsafe but better than silently dropping a file).
fn file_mtime_secs(path: &Path) -> u64 {
    let Ok(meta) = fs::metadata(path) else { return 0 };
    let Ok(mtime) = meta.modified() else { return 0 };
    mtime
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Wall-clock now in Unix seconds. 0 if the system clock is broken.
fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
