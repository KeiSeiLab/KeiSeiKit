//! TODO/FIXME age scanner — see CLEANUP-RUNTIME-SPEC.md §3.2 + Appendix A.
//!
//! Walks workspace src/, finds `(TODO|FIXME|XXX|HACK)` markers, then
//! shells out to `git blame -L N,N <file>` to get the commit timestamp
//! of each match. Severity grades from age:
//!   * age > cfg.todo_age.fail_days → HIGH
//!   * age > cfg.todo_age.warn_days → MEDIUM
//!   * else → LOW
//!
//! If `git` is not on PATH, scanner returns `ToolNotFound` and the
//! runtime records it in `scanners_skipped` per CLEANUP §3.3 contract.

use crate::config::Config;
use crate::error::CleanupError;
use crate::report::{Confidence, Finding, FindingKind, Location, Severity};
use chrono::{DateTime, Utc};
use regex::Regex;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

/// Public scanner entry — see Appendix A row "todo_age".
pub fn scan(workspace: &Path, cfg: &Config) -> Result<Vec<Finding>, CleanupError> {
    if !has_git() {
        return Err(CleanupError::ToolNotFound("git not on PATH".into()));
    }
    let re = Regex::new(r"(?i)\b(TODO|FIXME|XXX|HACK)\b")
        .map_err(|e| CleanupError::Walk(format!("regex: {e}")))?;
    let mut out = Vec::new();
    for entry in WalkDir::new(workspace).into_iter().filter_map(|e| e.ok()) {
        if !is_rust_source(entry.path(), cfg) {
            continue;
        }
        scan_file(entry.path(), workspace, cfg, &re, &mut out)?;
    }
    Ok(out)
}

fn is_rust_source(path: &Path, cfg: &Config) -> bool {
    if !path.is_file() {
        return false;
    }
    if path.extension().and_then(|e| e.to_str()) != Some("rs") {
        return false;
    }
    !cfg.excluded(path)
}

fn scan_file(
    path: &Path,
    workspace: &Path,
    cfg: &Config,
    re: &Regex,
    out: &mut Vec<Finding>,
) -> Result<(), CleanupError> {
    let text = std::fs::read_to_string(path)?;
    for (i, line) in text.lines().enumerate() {
        if !re.is_match(line) {
            continue;
        }
        let lineno = (i + 1) as u32;
        let Some(commit_ts) = git_blame_ts(path, lineno) else {
            continue;
        };
        let age_days = age_days_from(commit_ts);
        out.push(make_finding(path, workspace, lineno, age_days, cfg, commit_ts));
    }
    Ok(())
}

fn make_finding(
    path: &Path,
    workspace: &Path,
    line: u32,
    age_days: i64,
    cfg: &Config,
    ts: DateTime<Utc>,
) -> Finding {
    let rel = path
        .strip_prefix(workspace)
        .map(Path::to_path_buf)
        .unwrap_or_else(|_| path.to_path_buf());
    let severity = severity_for(age_days, cfg);
    let date = ts.format("%Y-%m-%d").to_string();
    Finding {
        kind: FindingKind::StaleTodo,
        severity,
        location: Location { file: rel, line: Some(line) },
        message: format!("TODO from {date} ({age_days}d)"),
        fix_hint: Some("triage: resolve, file an issue, or drop the marker".into()),
        rule_ref: Some("CLEANUP §3.2 todo_age".into()),
        confidence: Confidence::High,
    }
}

fn severity_for(age_days: i64, cfg: &Config) -> Severity {
    if age_days < 0 {
        return Severity::Low;
    }
    let age = age_days as u64;
    if age > cfg.todo_age.fail_days {
        Severity::High
    } else if age > cfg.todo_age.warn_days {
        Severity::Medium
    } else {
        Severity::Low
    }
}

fn age_days_from(commit_ts: DateTime<Utc>) -> i64 {
    let now = Utc::now();
    (now - commit_ts).num_days()
}

/// `git blame -L N,N --porcelain <file>` → first `author-time <unix>` line.
fn git_blame_ts(path: &Path, line: u32) -> Option<DateTime<Utc>> {
    let dir = path.parent()?;
    let file = path.file_name()?;
    let range = format!("{line},{line}");
    let out = Command::new("git")
        .args(["blame", "-L", &range, "--porcelain"])
        .arg(file)
        .current_dir(dir)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    parse_author_time(&stdout)
}

fn parse_author_time(porcelain: &str) -> Option<DateTime<Utc>> {
    for line in porcelain.lines() {
        if let Some(rest) = line.strip_prefix("author-time ") {
            if let Ok(secs) = rest.trim().parse::<i64>() {
                return DateTime::<Utc>::from_timestamp(secs, 0);
            }
        }
    }
    None
}

fn has_git() -> bool {
    Command::new("git")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
