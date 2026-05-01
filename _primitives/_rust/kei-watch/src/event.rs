//! Canonical event type emitted by [`crate::Watcher`].
//!
//! Decoupled from `notify::Event` so downstream consumers don't bind to
//! notify's evolving hierarchy. Only four kinds are emitted:
//! Created / Modified / Deleted / Renamed.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Coarse event classification. All notify sub-kinds fold into these four.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventKind {
    Created,
    Modified,
    Deleted,
    Renamed,
}

impl EventKind {
    /// Short lowercase tag (matches CLI JSON `kind` field).
    pub fn as_str(&self) -> &'static str {
        match self {
            EventKind::Created => "Created",
            EventKind::Modified => "Modified",
            EventKind::Deleted => "Deleted",
            EventKind::Renamed => "Renamed",
        }
    }
}

/// Filesystem event emitted by the watcher.
///
/// `from_path` is `Some(..)` only for `Renamed` events where both endpoints
/// are known at emission time (backend-dependent — see
/// `map::from_notify` for the folding rules).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Event {
    pub kind: EventKind,
    pub path: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub from_path: Option<PathBuf>,
    /// Unix seconds since epoch.
    pub timestamp: i64,
}

impl Event {
    /// Construct a new event; timestamp is captured here.
    pub fn new(kind: EventKind, path: PathBuf, from_path: Option<PathBuf>) -> Self {
        Self {
            kind,
            path,
            from_path,
            timestamp: unix_now(),
        }
    }
}

fn unix_now() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind_as_str_is_stable() {
        assert_eq!(EventKind::Created.as_str(), "Created");
        assert_eq!(EventKind::Modified.as_str(), "Modified");
        assert_eq!(EventKind::Deleted.as_str(), "Deleted");
        assert_eq!(EventKind::Renamed.as_str(), "Renamed");
    }

    #[test]
    fn event_constructs_with_timestamp() {
        let ev = Event::new(EventKind::Created, PathBuf::from("/tmp/x"), None);
        assert!(ev.timestamp > 0);
        assert_eq!(ev.kind, EventKind::Created);
        assert!(ev.from_path.is_none());
    }
}
