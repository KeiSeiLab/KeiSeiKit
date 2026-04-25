//! Per-step and DAG-level cache configuration types + TOML parsers.
//!
//! Kept separate from `dag.rs` so the core DAG cube stays under the 200-LOC
//! Constructor Pattern limit. Everything here is a pure value type or a
//! small string-validation helper — no I/O, no side effects.

use serde::Deserialize;

use crate::dag::DagError;

/// Per-step or DAG-level cache opt-in. Both fields required when present.
#[derive(Debug, Clone, Copy)]
pub struct CacheConfig {
    pub enabled: bool,
    pub ttl_sec: i64,
}

/// Atom kind as declared in the DAG. Only `Query` and `Transform` are
/// cacheable (pure); `Command` and `Stream` bypass the cache gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepKind {
    Query,
    Transform,
    Command,
    Stream,
}

impl StepKind {
    pub fn is_cacheable(self) -> bool {
        matches!(self, StepKind::Query | StepKind::Transform)
    }
}

/// Internal TOML surface for the `[pipe]` block.
#[derive(Debug, Deserialize, Default)]
pub(crate) struct RawPipe {
    #[serde(default)]
    pub cache: Option<RawCache>,
}

/// Internal TOML surface for per-step or DAG-level `cache = { ... }`.
#[derive(Debug, Deserialize, Default)]
pub(crate) struct RawCache {
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(default, rename = "ttl_sec")]
    pub ttl_sec: Option<i64>,
    #[serde(default)]
    pub db: Option<String>,
}

impl RawCache {
    /// Flatten the TOML view into the public [`CacheConfig`] shape. `db`
    /// is dropped — the caller reads it separately for DAG-level routing.
    pub(crate) fn into_config(self) -> CacheConfig {
        CacheConfig {
            enabled: self.enabled.unwrap_or(false),
            ttl_sec: self.ttl_sec.unwrap_or(0),
        }
    }
}

/// Split the optional `[pipe]` block into `(cache_config, cache_db_path)`.
pub(crate) fn split_pipe_cache(
    raw: Option<RawPipe>,
) -> (Option<CacheConfig>, Option<String>) {
    let Some(p) = raw else { return (None, None); };
    let Some(c) = p.cache else { return (None, None); };
    let db = c.db.clone();
    (Some(c.into_config()), db)
}

/// Parse a `kind = "..."` string into a typed [`StepKind`].
pub(crate) fn parse_kind(step_id: &str, s: &str) -> Result<StepKind, DagError> {
    match s {
        "query" => Ok(StepKind::Query),
        "transform" => Ok(StepKind::Transform),
        "command" => Ok(StepKind::Command),
        "stream" => Ok(StepKind::Stream),
        other => Err(DagError::BadKind(step_id.into(), other.into())),
    }
}
