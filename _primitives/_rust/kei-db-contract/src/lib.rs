//! kei-db-contract — diff SQL migration schemas against TypeScript types.
//!
//! Public API exists for integration tests. The binary lives in `main.rs`.

pub mod diff;
pub mod matching;
pub mod output;
pub mod report_builders;
pub mod sql_parse;
pub mod ts_parse;
pub mod types_map;

pub use diff::{diff_project, DriftReport, FieldStatus, TableStatus};
pub use matching::pair_tables_with_types;
pub use sql_parse::{parse_migrations_dir, SqlColumn, SqlTable};
pub use ts_parse::{parse_ts_glob, TsField, TsType};
