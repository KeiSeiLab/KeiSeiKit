//! `HotReloader` — kei-watch → kei-pipe bridge.
//!
//! Thin wrapper that owns a [`kei_watch::Watcher`] plus a user callback
//! fired on every accepted filesystem event. Typical usage: a driver
//! loop watches a DAG TOML + its source tree; every
//! `Created`/`Modified`/`Deleted` event triggers a re-parse and re-run.
//!
//! The wrapper is synchronous (no async runtime). `poll_once` blocks up
//! to `timeout` for the first event, then drains anything else already
//! queued and returns the full batch. Zero events → empty vec.
//!
//! Trust boundary: callback runs on the caller's thread inside
//! `poll_once`, NOT on the internal pump thread.

use kei_watch::{Event, WatchError, Watcher};
use std::path::Path;
use std::time::Duration;

/// Public error surface: mirrors [`kei_watch::WatchError`] plus a local
/// `EmptyPaths` variant so we fail fast when the caller forgets to list
/// a watch target.
#[derive(Debug)]
pub enum Error {
    EmptyPaths,
    Watch(WatchError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EmptyPaths => write!(f, "hot_reload: paths slice is empty"),
            Error::Watch(e) => write!(f, "hot_reload: watch: {e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Watch(e) => Some(e),
            _ => None,
        }
    }
}

impl From<WatchError> for Error {
    fn from(e: WatchError) -> Self {
        Error::Watch(e)
    }
}

/// Owns the [`Watcher`] and the per-event callback.
///
/// Stays parked in the caller's struct for the lifetime of the
/// reload loop; dropping it drops the inner `Watcher` which joins its
/// pump thread cleanly (see `kei_watch::Watcher::drop`).
pub struct HotReloader {
    watcher: Watcher,
    cb: Box<dyn Fn(&Event) + Send>,
}

impl HotReloader {
    /// Build a reloader that watches each entry in `paths` recursively.
    /// `cb` fires once per accepted event during each `poll_once` call.
    pub fn new<F>(paths: &[&Path], cb: F) -> Result<Self, Error>
    where
        F: Fn(&Event) + Send + 'static,
    {
        if paths.is_empty() {
            return Err(Error::EmptyPaths);
        }
        let mut watcher = Watcher::new()?;
        for p in paths {
            watcher.watch(p, true)?;
        }
        Ok(Self {
            watcher,
            cb: Box::new(cb),
        })
    }

    /// Block up to `timeout` for the first event, drain anything else
    /// already queued, fire `cb` on each, return the full batch.
    pub fn poll_once(&self, timeout: Duration) -> Vec<Event> {
        let mut out = Vec::new();
        if let Some(first) = self.watcher.next_event(timeout) {
            (self.cb)(&first);
            out.push(first);
        }
        for ev in self.watcher.drain() {
            (self.cb)(&ev);
            out.push(ev);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use tempfile::tempdir;

    #[test]
    fn new_rejects_empty_paths() {
        let res = HotReloader::new(&[], |_| {});
        match res {
            Err(Error::EmptyPaths) => {}
            Err(other) => panic!("expected EmptyPaths, got {other:?}"),
            Ok(_) => panic!("expected Err, got Ok"),
        }
    }

    #[test]
    fn poll_empty_returns_empty() {
        let d = tempdir().unwrap();
        let r = HotReloader::new(&[d.path()], |_| {}).expect("reloader");
        let evs = r.poll_once(Duration::from_millis(50));
        assert!(evs.is_empty(), "idle poll → empty, got {evs:?}");
    }

    #[test]
    fn poll_detects_modify() {
        let d = tempdir().unwrap();
        let file = d.path().join("dag.toml");
        fs::write(&file, "initial").unwrap();
        let fired = Arc::new(AtomicUsize::new(0));
        let fired_cb = fired.clone();
        let r = HotReloader::new(&[d.path()], move |_ev| {
            fired_cb.fetch_add(1, Ordering::SeqCst);
        })
        .expect("reloader");
        // Give watcher a moment to register the initial state, then
        // mutate.
        std::thread::sleep(Duration::from_millis(100));
        fs::write(&file, "changed").unwrap();
        let evs = r.poll_once(Duration::from_millis(2000));
        assert!(!evs.is_empty(), "expected ≥1 event, got none");
        assert!(fired.load(Ordering::SeqCst) >= 1, "cb must fire");
    }
}
