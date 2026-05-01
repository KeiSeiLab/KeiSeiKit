//! DAG spec parsing + topological sort.
//!
//! TOML shape — `[[steps]]` array with fields `id`, `atom`, optional
//! `depends-on = [ids...]`, optional `input = { ... }`. Optional per-step
//! `kind = "query|transform|command|stream"` and `cache = { enabled, ttl_sec }`.
//! Optional DAG-level `[pipe] cache = { enabled, ttl_sec, db = "..." }`.
//!
//! Invariants:
//! - `id` and `atom` must be non-empty strings
//! - `id` must be unique across the DAG
//! - every `depends-on` entry must reference a known step id
//! - the dependency graph must be acyclic

use serde::Deserialize;
use serde_json::Value;
use std::collections::HashSet;

pub use crate::topo::topo_sort;

/// Error cases raised while parsing or sorting a DAG.
#[derive(Debug, thiserror::Error)]
pub enum DagError {
    #[error("toml parse: {0}")]
    Toml(String),
    #[error("step {0} missing required field `{1}`")]
    MissingField(String, &'static str),
    #[error("duplicate step id: {0}")]
    DuplicateId(String),
    #[error("step `{0}` depends on unknown id `{1}`")]
    UnknownDep(String, String),
    #[error("cycle detected involving: {0}")]
    Cycle(String),
    #[error("input for step `{0}` must be a TOML table")]
    BadInput(String),
    #[error("step `{0}` has invalid kind `{1}` (expected query|transform|command|stream)")]
    BadKind(String, String),
}

pub use crate::config::{CacheConfig, StepKind};
use crate::config::{parse_kind, split_pipe_cache, RawCache, RawPipe};

/// One atom invocation in a DAG. `input` is retained as `serde_json::Value`
/// so the resolver can walk it uniformly (strings, objects, arrays).
#[derive(Debug, Clone)]
pub struct Step {
    pub id: String,
    pub atom: String,
    pub depends_on: Vec<String>,
    pub input: Value,
    pub kind: Option<StepKind>,
    pub cache: Option<CacheConfig>,
}

/// Parsed DAG. `steps` preserves declaration order so error messages line
/// up with the TOML source. `cache` is the DAG-level default applied to
/// any cacheable step that lacks its own `cache` override.
#[derive(Debug, Clone, Default)]
pub struct DagSpec {
    pub steps: Vec<Step>,
    pub cache: Option<CacheConfig>,
    pub cache_db: Option<String>,
}

/// Internal TOML surface — kept private so callers only see the cleaned
/// `DagSpec` / `Step` shape.
#[derive(Debug, Deserialize)]
struct RawDag {
    #[serde(default)]
    steps: Vec<RawStep>,
    #[serde(default)]
    pipe: Option<RawPipe>,
}

#[derive(Debug, Deserialize)]
struct RawStep {
    id: Option<String>,
    atom: Option<String>,
    #[serde(rename = "depends-on", default)]
    depends_on: Vec<String>,
    #[serde(default)]
    input: Option<toml::Value>,
    #[serde(default)]
    kind: Option<String>,
    #[serde(default)]
    cache: Option<RawCache>,
}

/// Parse TOML text into a cleaned `DagSpec` with per-step validation.
pub fn parse_dag(text: &str) -> Result<DagSpec, DagError> {
    let raw: RawDag = toml::from_str(text).map_err(|e| DagError::Toml(e.to_string()))?;
    let mut seen: HashSet<String> = HashSet::new();
    let mut steps: Vec<Step> = Vec::with_capacity(raw.steps.len());
    for (idx, rs) in raw.steps.into_iter().enumerate() {
        let step = build_step(idx, rs, &mut seen)?;
        steps.push(step);
    }
    let (cache, cache_db) = split_pipe_cache(raw.pipe);
    Ok(DagSpec { steps, cache, cache_db })
}

fn build_step(idx: usize, rs: RawStep, seen: &mut HashSet<String>) -> Result<Step, DagError> {
    let id = rs
        .id
        .filter(|s| !s.is_empty())
        .ok_or_else(|| DagError::MissingField(format!("#{idx}"), "id"))?;
    if !seen.insert(id.clone()) {
        return Err(DagError::DuplicateId(id));
    }
    let atom = rs
        .atom
        .filter(|s| !s.is_empty())
        .ok_or_else(|| DagError::MissingField(id.clone(), "atom"))?;
    let input = normalize_input(&id, rs.input)?;
    let kind = match rs.kind {
        None => None,
        Some(s) => Some(parse_kind(&id, &s)?),
    };
    let cache = rs.cache.map(|c| c.into_config());
    Ok(Step { id, atom, depends_on: rs.depends_on, input, kind, cache })
}

fn normalize_input(id: &str, raw: Option<toml::Value>) -> Result<Value, DagError> {
    let v = raw.unwrap_or(toml::Value::Table(toml::map::Map::new()));
    if !matches!(v, toml::Value::Table(_)) {
        return Err(DagError::BadInput(id.into()));
    }
    let s = serde_json::to_value(v).map_err(|e| DagError::Toml(e.to_string()))?;
    Ok(s)
}

