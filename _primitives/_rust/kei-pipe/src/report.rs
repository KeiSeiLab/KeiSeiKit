//! Per-step and DAG-level run reports.
//!
//! A [`StepReport`] is emitted for every step actually attempted, in
//! execution order. A [`DagReport`] aggregates them and exposes the
//! resolver lookup map so later steps can reference earlier outputs.
//!
//! When a step fails, execution halts (sequential runtime) and the
//! failing step is the last entry in `steps`. Callers can check
//! `final_ok()` and inspect `steps.last()` for the error.

use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;

/// One step's outcome.
///
/// `source` is set only when caching was active for the step:
/// `Some("cache")` on a cache hit, `Some("fresh")` on a cache miss (atom
/// was invoked and its result stored), `None` when caching was disabled
/// or the atom kind gated it out.
#[derive(Debug, Clone, Serialize)]
pub struct StepReport {
    pub id: String,
    pub atom: String,
    pub ok: bool,
    pub result: Option<Value>,
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl StepReport {
    pub fn ok(id: &str, atom: &str, result: Value) -> Self {
        Self {
            id: id.into(),
            atom: atom.into(),
            ok: true,
            result: Some(result),
            error: None,
            source: None,
        }
    }
    pub fn fail(id: &str, atom: &str, error: String) -> Self {
        Self {
            id: id.into(),
            atom: atom.into(),
            ok: false,
            result: None,
            error: Some(error),
            source: None,
        }
    }
    /// Builder-style: attach a cache source label (`"cache"` or `"fresh"`).
    pub fn with_source(mut self, source: &str) -> Self {
        self.source = Some(source.into());
        self
    }
}

/// Full-DAG outcome. `final_result` is the `result` of the last
/// successful step, or `null` when nothing ran successfully.
#[derive(Debug, Clone, Default, Serialize)]
pub struct DagReport {
    pub steps: Vec<StepReport>,
    pub final_result: Value,
    /// Resolver lookup — envelope shape `{"atom":..., "result":...}`.
    #[serde(skip)]
    resolver: HashMap<String, Value>,
}

impl DagReport {
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            final_result: Value::Null,
            resolver: HashMap::new(),
        }
    }

    /// Append one step's report. On success, also updates the resolver
    /// map so downstream `$step.result.foo` references work.
    pub fn push(&mut self, step: StepReport) {
        if step.ok {
            let envelope = json!({ "atom": step.atom, "result": step.result });
            self.resolver.insert(step.id.clone(), envelope);
            if let Some(ref r) = step.result {
                self.final_result = r.clone();
            }
        }
        self.steps.push(step);
    }

    /// Borrow the resolver map for downstream `$step.path` lookups.
    pub fn results(&self) -> &HashMap<String, Value> {
        &self.resolver
    }

    /// True when every step completed with `ok = true` AND at least one
    /// step ran (an empty DAG counts as ok-but-empty).
    pub fn final_ok(&self) -> bool {
        self.steps.iter().all(|s| s.ok)
    }
}
