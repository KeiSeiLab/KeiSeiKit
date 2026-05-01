//! kei-watch — filesystem watcher primitive.
//!
//! Thin, synchronous wrapper around the [`notify`] crate. Emits a stable
//! canonical event format so downstream consumers (kei-pipe hot-reload,
//! kei-replay drift detection, dev-loop cache invalidation) don't bind
//! to notify's evolving [`notify::EventKind`] hierarchy.
//!
//! # Surface
//!
//! | Type | Role |
//! |------|------|
//! | [`Watcher`]    | owns notify backend + pump thread |
//! | [`Event`]      | canonical event ({kind, path, from_path, timestamp}) |
//! | [`EventKind`]  | `Created` / `Modified` / `Deleted` / `Renamed` |
//! | [`WatchError`] | failure modes (Io / NotifyBackend / PathNotFound / WatchNotFound) |
//!
//! # Example
//! ```no_run
//! use kei_watch::{Watcher, EventKind};
//! use std::{path::Path, time::Duration};
//!
//! let mut w = Watcher::new().unwrap();
//! w.watch(Path::new("."), true).unwrap();
//! while let Some(ev) = w.next_event(Duration::from_secs(1)) {
//!     if ev.kind == EventKind::Modified {
//!         println!("{}", ev.path.display());
//!     }
//! }
//! ```
//!
//! # Platform notes
//!
//! Rename semantics differ by backend. On macOS/Windows,
//! `RenameMode::Both` is emitted with both endpoints and we populate
//! `from_path`. On Linux (inotify), rename fires as two events
//! (`RenameMode::From` then `RenameMode::To`) correlated by tracker;
//! kei-watch emits each as a separate `Renamed` with `from_path=None`.
//! Downstream code that needs strict from→to pairing should fall back
//! to notify-debouncer-full.

pub mod debounce;
pub mod error;
pub mod event;
pub mod map;
pub mod pump;
pub mod watcher;

pub use error::WatchError;
pub use event::{Event, EventKind};
pub use watcher::Watcher;
