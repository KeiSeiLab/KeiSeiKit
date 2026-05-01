//! Coarse debounce: collapse duplicate `(path, kind)` events fired
//! within [`DEBOUNCE_WINDOW`] of the previous one.
//!
//! Intent: swallow FS-level bursts (editor-write double-fire, compiler
//! rewrite patterns). NOT a replacement for notify-debouncer-full — we
//! don't do event reordering or close/write correlation, just per-key
//! rate-limiting.

use crate::event::{Event, EventKind};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// Collapse window for duplicate `(path, kind)` pairs.
pub const DEBOUNCE_WINDOW: Duration = Duration::from_millis(50);

/// Per-key last-seen state. Small enough to live in a `HashMap` — pruned
/// opportunistically when entries exceed [`PRUNE_THRESHOLD`] (keeps
/// long-running watchers from growing unboundedly).
pub struct Debouncer {
    last_seen: HashMap<(PathBuf, EventKind), Instant>,
}

const PRUNE_THRESHOLD: usize = 1024;

impl Debouncer {
    pub fn new() -> Self {
        Self { last_seen: HashMap::new() }
    }

    /// Should this event pass through?
    ///
    /// Returns `true` on first occurrence of `(path, kind)` or if the
    /// last occurrence was ≥ `DEBOUNCE_WINDOW` ago. Updates internal
    /// state regardless of outcome.
    pub fn accept(&mut self, ev: &Event) -> bool {
        let key = (ev.path.clone(), ev.kind);
        let now = Instant::now();
        let decision = match self.last_seen.get(&key) {
            Some(&prev) if now.duration_since(prev) < DEBOUNCE_WINDOW => false,
            _ => true,
        };
        self.last_seen.insert(key, now);
        if self.last_seen.len() > PRUNE_THRESHOLD {
            self.prune(now);
        }
        decision
    }

    /// Drop entries older than 10× the debounce window. Called
    /// opportunistically when the map grows large.
    fn prune(&mut self, now: Instant) {
        let cutoff = DEBOUNCE_WINDOW * 10;
        self.last_seen.retain(|_, &mut t| now.duration_since(t) < cutoff);
    }
}

impl Default for Debouncer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    fn ev(kind: EventKind, path: &str) -> Event {
        Event::new(kind, PathBuf::from(path), None)
    }

    #[test]
    fn first_event_passes() {
        let mut d = Debouncer::new();
        assert!(d.accept(&ev(EventKind::Modified, "/a")));
    }

    #[test]
    fn rapid_duplicate_is_dropped() {
        let mut d = Debouncer::new();
        assert!(d.accept(&ev(EventKind::Modified, "/a")));
        assert!(!d.accept(&ev(EventKind::Modified, "/a")));
    }

    #[test]
    fn different_kind_is_not_debounced() {
        let mut d = Debouncer::new();
        assert!(d.accept(&ev(EventKind::Modified, "/a")));
        assert!(d.accept(&ev(EventKind::Created, "/a")));
    }

    #[test]
    fn after_window_event_passes_again() {
        let mut d = Debouncer::new();
        assert!(d.accept(&ev(EventKind::Modified, "/a")));
        sleep(DEBOUNCE_WINDOW + Duration::from_millis(20));
        assert!(d.accept(&ev(EventKind::Modified, "/a")));
    }
}
