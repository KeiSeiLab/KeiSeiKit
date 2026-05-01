//! Pure helpers used by the watcher: project-root mapping and a 2-second
//! debounce buffer that collapses many filesystem events for the same
//! project into one notification.
//!
//! No async, no I/O, no `notify` types — easy to unit-test.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// Map any filesystem path to the immediate child of `root` it sits inside.
///
/// Example: `/home/x/Projects/MyApp/src/main.rs` with root
/// `/home/x/Projects` → `Some(/home/x/Projects/MyApp)`.
///
/// Returns `None` when `path` is not strictly under `root` (so events on
/// `root` itself, or on a sibling tree, are ignored).
pub fn project_root_of(path: &Path, root: &Path) -> Option<PathBuf> {
    let rel = path.strip_prefix(root).ok()?;
    let first = rel.components().next()?;
    let name = first.as_os_str();
    if name.is_empty() {
        return None;
    }
    Some(root.join(name))
}

/// Debounce window: collapse repeated events on the same project into one
/// emission per `window` (default 2 s).
///
/// Caller pushes incoming project paths via [`Debouncer::push`]; periodic
/// [`Debouncer::drain_ready`] returns the project paths whose window has
/// elapsed (i.e. no further events arrived in the last `window`).
pub struct Debouncer {
    window: Duration,
    pending: HashMap<PathBuf, Instant>,
}

impl Debouncer {
    /// Create a debouncer with the given quiet window.
    pub fn new(window: Duration) -> Self {
        Self {
            window,
            pending: HashMap::new(),
        }
    }

    /// Record an event for `project` at time `now`. Resets the project's
    /// quiet timer.
    pub fn push(&mut self, project: PathBuf, now: Instant) {
        self.pending.insert(project, now);
    }

    /// Return all projects whose last event is older than `window` at
    /// `now`, removing them from the pending set.
    pub fn drain_ready(&mut self, now: Instant) -> Vec<PathBuf> {
        let window = self.window;
        let ready: Vec<PathBuf> = self
            .pending
            .iter()
            .filter_map(|(p, t)| {
                if now.duration_since(*t) >= window {
                    Some(p.clone())
                } else {
                    None
                }
            })
            .collect();
        for p in &ready {
            self.pending.remove(p);
        }
        ready
    }

    /// Number of projects currently waiting for their quiet window to
    /// elapse.
    pub fn pending_len(&self) -> usize {
        self.pending.len()
    }
}
