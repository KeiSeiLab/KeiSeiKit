//! Host path resolution — SSoT for `$KEISEI_HOME` / `$HOME` fallback.
//!
//! Constructor Pattern: single responsibility — resolve the user's home
//! directory for every adapter + the keisei state dir. `$KEISEI_HOME`
//! overrides `$HOME` so integration tests can isolate state per tmpdir.
//! Adapters compose on top of this SSoT; no duplication of the env-var
//! chain across the codebase.

use std::path::PathBuf;

/// Resolve the user's home directory.
///
/// Precedence:
///   1. `$KEISEI_HOME` (test isolation knob)
///   2. `$HOME` (standard POSIX)
///   3. `"."` (degenerate; only hit if both env vars are absent)
pub fn resolve_home() -> PathBuf {
    std::env::var("KEISEI_HOME")
        .ok()
        .map(PathBuf::from)
        .or_else(|| std::env::var("HOME").ok().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Keisei's own state directory (marker file + future per-tool state).
///
/// Rationale: v0.20 stored the marker under `~/.claude/keisei-attached.toml`
/// which baked a Claude-Code-specific subpath into a tool that must support
/// 4+ clients. v0.21 moves it to `~/.keisei/` — independent of any client's
/// config layout. Adapters still own their own per-client dirs
/// (`~/.claude/`, `~/.cursor/`, etc) via their own helpers.
pub fn keisei_state_dir() -> PathBuf {
    resolve_home().join(".keisei")
}
