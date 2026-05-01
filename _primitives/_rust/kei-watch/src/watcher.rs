//! Public [`Watcher`] type — owns the notify backend and the pump thread.
//!
//! Layout invariants:
//! - exactly one pump thread per watcher
//! - pump thread exits when `notify::Watcher` is dropped (closes
//!   notify's sender, which closes the pump's `recv`)
//! - `Drop` explicitly drops the notify watcher first, then joins the
//!   pump — preventing zombie threads

use crate::error::{from_notify, WatchError};
use crate::event::Event;
use crate::pump;
use notify::{RecommendedWatcher, RecursiveMode, Watcher as NotifyWatcher};
use std::path::Path;
use std::sync::mpsc::{self, Receiver};
use std::thread::JoinHandle;
use std::time::Duration;

/// Filesystem watcher. Synchronous API only; see crate docs.
pub struct Watcher {
    inner: Option<RecommendedWatcher>,
    out_rx: Receiver<Event>,
    pump_handle: Option<JoinHandle<()>>,
}

impl Watcher {
    /// Construct a new watcher. Spawns one background thread for the
    /// event pump and initialises the notify backend with its own
    /// internal thread(s).
    pub fn new() -> Result<Self, WatchError> {
        let (n_tx, n_rx) = mpsc::channel::<notify::Result<notify::Event>>();
        let (o_tx, o_rx) = mpsc::channel::<Event>();
        let inner = notify::recommended_watcher(n_tx).map_err(from_notify)?;
        let pump_handle = pump::spawn(n_rx, o_tx);
        Ok(Self {
            inner: Some(inner),
            out_rx: o_rx,
            pump_handle: Some(pump_handle),
        })
    }

    /// Begin watching `path`. When `recursive=true`, all descendants
    /// are watched too; otherwise only the path itself (and its
    /// immediate children if a directory).
    pub fn watch(&mut self, path: &Path, recursive: bool) -> Result<(), WatchError> {
        let mode = if recursive {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };
        let inner = self.inner.as_mut().expect("watcher initialised");
        inner.watch(path, mode).map_err(from_notify)
    }

    /// Stop watching `path`.
    pub fn unwatch(&mut self, path: &Path) -> Result<(), WatchError> {
        let inner = self.inner.as_mut().expect("watcher initialised");
        inner.unwatch(path).map_err(from_notify)
    }

    /// Block until either an event arrives or `timeout` elapses.
    /// Returns `None` on timeout or channel closure.
    pub fn next_event(&self, timeout: Duration) -> Option<Event> {
        self.out_rx.recv_timeout(timeout).ok()
    }

    /// Non-blocking: drain all currently queued events.
    pub fn drain(&self) -> Vec<Event> {
        let mut out = Vec::new();
        while let Ok(ev) = self.out_rx.try_recv() {
            out.push(ev);
        }
        out
    }
}

impl Drop for Watcher {
    fn drop(&mut self) {
        // Dropping the notify watcher closes the pump's input channel;
        // the pump loop exits, and we can join its thread cleanly.
        drop(self.inner.take());
        if let Some(h) = self.pump_handle.take() {
            let _ = h.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn new_and_drop_is_clean() {
        let w = Watcher::new().unwrap();
        drop(w);
    }

    #[test]
    fn watch_missing_path_is_error() {
        let mut w = Watcher::new().unwrap();
        let r = w.watch(Path::new("/definitely/does/not/exist/kei-watch"), false);
        assert!(r.is_err());
    }

    #[test]
    fn drain_on_idle_is_empty() {
        let d = tempdir().unwrap();
        let mut w = Watcher::new().unwrap();
        w.watch(d.path(), false).unwrap();
        // No activity → drain returns empty.
        assert!(w.drain().is_empty());
    }
}
