//! Built-in schemas — 5 shipped schemas, embedded at compile time.
//!
//! Chain: architect(spec) → code-implementer(plan → patch) →
//!        critic/security(review) → researcher(research) feeds back.
//! Each file lives in `kei-artifact/schemas/*.json` and is embedded via
//! `include_str!` so the CLI `--self-register` path needs no filesystem.

use crate::artifact::register_schema;
use crate::store::Store;
use anyhow::Result;

/// (name, schema JSON text). Keep in sync with `schemas/*.json`.
pub const BUILTIN: &[(&str, &str)] = &[
    ("spec", include_str!("../schemas/spec.json")),
    ("plan", include_str!("../schemas/plan.json")),
    ("patch", include_str!("../schemas/patch.json")),
    ("review", include_str!("../schemas/review.json")),
    ("research", include_str!("../schemas/research.json")),
];

/// Register all 5 built-in schemas. Idempotent.
pub fn register_builtins(store: &Store) -> Result<()> {
    for (name, text) in BUILTIN {
        register_schema(store, name, text)?;
    }
    Ok(())
}
