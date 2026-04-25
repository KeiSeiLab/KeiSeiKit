//! Verb templates — one module per generic CRUD / graph verb.
//!
//! Each verb exposes `pub fn run(conn, schema, input) -> Result<Value,
//! VerbError>` with JSON in / JSON out. Sibling crates wrap these in
//! their typed atom `Input` / `Output` structs via `serde_json::from_value`.
//!
//! The `input` arg is always a `serde_json::Value`. Verbs extract fields
//! they need and ignore everything else, except `create` / `update` which
//! only copy declared schema fields into SQL (defence against
//! unexpected keys).

pub mod archive;
pub mod create;
pub mod create_defaults;
pub mod delete;
pub mod get;
pub mod link;
pub mod list;
pub mod pk;
pub mod rank;
pub mod search;
pub mod update;
pub(crate) mod update_invariant;
pub mod validate;

/// Full list of supported verbs — SSoT for documentation + schema
/// validation. `EntitySchema.enabled_verbs` entries MUST appear here.
pub const ALL_VERBS: &[&str] = &[
    "create", "get", "list", "search", "update", "delete", "link", "rank", "archive",
];
