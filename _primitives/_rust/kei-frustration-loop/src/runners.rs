//! Loop CLI runners — `bootstrap` / `nightly-scan` / `feedback` /
//! `auto-train` / `personalize`.
//!
//! Constructor Pattern: each runner is a thin shim that adapts the parsed
//! args to a domain cube and prints a JSON record. No business logic
//! lives here; the cubes own corpus prep, classification, threshold check.

use anyhow::{Context, Result};
use frustration_matrix::firmware::Firmware;
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::auto_train;
use crate::bootstrap;
use crate::feedback::{append_feedback, count_pending, Feedback, Label};
use crate::nightly;
use crate::persistence::{
    feedback_path, last_scan_ts_path, queue_path, read_last_scan_ts, user_firmware_path,
    write_last_scan_ts,
};

/// Run the install-time bootstrap and print a JSON `BootstrapReport`.
pub fn run_bootstrap(user: &str, home: Option<&Path>) -> Result<()> {
    let home = resolve_home(home)?;
    let traces = home.join(".claude/memory/traces");
    let report = bootstrap::bootstrap(user, &traces, &home)?;
    print_json(&report)
}

/// Run the Phase-0 nightly scan and print a JSON `ScanReport`.
pub fn run_nightly(user: &str, since: Option<u64>, home: Option<&Path>) -> Result<()> {
    let home = resolve_home(home)?;
    let fw_path = user_firmware_path(&home, user);
    let firmware = Firmware::load(&fw_path).with_context(|| {
        format!("load firmware {}; run `bootstrap` first", fw_path.display())
    })?;
    let since_ts = since.unwrap_or_else(|| read_last_scan_ts(&last_scan_ts_path(&home, user)));
    let traces = home.join(".claude/memory/traces");
    let q = queue_path(&home);
    let report = nightly::nightly_scan(&traces, &firmware, user, since_ts, &q)?;
    write_last_scan_ts(&last_scan_ts_path(&home, user), now_secs())?;
    print_json(&report)
}

/// Append one feedback row and surface the retrain recommendation.
pub fn run_feedback(
    hit_id: &str,
    label: &str,
    user: &str,
    home: Option<&Path>,
    message: &str,
    category: &str,
) -> Result<()> {
    let home = resolve_home(home)?;
    let path = feedback_path(&home, user);
    let fb = build_feedback(hit_id, label, user, message, category)?;
    append_feedback(&path, &fb)?;
    let count = count_pending(&path)?;
    let threshold = auto_train::resolve_threshold(None);
    let retrain_recommended = count >= threshold;
    print_json(&serde_json::json!({
        "appended": true, "count": count,
        "retrain_recommended": retrain_recommended, "threshold": threshold,
    }))
}

/// Build the `Feedback` struct from CLI fragments. Splits parse + struct
/// construction off so `run_feedback` stays under the LOC budget.
fn build_feedback(
    hit_id: &str,
    label: &str,
    user: &str,
    message: &str,
    category: &str,
) -> Result<Feedback> {
    let label = Label::parse(label)?;
    Ok(Feedback {
        hit_id: hit_id.to_string(),
        message: message.to_string(),
        label,
        category: category.to_string(),
        ts: now_secs(),
        user: user.to_string(),
    })
}

/// Trigger an `auto_train` run and print the resulting `TrainReport`.
pub fn run_auto_train(
    user: &str,
    threshold: Option<usize>,
    home: Option<&Path>,
    traces_dir: Option<&Path>,
) -> Result<()> {
    let home = resolve_home(home)?;
    let traces = traces_dir
        .map(Path::to_path_buf)
        .unwrap_or_else(|| home.join(".claude/memory/traces"));
    let fb = feedback_path(&home, user);
    let out = user_firmware_path(&home, user);
    let t = auto_train::resolve_threshold(threshold);
    let report = auto_train::auto_train(&traces, &fb, &out, t)?;
    print_json(&report)
}

/// Inspect which firmware will be used for `--user`.
pub fn run_personalize(user: &str, home: Option<&Path>) -> Result<()> {
    let home = resolve_home(home)?;
    let path = user_firmware_path(&home, user);
    let exists = path.exists();
    let depth = if exists {
        Firmware::load(&path)
            .map(|fw| fw.max_depth as i64)
            .unwrap_or(-1)
    } else {
        -1
    };
    print_json(&serde_json::json!({
        "user": user,
        "firmware_path": path.display().to_string(),
        "exists": exists,
        "ngram_depth": depth,
    }))
}

/// Resolve `home` argument: explicit `--home` wins; falls back to `$HOME`.
pub fn resolve_home(home: Option<&Path>) -> Result<PathBuf> {
    if let Some(h) = home {
        return Ok(h.to_path_buf());
    }
    let env = std::env::var("HOME").context("HOME env var not set; pass --home")?;
    Ok(PathBuf::from(env))
}

/// Print one JSON record to stdout (newline-terminated). Used by every loop
/// subcommand so callers can pipe into `jq` / shell wrappers.
fn print_json<T: Serialize>(value: &T) -> Result<()> {
    let s = serde_json::to_string(value)?;
    println!("{s}");
    Ok(())
}

/// Wall-clock now in Unix seconds. 0 if the system clock is broken.
fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
