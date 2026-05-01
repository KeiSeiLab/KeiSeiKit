//! Shared helpers for the integration tests.
//!
//! `tests/` files are separate crates; common code lives under
//! `tests/common/mod.rs` per cargo convention (not a top-level
//! `tests/common.rs`, which would itself be a test binary).

use kei_watch::Watcher;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

pub const EVENT_TIMEOUT: Duration = Duration::from_secs(3);

/// Pull events until one matches `pred` or the global timeout elapses.
pub fn wait_for<F: Fn(&kei_watch::Event) -> bool>(
    w: &Watcher,
    pred: F,
) -> Option<kei_watch::Event> {
    let deadline = Instant::now() + EVENT_TIMEOUT;
    while Instant::now() < deadline {
        let remaining = deadline.saturating_duration_since(Instant::now());
        let step = std::cmp::min(remaining, Duration::from_millis(200));
        if let Some(ev) = w.next_event(step) {
            if pred(&ev) {
                return Some(ev);
            }
        }
    }
    None
}

/// macOS reports paths under `/private/var/...`; tempdirs live at `/var/...`.
/// Canonicalise both sides before compare. When the target file has been
/// deleted, `canonicalize` fails — fall back to symlink-free parent match.
pub fn same_path(a: &Path, b: &Path) -> bool {
    if a == b {
        return true;
    }
    let ca = fs::canonicalize(a).ok();
    let cb = fs::canonicalize(b).ok();
    match (ca, cb) {
        (Some(x), Some(y)) => x == y,
        _ => canonicalize_parent(a) == canonicalize_parent(b),
    }
}

fn canonicalize_parent(p: &Path) -> PathBuf {
    let parent = p.parent().and_then(|q| fs::canonicalize(q).ok());
    let file = p.file_name().map(|s| s.to_owned());
    match (parent, file) {
        (Some(par), Some(f)) => par.join(f),
        _ => p.to_path_buf(),
    }
}
