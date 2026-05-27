//! `kei-pipe` — atom DAG runtime.
//!
//! Reads a TOML DAG spec, topologically sorts the steps, then runs each
//! atom sequentially. JSON output of a step can be referenced by a later
//! step via `$step-id.path.to.field` in its input block.
//!
//! Atom invocation: spawn `<crate-name> run-atom <verb>` with JSON on
//! stdin, parse stdout as JSON. Binary resolution honours
//! `KEI_RUNTIME_BIN_DIR` first, then `PATH` (same contract as
//! `kei-runtime`).
//!
//! Public surface:
//! - [`dag::DagSpec`] / [`dag::Step`] — parsed DAG structures
//! - [`dag::parse_dag`] / [`dag::topo_sort`] — DAG pipeline
//! - [`resolve::resolve_input`] — substitute `$step.path` in input values
//! - [`exec::run_atom`] — invoke one atom via subprocess
//! - [`report::DagReport`] / [`report::StepReport`] — run outcome
//! - [`run_dag`] / [`validate_dag`] — top-level entry points

pub mod config;
pub mod dag;
pub mod exec;
pub mod hot_reload;
pub mod report;
pub mod resolve;
pub mod scheduler_bridge;
pub mod scheduler_denylist;
pub mod topo;

use std::path::{Path, PathBuf};

use crate::dag::{parse_dag, topo_sort, CacheConfig, DagError, DagSpec, Step};
use crate::exec::{run_atom, run_atom_cached, CacheOutcome, ExecError};
use crate::report::{DagReport, StepReport};
use crate::resolve::{resolve_input, ResolveError};

/// Top-level errors from running a DAG.
#[derive(Debug, thiserror::Error)]
pub enum PipeError {
    #[error("read {0}: {1}")]
    Read(String, std::io::Error),
    #[error(transparent)]
    Dag(#[from] DagError),
    #[error(transparent)]
    Resolve(#[from] ResolveError),
    #[error(transparent)]
    Exec(#[from] ExecError),
    #[error("open cache db {0}: {1}")]
    CacheOpen(String, String),
}

/// Parse + topo-sort a DAG file without running any atoms. Returns Ok
/// with the ordered list of step IDs when the DAG is well-formed.
pub fn validate_dag(path: &Path) -> Result<Vec<String>, PipeError> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| PipeError::Read(path.display().to_string(), e))?;
    let spec = parse_dag(&text)?;
    let order = topo_sort(&spec)?;
    Ok(order.into_iter().map(|s| s.id.clone()).collect())
}

/// Parse + execute a DAG file. On success returns a full report; on the
/// first step failure the report still contains every step processed up
/// to (and including) the failing one, with `ok=false` on that step.
pub fn run_dag(path: &Path) -> Result<DagReport, PipeError> {
    let text = std::fs::read_to_string(path)
        .map_err(|e| PipeError::Read(path.display().to_string(), e))?;
    let spec = parse_dag(&text)?;
    let ordered = topo_sort(&spec)?;
    let conn = open_cache_if_needed(&spec)?;
    Ok(execute_sorted(&spec, &ordered, conn.as_ref()))
}

/// Open the SQLite cache Connection only if the DAG declares a path AND
/// at least one step would actually use the cache. Otherwise returns None
/// so the runtime skips the cache layer entirely.
fn open_cache_if_needed(spec: &DagSpec) -> Result<Option<rusqlite::Connection>, PipeError> {
    let Some(db_path) = spec.cache_db.as_ref() else { return Ok(None); };
    let any_cacheable = spec.steps.iter().any(|s| effective_cache(spec, s).is_some());
    if !any_cacheable {
        return Ok(None);
    }
    let conn = kei_cache::store::open(&PathBuf::from(db_path))
        .map_err(|e| PipeError::CacheOpen(db_path.clone(), e.to_string()))?;
    Ok(Some(conn))
}

/// Resolve the effective cache config for a step: per-step wins over
/// DAG-level. Returns None when caching is disabled or the step's kind
/// is not cacheable (only `query` / `transform` are).
fn effective_cache(spec: &DagSpec, step: &Step) -> Option<CacheConfig> {
    let cfg = step.cache.or(spec.cache)?;
    if !cfg.enabled || cfg.ttl_sec <= 0 {
        return None;
    }
    let kind = step.kind?;
    if !kind.is_cacheable() {
        return None;
    }
    Some(cfg)
}

fn execute_sorted(
    spec: &DagSpec,
    steps: &[&Step],
    conn: Option<&rusqlite::Connection>,
) -> DagReport {
    let mut report = DagReport::new();
    for step in steps {
        match run_one_step(spec, step, &report, conn) {
            Ok(sr) => {
                report.push(sr);
            }
            Err(sr) => {
                report.push(sr);
                break;
            }
        }
    }
    report
}

fn run_one_step(
    spec: &DagSpec,
    step: &Step,
    report: &DagReport,
    conn: Option<&rusqlite::Connection>,
) -> Result<StepReport, StepReport> {
    let input_value = match resolve_input(&step.input, report.results()) {
        Ok(v) => v,
        Err(e) => return Err(StepReport::fail(&step.id, &step.atom, format!("resolve: {e}"))),
    };
    let cache_cfg = conn.and_then(|_| effective_cache(spec, step));
    match (conn, cache_cfg) {
        (Some(c), Some(cfg)) => invoke_with_cache(step, &input_value, c, cfg),
        _ => invoke_direct(step, &input_value),
    }
}

fn invoke_direct(step: &Step, input: &serde_json::Value) -> Result<StepReport, StepReport> {
    match run_atom(&step.atom, input) {
        Ok(result) => Ok(StepReport::ok(&step.id, &step.atom, result)),
        Err(e) => Err(StepReport::fail(&step.id, &step.atom, format!("exec: {e}"))),
    }
}

fn invoke_with_cache(
    step: &Step,
    input: &serde_json::Value,
    conn: &rusqlite::Connection,
    cfg: CacheConfig,
) -> Result<StepReport, StepReport> {
    match run_atom_cached(conn, &step.atom, input, cfg.ttl_sec) {
        Ok((result, outcome)) => Ok(StepReport::ok(&step.id, &step.atom, result)
            .with_source(label(outcome))),
        Err(e) => Err(StepReport::fail(&step.id, &step.atom, format!("exec: {e}"))),
    }
}

fn label(o: CacheOutcome) -> &'static str {
    match o {
        CacheOutcome::Hit => "cache",
        CacheOutcome::Fresh => "fresh",
    }
}
