//! Auto-retrain trigger logic.
//!
//! When the per-user feedback log crosses `threshold` rows, the orchestrator
//! invokes `auto_train`: rebuild the corpus from confirmed-correct hits
//! (treating `Wrong` and `NewCategory(_)` as exclusions / extension only),
//! retrain the firmware, and atomic-swap it into place.
//!
//! Default threshold = 20 (see `DEFAULT_THRESHOLD`); overridable via the
//! `KEI_FRUSTRATION_THRESHOLD` env var or the `--threshold` CLI flag.
//!
//! Constructor Pattern: this cube only orchestrates `feedback`,
//! `frustration_matrix::firmware`, `frustration_matrix::firmware_corpus`,
//! `persistence`. No format decisions.

use crate::feedback::{count_pending, read_all, Feedback, Label};
use crate::persistence::atomic_write;
use anyhow::{Context, Result};
use frustration_matrix::firmware::{Firmware, DEFAULT_MAX_DEPTH};
use frustration_matrix::firmware_corpus::load_corpus_text;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Default feedback-row count at which `should_retrain` flips to `true`.
pub const DEFAULT_THRESHOLD: usize = 20;

/// Env var name that overrides `DEFAULT_THRESHOLD`.
pub const THRESHOLD_ENV: &str = "KEI_FRUSTRATION_THRESHOLD";

/// Result returned to the CLI for the `auto-train` subcommand.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrainReport {
    /// True iff a retrain happened (false = under threshold or no corpus).
    pub trained: bool,
    /// Total UTF-8 chars used to train the new firmware.
    pub corpus_size: usize,
    /// Path of the per-user firmware after the swap.
    pub output_path: String,
    /// Feedback row count seen at trigger time.
    pub feedback_count: usize,
    /// Threshold the count was compared against.
    pub threshold: usize,
}

/// Resolve effective threshold: explicit `cli_threshold` (if `Some`) wins,
/// then `KEI_FRUSTRATION_THRESHOLD` env var, then `DEFAULT_THRESHOLD`.
pub fn resolve_threshold(cli_threshold: Option<usize>) -> usize {
    if let Some(t) = cli_threshold {
        return t;
    }
    if let Ok(raw) = std::env::var(THRESHOLD_ENV) {
        if let Ok(parsed) = raw.trim().parse::<usize>() {
            return parsed;
        }
    }
    DEFAULT_THRESHOLD
}

/// True iff the feedback log at `path` has at least `threshold` rows.
pub fn should_retrain(path: &Path, threshold: usize) -> Result<bool> {
    Ok(count_pending(path)? >= threshold)
}

/// Rebuild the per-user firmware from the supplied corpus + feedback log.
///
/// Treats `Label::Correct` rows as additional positive corpus material
/// appended to the corpus loaded from `traces_dir`. `Label::Wrong` rows
/// are dropped. `Label::NewCategory(_)` rows are also appended to corpus
/// (the slug is informational; the message text is the training signal).
pub fn auto_train(
    traces_dir: &Path,
    feedback_path: &Path,
    output: &Path,
    threshold: usize,
) -> Result<TrainReport> {
    let count = count_pending(feedback_path)?;
    let mut report = TrainReport {
        trained: false,
        corpus_size: 0,
        output_path: output.display().to_string(),
        feedback_count: count,
        threshold,
    };
    if count < threshold {
        return Ok(report);
    }
    let corpus = build_corpus(traces_dir, feedback_path)?;
    if corpus.is_empty() {
        return Ok(report);
    }
    let firmware = Firmware::train_from_text(&corpus, DEFAULT_MAX_DEPTH);
    save_firmware_atomic(&firmware, output)?;
    report.trained = true;
    report.corpus_size = corpus.chars().count();
    Ok(report)
}

/// Concatenate base-corpus text with every kept feedback message into one
/// big training buffer. Newline-separated to keep n-grams from bleeding.
fn build_corpus(traces_dir: &Path, feedback_path: &Path) -> Result<String> {
    let mut buf = load_corpus_text(traces_dir)
        .with_context(|| format!("load corpus from {}", traces_dir.display()))?;
    let extra = collect_feedback_text(feedback_path)?;
    if !extra.is_empty() {
        if !buf.is_empty() && !buf.ends_with('\n') {
            buf.push('\n');
        }
        buf.push_str(&extra);
    }
    Ok(buf)
}

/// Pull every kept feedback message body into one newline-joined string.
fn collect_feedback_text(path: &Path) -> Result<String> {
    let rows = read_all(path)?;
    let mut buf = String::new();
    for fb in rows {
        if !keep_for_corpus(&fb) {
            continue;
        }
        if !buf.is_empty() {
            buf.push('\n');
        }
        buf.push_str(&fb.message);
    }
    Ok(buf)
}

/// Decide whether one feedback row contributes to the new corpus.
/// `Wrong` rows are excluded (the user said the original classification
/// was bogus); everything else feeds the rebuild.
fn keep_for_corpus(fb: &Feedback) -> bool {
    !matches!(fb.label, Label::Wrong)
}

/// Save firmware via tmp-sibling + atomic_write so a crash mid-write
/// never leaves a half-baked `.gz` in place of the previous one.
fn save_firmware_atomic(firmware: &Firmware, dest: &Path) -> Result<()> {
    let tmp = {
        let mut s = dest.as_os_str().to_owned();
        s.push(".autotrain-tmp");
        std::path::PathBuf::from(s)
    };
    firmware
        .save(&tmp)
        .with_context(|| format!("save tmp {}", tmp.display()))?;
    let bytes = std::fs::read(&tmp)
        .with_context(|| format!("read tmp {}", tmp.display()))?;
    let _ = std::fs::remove_file(&tmp);
    atomic_write(dest, &bytes)?;
    Ok(())
}
