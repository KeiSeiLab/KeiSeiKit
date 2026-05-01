//! Concrete `ClientAdapter` implementations, one file per client.
//!
//! Constructor Pattern: this file is the module declaration hub only —
//! no logic lives here. `jsonmcp` owns the shared JSON merge helpers
//! used by every JSON-keyed adapter (claude-code, cursor, zed).
//! `_registry` is the single canonical adapter list (v0.22).

#[path = "_registry.rs"]
pub mod _registry;
pub mod claude_code;
pub mod continue_adapter;
pub mod cursor;
pub mod jsonmcp;
pub mod zed;
