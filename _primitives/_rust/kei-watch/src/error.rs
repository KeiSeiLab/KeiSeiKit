//! Error type for the kei-watch primitive.
//!
//! Wraps `notify` + `io` errors behind a stable surface so downstream
//! consumers don't bind to notify's error hierarchy.

use std::fmt;
use std::path::PathBuf;

/// Failure modes for [`crate::Watcher`] operations.
#[derive(Debug)]
pub enum WatchError {
    /// Underlying OS I/O failure.
    Io(std::io::Error),
    /// notify backend failed to start or watch.
    NotifyBackend(String),
    /// Path given to `watch` does not exist on disk.
    PathNotFound(PathBuf),
    /// `unwatch` called on a path that was never registered.
    WatchNotFound(PathBuf),
}

impl fmt::Display for WatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WatchError::Io(e) => write!(f, "io: {e}"),
            WatchError::NotifyBackend(s) => write!(f, "notify backend: {s}"),
            WatchError::PathNotFound(p) => write!(f, "path not found: {}", p.display()),
            WatchError::WatchNotFound(p) => write!(f, "watch not found: {}", p.display()),
        }
    }
}

impl std::error::Error for WatchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            WatchError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for WatchError {
    fn from(e: std::io::Error) -> Self {
        WatchError::Io(e)
    }
}

/// Convert a `notify::Error` into `WatchError`, preserving the path hint
/// for path-related errors where possible.
pub fn from_notify(err: notify::Error) -> WatchError {
    use notify::ErrorKind as NK;
    let first_path = err.paths.first().cloned();
    match err.kind {
        NK::PathNotFound => {
            WatchError::PathNotFound(first_path.unwrap_or_default())
        }
        NK::WatchNotFound => {
            WatchError::WatchNotFound(first_path.unwrap_or_default())
        }
        NK::Io(ioe) => WatchError::Io(ioe),
        NK::Generic(s) => WatchError::NotifyBackend(s),
        NK::InvalidConfig(_) => {
            WatchError::NotifyBackend("invalid config".into())
        }
        NK::MaxFilesWatch => {
            WatchError::NotifyBackend("OS watch limit reached".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_has_prefix() {
        let e = WatchError::PathNotFound(PathBuf::from("/nope"));
        assert!(format!("{e}").starts_with("path not found"));
    }

    #[test]
    fn notify_path_not_found_maps() {
        let ne = notify::Error::path_not_found().add_path(PathBuf::from("/x"));
        match from_notify(ne) {
            WatchError::PathNotFound(p) => assert_eq!(p, PathBuf::from("/x")),
            other => panic!("expected PathNotFound, got {other:?}"),
        }
    }
}
