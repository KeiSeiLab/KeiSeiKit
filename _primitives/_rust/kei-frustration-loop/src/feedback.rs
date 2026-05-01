//! User-feedback log — Feedback struct + JSONL append/count/read.
//!
//! One line in `<user>.feedback.jsonl` records one correction the user made
//! while reviewing a queued nightly hit. The retrain trigger walks this log
//! to decide whether the per-user firmware should be rebaked.
//!
//! Constructor Pattern: this cube only owns the on-disk shape of feedback.
//! Threshold logic lives in `auto_train.rs`; queue-emission lives in
//! `nightly.rs`; firmware retraining lives in `firmware.rs`.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

/// One correction the user made about one queued nightly hit.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Feedback {
    /// The hit identifier the user reviewed (matches `Hit.id` in the queue).
    pub hit_id: String,
    /// The original message text (denormalised so log is self-contained).
    pub message: String,
    /// Verdict: classifier was right / wrong / a new category emerged.
    pub label: Label,
    /// Predicted (or new) category name. Empty for `Wrong` with no
    /// suggestion attached.
    pub category: String,
    /// Unix-seconds timestamp of when the user filed the feedback.
    pub ts: u64,
    /// User identifier (the `--user` slug; defaults to `$USER`).
    pub user: String,
}

/// Verdict the user attached to one queued hit.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Label {
    /// Classifier categorised this hit correctly.
    Correct,
    /// Classifier mis-categorised — discard this hit from corpus.
    Wrong,
    /// User wants a brand-new category to be tracked. Inner string is the
    /// suggested category slug.
    NewCategory(String),
}

impl Label {
    /// Parse the CLI form: `correct`, `wrong`, `new:<slug>`.
    pub fn parse(raw: &str) -> Result<Self> {
        let s = raw.trim();
        match s {
            "correct" => Ok(Label::Correct),
            "wrong" => Ok(Label::Wrong),
            other => parse_new_category(other),
        }
    }
}

/// Append one feedback row to `path` as a single JSONL line.
///
/// Atomic for the per-row sense: we open `O_APPEND`, write the full line
/// (including trailing `\n`) in one syscall. Concurrent writers get
/// interleaved-but-line-intact output on POSIX.
pub fn append_feedback(path: &Path, fb: &Feedback) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("mkdir {}", parent.display()))?;
        }
    }
    let mut line = serde_json::to_string(fb).context("serialise feedback")?;
    line.push('\n');
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("open append {}", path.display()))?;
    file.write_all(line.as_bytes())
        .with_context(|| format!("append {}", path.display()))?;
    Ok(())
}

/// Count rows in `path`. Missing file → 0. Malformed lines are skipped
/// (one bad row must not abort the count).
pub fn count_pending(path: &Path) -> Result<usize> {
    let lines = read_raw_lines(path)?;
    let mut n = 0usize;
    for raw in lines {
        if serde_json::from_str::<Feedback>(&raw).is_ok() {
            n += 1;
        }
    }
    Ok(n)
}

/// Read every well-formed feedback row from `path`. Missing file → empty.
pub fn read_all(path: &Path) -> Result<Vec<Feedback>> {
    let lines = read_raw_lines(path)?;
    let mut out = Vec::new();
    for raw in lines {
        let Ok(fb) = serde_json::from_str::<Feedback>(&raw) else {
            continue;
        };
        out.push(fb);
    }
    Ok(out)
}

/// Stream JSONL lines from `path`, dropping blanks. Missing file → empty
/// vector (callers treat "no feedback yet" as a valid state).
fn read_raw_lines(path: &Path) -> Result<Vec<String>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let file = File::open(path)
        .with_context(|| format!("open {}", path.display()))?;
    let reader = BufReader::new(file);
    let mut out = Vec::new();
    for line in reader.lines() {
        let raw = line.with_context(|| format!("read {}", path.display()))?;
        if !raw.trim().is_empty() {
            out.push(raw);
        }
    }
    Ok(out)
}

/// Helper for `Label::parse`: handle the `new:<slug>` form and reject
/// anything else with a useful error message.
fn parse_new_category(raw: &str) -> Result<Label> {
    let Some(slug) = raw.strip_prefix("new:") else {
        return Err(anyhow::anyhow!(
            "invalid label {raw:?}: expected correct|wrong|new:<slug>"
        ));
    };
    let trimmed = slug.trim();
    if trimmed.is_empty() {
        return Err(anyhow::anyhow!(
            "invalid label {raw:?}: new:<slug> requires a non-empty slug"
        ));
    }
    Ok(Label::NewCategory(trimmed.to_string()))
}
