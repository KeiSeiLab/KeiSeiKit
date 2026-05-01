//! kei-entity-store — Layer A verb-template engine.
//!
//! Provides a schema-driven store that 6 sibling kei-*-store crates can
//! plug into instead of hand-rolling their own `Store::open` + CRUD
//! helpers. An `EntitySchema` declaratively describes one entity table
//! (fields, FTS columns, edge table, enabled verbs); verb modules
//! (`create`, `get`, `list`, `search`, `update`, `delete`, `link`,
//! `rank`) consume the schema and run parameterized SQL.
//!
//! Pilot target: `kei-task` (see its `schema.rs` for an example usage).
//! Follow-up waves: kei-chat-store, kei-content-store, kei-social-store,
//! kei-sage, kei-crossdomain.
//!
//! Per substrate schema v1 this crate stays library-only — no CLI, no
//! `bin`. Each sibling crate remains the user-facing binary.

pub mod ddl;
pub mod ddl_edge;
pub mod ddl_error;
pub mod engine;
pub mod error;
pub mod field;
pub mod schema;
pub mod verbs;

pub use engine::Store;
pub use error::VerbError;
pub use schema::{EdgeKeyKind, EntitySchema, FieldDef, FieldKind};
