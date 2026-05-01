//! Adapter registry — the ONE place to add a new `ClientAdapter`.
//!
//! Constructor Pattern: single responsibility — own the concrete list of
//! adapters the binary knows about, in priority order. `adapter::all()`
//! delegates here so callers never have to edit two files when a fifth
//! adapter ships.
//!
//! Adding a 5th adapter: create its file under `adapters/<name>.rs`,
//! register the module in `adapters/mod.rs`, and add one `Box::new(...)`
//! line below. That's it — `detect_active`, `by_name`, `list-adapters`,
//! `mount`, and `detach` all pick it up automatically.
//!
//! Rationale for NOT using the `inventory` crate yet: at the 4→5 scale we
//! don't pay the dependency cost; a plain function is cheaper and easier
//! to audit.

use super::claude_code::ClaudeCodeAdapter;
use super::continue_adapter::ContinueAdapter;
use super::cursor::CursorAdapter;
use super::zed::ZedAdapter;
use crate::adapter::ClientAdapter;

/// Enumerate every adapter the binary knows about, in priority order.
/// Order matters: `detect_active()` returns the first positive hit.
pub fn all_adapters() -> Vec<Box<dyn ClientAdapter>> {
    vec![
        Box::new(ClaudeCodeAdapter::new()),
        Box::new(CursorAdapter::new()),
        Box::new(ContinueAdapter::new()),
        Box::new(ZedAdapter::new()),
    ]
}
