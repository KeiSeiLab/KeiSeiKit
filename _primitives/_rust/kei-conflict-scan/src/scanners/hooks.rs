//! Hook-overlap detector — DISABLED (2026-05-12).
//!
//! Previous heuristic flagged any two hook scripts sharing a matcher (event
//! name like `PreToolUse:Edit`, `Stop`, etc.) as a "redundancy conflict".
//!
//! This is fundamentally wrong: Claude Code's hook chain is designed to
//! support N hooks per matcher — they run in registration order, each
//! contributes its own side effect (logging, validation, advisory). Two
//! `Stop`-event hooks are not a conflict, they are the normal architecture.
//!
//! Backlog entry (`~/.claude/memory/sync-repo/backlog.md` 2026-05-11):
//! > "Несколько хуков на один matcher" = false conflict. Claude Code
//! > поддерживает N hooks per event by design. 9 hooks/medium findings —
//! > все ложные. Убрать класс `hooks/medium "shares matcher"` целиком.
//!
//! Scanner kept as a stub returning `Vec::new()` rather than removed from
//! the scanner registry, so the `--only hooks` CLI flag still validates.
//! Real hook-related conflicts (broken shebangs, missing chmod, syntax
//! errors) belong in a future `hooks-validity` scanner — not here.

use crate::conflict::Conflict;
use std::path::Path;

pub fn scan(_root: &Path) -> Vec<Conflict> {
    Vec::new()
}
