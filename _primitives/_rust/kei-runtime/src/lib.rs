//! kei-runtime — atom invocation runtime + schema linter.
//!
//! Four modules:
//!   - `discover` — walks `<root>/*/atoms/*.md`, parses YAML frontmatter
//!   - `validate` — JSON Schema draft-07 validation of input/output
//!   - `invoke`   — MVP stub: discovers + validates, exec wire-up TBD
//!   - `lint`     — `schema-lint` correctness pass over atom frontmatter
//!
//! Per `docs/SUBSTRATE-SCHEMA.md` §Runtime invocation contract (LOCKED).

pub mod discover;
pub mod invoke;
pub(crate) mod invoke_error;
pub(crate) mod invoke_io;
pub mod lint;
pub mod validate;
