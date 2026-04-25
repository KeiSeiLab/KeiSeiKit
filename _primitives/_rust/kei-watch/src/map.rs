//! Mapping: `notify::Event` → zero or more canonical [`Event`].
//!
//! Folding rules:
//! - `Create(*)`       → `EventKind::Created`
//! - `Modify(Data*)` / `Modify(Any)` / `Modify(Other)` → `EventKind::Modified`
//! - `Remove(*)`       → `EventKind::Deleted`
//! - `Modify(Name(*))` → `EventKind::Renamed` (from_path populated if both
//!                      endpoints present in `paths`; else None)
//! - `Access(*)` / `Modify(Metadata(*))` / `Other` / `Any` → SKIP
//!
//! Rationale: Access events fire constantly on macOS fsevents and are
//! rarely what a hot-reload / drift-detection consumer wants. Metadata
//! changes (mtime-only touch) are likewise noise.

use crate::event::{Event, EventKind};
use notify::event::{EventKind as NK, ModifyKind, RenameMode};

/// Convert one `notify::Event` into 0..N canonical [`Event`]s.
///
/// Returns `Vec` because a single notify event may carry multiple paths
/// (primarily for `Rename::Both`, which we still emit as a single event
/// with `from_path` populated — but we fold multi-path Create/Remove
/// sensibly too, one emitted event per path).
pub fn from_notify(ev: &notify::Event) -> Vec<Event> {
    match ev.kind {
        NK::Create(_) => fan_out(EventKind::Created, ev),
        NK::Remove(_) => fan_out(EventKind::Deleted, ev),
        NK::Modify(ModifyKind::Name(rm)) => rename(rm, ev),
        NK::Modify(ModifyKind::Data(_))
        | NK::Modify(ModifyKind::Any)
        | NK::Modify(ModifyKind::Other) => fan_out(EventKind::Modified, ev),
        // Skip: Access, Modify(Metadata(*)), Other, Any.
        _ => Vec::new(),
    }
}

/// Emit one canonical event per path in `ev.paths`.
fn fan_out(kind: EventKind, ev: &notify::Event) -> Vec<Event> {
    ev.paths
        .iter()
        .map(|p| Event::new(kind, p.clone(), None))
        .collect()
}

/// Rename mapping. `RenameMode::Both` carries `[from, to]` in paths;
/// other modes may carry only a single path (backend-dependent — see
/// crate-level docs). Callers receive partial information on those.
fn rename(mode: RenameMode, ev: &notify::Event) -> Vec<Event> {
    match mode {
        RenameMode::Both if ev.paths.len() >= 2 => {
            let from = ev.paths[0].clone();
            let to = ev.paths[1].clone();
            vec![Event::new(EventKind::Renamed, to, Some(from))]
        }
        // RenameMode::To: path is the destination; no `from` known here.
        // RenameMode::From: path is the origin; this event effectively
        // says "this path moved away" — we surface it as Renamed with
        // only `path` populated (no destination known yet).
        // RenameMode::Any / Other: same — emit whatever path we have.
        _ => ev
            .paths
            .iter()
            .map(|p| Event::new(EventKind::Renamed, p.clone(), None))
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use notify::event::{AccessKind, CreateKind, MetadataKind, RemoveKind};
    use std::path::PathBuf;

    fn nev(kind: NK, paths: Vec<PathBuf>) -> notify::Event {
        let mut e = notify::Event::new(kind);
        e.paths = paths;
        e
    }

    #[test]
    fn create_maps_to_created() {
        let e = nev(NK::Create(CreateKind::File), vec!["/a".into()]);
        let out = from_notify(&e);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].kind, EventKind::Created);
    }

    #[test]
    fn access_is_skipped() {
        let e = nev(NK::Access(AccessKind::Read), vec!["/a".into()]);
        assert!(from_notify(&e).is_empty());
    }

    #[test]
    fn metadata_is_skipped() {
        let e = nev(
            NK::Modify(ModifyKind::Metadata(MetadataKind::AccessTime)),
            vec!["/a".into()],
        );
        assert!(from_notify(&e).is_empty());
    }

    #[test]
    fn rename_both_populates_from_path() {
        let e = nev(
            NK::Modify(ModifyKind::Name(RenameMode::Both)),
            vec!["/a".into(), "/b".into()],
        );
        let out = from_notify(&e);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].kind, EventKind::Renamed);
        assert_eq!(out[0].path, PathBuf::from("/b"));
        assert_eq!(out[0].from_path, Some(PathBuf::from("/a")));
    }

    #[test]
    fn remove_maps_to_deleted() {
        let e = nev(NK::Remove(RemoveKind::File), vec!["/a".into()]);
        let out = from_notify(&e);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].kind, EventKind::Deleted);
    }
}
