//! `DdlError` — typed DDL-generation failures surfaced by the fallible
//! edge-table dispatcher in `ddl::try_edge_table_for`.
//!
//! Split out of `ddl.rs` to keep each file inside the Constructor
//! Pattern 200-LOC cap (1 file = 1 responsibility). `ddl.rs` owns DDL
//! string emission; this module owns the error type only.

use thiserror::Error;

/// Typed DDL-generation failure. Surfaces caller-input problems (e.g.
/// an unsupported `FieldKind` passed as an `edge.extra_columns` entry)
/// as `Result` errors instead of panicking from library code.
#[derive(Debug, Error)]
pub enum DdlError {
    /// Caller passed a `FieldKind` that edge-column DDL cannot emit
    /// (PKs, archive enums, auto-stamped timestamps are disallowed —
    /// see `ddl::try_extra_column` for the supported subset).
    #[error(
        "edge extra_columns: unsupported FieldKind {kind_debug} for column '{column_name}'"
    )]
    UnsupportedExtraColumn {
        kind_debug: String,
        column_name: String,
    },
}
