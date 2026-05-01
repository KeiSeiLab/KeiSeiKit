//! Per-user firmware persistence — paths + atomic file swap.
//!
//! Layout under `<home>/.claude/frustration/`:
//!   * `<user>.firmware.gz`   — per-user trained byte n-gram (gz JSON, same
//!                              format as `firmware::Firmware::save`)
//!   * `<user>.last-scan.ts`  — Unix timestamp (seconds) of last nightly scan
//!   * `<user>.feedback.jsonl` — one JSON record per user correction
//!   * `queue.jsonl`          — shared queue of new hits awaiting review
//!
//! Constructor Pattern: this cube only resolves paths and shovels bytes.
//! Format decisions live in `feedback.rs` / `firmware.rs`.

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Directory name under `~/.claude/` where the loop persists per-user state.
pub const FRUSTRATION_DIR: &str = ".claude/frustration";

/// Build the per-user firmware path: `<home>/.claude/frustration/<user>.firmware.gz`.
pub fn user_firmware_path(home: &Path, user: &str) -> PathBuf {
    frustration_dir(home).join(format!("{user}.firmware.gz"))
}

/// Path to the per-user last-scan timestamp marker (Unix seconds, plain text).
pub fn last_scan_ts_path(home: &Path, user: &str) -> PathBuf {
    frustration_dir(home).join(format!("{user}.last-scan.ts"))
}

/// Path to the per-user feedback log (jsonl, append-only).
pub fn feedback_path(home: &Path, user: &str) -> PathBuf {
    frustration_dir(home).join(format!("{user}.feedback.jsonl"))
}

/// Shared queue of new hits awaiting user review (across users).
pub fn queue_path(home: &Path) -> PathBuf {
    frustration_dir(home).join("queue.jsonl")
}

/// Resolve `<home>/.claude/frustration` (no side effects).
pub fn frustration_dir(home: &Path) -> PathBuf {
    home.join(FRUSTRATION_DIR)
}

/// Create `<home>/.claude/frustration` with 0700 permissions if missing.
pub fn ensure_dir(home: &Path) -> Result<PathBuf> {
    let dir = frustration_dir(home);
    if !dir.exists() {
        fs::create_dir_all(&dir)
            .with_context(|| format!("mkdir {}", dir.display()))?;
        set_dir_mode_700(&dir)?;
    }
    Ok(dir)
}

/// Atomic write: pipe `bytes` into `<dest>.tmp`, fsync, rename → `dest`.
///
/// Crash-safe: if rename completes, `dest` holds either the new bytes or
/// the old bytes — never a partial file. The rename is atomic on POSIX
/// when source and destination are on the same filesystem (we always put
/// `.tmp` next to `dest`, so this holds).
pub fn atomic_write(dest: &Path, bytes: &[u8]) -> Result<()> {
    if let Some(parent) = dest.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .with_context(|| format!("mkdir {}", parent.display()))?;
        }
    }
    let tmp = tmp_sibling(dest);
    fs::write(&tmp, bytes)
        .with_context(|| format!("write tmp {}", tmp.display()))?;
    atomic_swap(&tmp, dest)?;
    Ok(())
}

/// Rename `tmp` over `dest`. Both must live on the same filesystem.
pub fn atomic_swap(tmp: &Path, dest: &Path) -> Result<()> {
    fs::rename(tmp, dest).with_context(|| {
        format!("rename {} → {}", tmp.display(), dest.display())
    })?;
    Ok(())
}

/// Read a file or return `fallback` if it does not exist. Other IO
/// errors propagate (corrupt file should not silently disappear).
pub fn load_or_default(path: &Path, fallback: Vec<u8>) -> Result<Vec<u8>> {
    match fs::read(path) {
        Ok(bytes) => Ok(bytes),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(fallback),
        Err(e) => Err(e).with_context(|| format!("read {}", path.display())),
    }
}

/// Read a Unix-seconds timestamp from `<user>.last-scan.ts`. Missing file
/// or unparseable contents → 0 (i.e. scan everything).
pub fn read_last_scan_ts(path: &Path) -> u64 {
    let Ok(text) = fs::read_to_string(path) else {
        return 0;
    };
    text.trim().parse::<u64>().unwrap_or(0)
}

/// Persist `ts` into `<user>.last-scan.ts` atomically.
pub fn write_last_scan_ts(path: &Path, ts: u64) -> Result<()> {
    atomic_write(path, format!("{ts}\n").as_bytes())
}

/// Build the `<dest>.tmp` sibling path used by `atomic_write`.
fn tmp_sibling(dest: &Path) -> PathBuf {
    let mut s = dest.as_os_str().to_owned();
    s.push(".tmp");
    PathBuf::from(s)
}

/// Apply 0700 permissions on UNIX targets. No-op on Windows (the kit is
/// macOS / Linux only; this branch keeps the code cross-compilable).
#[cfg(unix)]
fn set_dir_mode_700(dir: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(dir)
        .with_context(|| format!("stat {}", dir.display()))?
        .permissions();
    perms.set_mode(0o700);
    fs::set_permissions(dir, perms)
        .with_context(|| format!("chmod 0700 {}", dir.display()))?;
    Ok(())
}

#[cfg(not(unix))]
fn set_dir_mode_700(_dir: &Path) -> Result<()> {
    Ok(())
}
