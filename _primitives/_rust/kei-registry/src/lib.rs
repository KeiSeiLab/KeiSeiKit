//! kei-registry — universal block identity layer.
//!
//! Generalises the agent DNA pattern (kei-shared::dna + kei-ledger) to ANY
//! kit block: primitive crate, skill, rule, hook, atom. One SQLite store
//! at `~/.claude/registry.sqlite`, one `<block_type>::<caps>::<scope_sha8>::
//! <body_sha8>-<nonce8>` DNA wire format per block, idempotent re-register,
//! supersede chain on body change.
//!
//! Constructor Pattern: each module is one cube with one responsibility.
//! Wire-format SSoT lives in `kei_shared::dna` — `dna_block::compose_for_block`
//! delegates to `kei_shared::compose_dna` so the format string exists in
//! exactly one place.

pub mod block;
pub mod cli;
pub mod diff;
pub mod dna_block;
pub mod encyclopedia;
pub mod encyclopedia_render;
pub mod encyclopedia_time;
pub mod handlers;
pub mod index_substrate;
pub mod lookup;
pub mod paths;
pub mod registry;
pub mod related;
pub mod scan_orchestrator;
pub mod scanners;
pub mod secrets;
pub mod secrets_handler;
pub mod stats;
pub mod status;
pub mod store;

pub use block::{Block, BlockType};
pub use diff::{diff_blocks, BlockDiff};
pub use dna_block::{compose_for_block, compose_for_block_with_nonce};
pub use registry::{find_by_path, get, list, list_by_type, mark_superseded, register};
pub use related::{find_related, RelatedHit};
// Both `secrets` and `status` expose a `render_ascii` function — keep them
// module-qualified at the top level (avoid re-exporting `render_ascii` to
// prevent a name collision; callers use `secrets::render_ascii` /
// `status::render_ascii`).
pub use secrets::{compute_secrets_report, SecretsReport};
pub use stats::{compute_stats, Stats};
pub use status::{compute_status, Status};
pub use store::{open_db, SCHEMA_VERSION};
