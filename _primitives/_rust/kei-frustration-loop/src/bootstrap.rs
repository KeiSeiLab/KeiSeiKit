//! Install-time bootstrap — first scan of existing chatlogs + first
//! per-user firmware bake.
//!
//! Idempotent: if `<user>.firmware.gz` already exists, we return without
//! re-training. The caller is expected to call this exactly once at
//! install time, then only rely on `auto_train` thereafter.
//!
//! Constructor Pattern: this cube wires existing primitives only —
//! `frustration_matrix::firmware_corpus` for corpus loading,
//! `frustration_matrix::firmware` for training, `persistence` for atomic
//! write, `nightly` for the initial scan.

use crate::nightly::{nightly_scan, ScanReport};
use crate::persistence::{
    atomic_write, ensure_dir, last_scan_ts_path, queue_path, user_firmware_path,
    write_last_scan_ts,
};
use anyhow::{Context, Result};
use frustration_matrix::firmware::{Firmware, DEFAULT_MAX_DEPTH};
use frustration_matrix::firmware_corpus::load_corpus_text;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

/// Outcome of a bootstrap call.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BootstrapReport {
    /// User identifier the firmware was baked for.
    pub user: String,
    /// Absolute path of the per-user firmware on disk after bootstrap.
    pub firmware_path: String,
    /// Number of trace files visited during the initial scan.
    pub scanned_traces: usize,
    /// Number of category hits the initial scan emitted into the queue.
    pub initial_hits: usize,
    /// True iff a per-user firmware already existed (no retraining done).
    pub skipped: bool,
}

/// Run the install-time bootstrap for `user` against `traces_dir`.
///
/// Steps:
///   1. Ensure `<home>/.claude/frustration/` exists (0700).
///   2. If `<user>.firmware.gz` already present → return early (idempotent).
///   3. Load corpus from `traces_dir`, train depth-`DEFAULT_MAX_DEPTH`
///      firmware, write to per-user path atomically.
///   4. Run an initial nightly scan with `since_ts = 0` so every trace
///      seeds the queue.
///   5. Stamp `<user>.last-scan.ts` with `now()`.
pub fn bootstrap(
    user: &str,
    traces_dir: &Path,
    home: &Path,
) -> Result<BootstrapReport> {
    ensure_dir(home).context("ensure frustration dir")?;
    let fw_path = user_firmware_path(home, user);
    if fw_path.exists() {
        return Ok(BootstrapReport {
            user: user.to_string(),
            firmware_path: fw_path.display().to_string(),
            scanned_traces: 0,
            initial_hits: 0,
            skipped: true,
        });
    }
    let firmware = train_initial_firmware(traces_dir, &fw_path)?;
    let report = run_initial_scan(traces_dir, &firmware, user, home)?;
    write_last_scan_ts(&last_scan_ts_path(home, user), now_secs())?;
    Ok(BootstrapReport {
        user: user.to_string(),
        firmware_path: fw_path.display().to_string(),
        scanned_traces: report.scanned,
        initial_hits: report.hits,
        skipped: false,
    })
}

/// Load corpus from `traces_dir`, train firmware, atomic-write to `dest`.
///
/// Empty corpora produce a near-trivial firmware (only a `\n` unigram from
/// the empty-text floor). We accept that — the user simply has no chatlogs
/// yet, and bootstrap should not error on a fresh machine.
fn train_initial_firmware(traces_dir: &Path, dest: &Path) -> Result<Firmware> {
    let text = load_corpus_text(traces_dir)
        .with_context(|| format!("load corpus from {}", traces_dir.display()))?;
    let firmware = Firmware::train_from_text(&text, DEFAULT_MAX_DEPTH);
    save_firmware_atomic(&firmware, dest)?;
    Ok(firmware)
}

/// Save the firmware to a temp sibling then `rename` over `dest`.
///
/// `Firmware::save` already writes gz-JSON, but it does so directly to its
/// destination — for atomicity we redirect through a `.tmp` sibling and
/// rename. The temp file is removed on early-return errors via the OS
/// (we accept the leak risk if the process is killed mid-rename).
fn save_firmware_atomic(firmware: &Firmware, dest: &Path) -> Result<()> {
    let tmp = tmp_sibling(dest);
    firmware
        .save(&tmp)
        .with_context(|| format!("save firmware tmp {}", tmp.display()))?;
    let bytes = std::fs::read(&tmp)
        .with_context(|| format!("read tmp {}", tmp.display()))?;
    let _ = std::fs::remove_file(&tmp);
    atomic_write(dest, &bytes)?;
    Ok(())
}

/// Initial scan over the WHOLE traces dir (since_ts = 0).
fn run_initial_scan(
    traces_dir: &Path,
    firmware: &Firmware,
    user: &str,
    home: &Path,
) -> Result<ScanReport> {
    let q = queue_path(home);
    nightly_scan(traces_dir, firmware, user, 0, &q)
}

/// Build the `<dest>.tmp` sibling.
fn tmp_sibling(dest: &Path) -> std::path::PathBuf {
    let mut s = dest.as_os_str().to_owned();
    s.push(".bootstrap-tmp");
    std::path::PathBuf::from(s)
}

/// Wall-clock now in Unix seconds. 0 if the system clock is broken.
fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
