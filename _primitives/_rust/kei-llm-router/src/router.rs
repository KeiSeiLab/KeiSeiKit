//! Core decision logic for kei-llm-router.
//!
//! Constructor Pattern: ONE responsibility — turn `(Machine, model_id, opts)`
//! into a `RouteDecision`. Two layers:
//!
//! 1. **Pure** — `decide()` accepts pre-probed candidates and the machine
//!    snapshot, returns a decision. Deterministic; unit-testable without
//!    network.
//! 2. **Live** — `route()` calls `discovery::discover_models` then `decide()`.
//!    The async surface — used by the CLI binary.
//!
//! The router does NOT spawn subprocesses; every backend interaction goes
//! through `health` / `discovery` which delegate to W57/W58/W59 crates.

use serde::{Deserialize, Serialize};

use crate::backend::{Backend, BackendKind};
use crate::discovery::{discover_models, ModelMatch};
use crate::error::{Error, Result};
use kei_machine_probe::{recommend, Capability, Machine};
use kei_model::{Pricing, Registry};

/// Caller-supplied options for one route decision.
#[derive(Debug, Clone, Default)]
pub struct RouteOpts {
    pub require_local: bool,
    pub role: Option<String>,
    pub budget_micro: Option<u64>,
}

/// Outcome of `route` / `decide`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RouteDecision {
    pub backend: Backend,
    pub machine_tier: Capability,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_pricing: Option<Pricing>,
    pub rationale: Vec<String>,
}

/// Live route — probe + decide. Used by the CLI binary.
pub async fn route(
    machine: &Machine,
    model_id: &str,
    opts: &RouteOpts,
    registry: Option<&Registry>,
) -> Result<RouteDecision> {
    let candidates = discover_models(machine, model_id).await;
    decide(machine, model_id, &candidates, opts, registry)
}

/// Pure decision — choose a backend given pre-probed candidates.
pub fn decide(
    machine: &Machine,
    model_id: &str,
    candidates: &[(BackendKind, ModelMatch)],
    opts: &RouteOpts,
    registry: Option<&Registry>,
) -> Result<RouteDecision> {
    let recs = recommend(machine);
    if recs.capability == Capability::NoLocalInferenceViable && opts.require_local {
        return Err(Error::NoCompatibleBackend {
            reason: recs.rationale.join("; "),
        });
    }
    if let Some(d) = pick_from_candidates(model_id, candidates, &recs.capability) {
        return Ok(annotate(d, registry, model_id));
    }
    if !opts.require_local {
        if let Some(reg) = registry {
            if let Some(d) = walk_fallback(machine, model_id, candidates, reg, &recs.capability) {
                return Ok(annotate(d, Some(reg), model_id));
            }
        }
    }
    Err(Error::NoBackendAvailable {
        model_id: model_id.into(),
        tried: candidate_kind_strs(candidates),
    })
}

fn pick_from_candidates(
    model_id: &str,
    candidates: &[(BackendKind, ModelMatch)],
    cap: &Capability,
) -> Option<RouteDecision> {
    let (kind, m) = candidates.first()?;
    let backend = build_backend(*kind, model_id, m);
    Some(RouteDecision {
        backend,
        machine_tier: cap.clone(),
        model_pricing: None,
        rationale: vec![format!(
            "first viable backend `{}` matched `{}` ({})",
            kind, model_id, match_label(m)
        )],
    })
}

fn build_backend(kind: BackendKind, model_id: &str, m: &ModelMatch) -> Backend {
    let chosen = m.alternative.clone().unwrap_or_else(|| model_id.to_string());
    match kind {
        BackendKind::Mlx => Backend::Mlx { model_id: chosen },
        BackendKind::LlamaCpp => Backend::LlamaCpp {
            gguf_path: std::path::PathBuf::from(&chosen),
            model_name: chosen,
        },
        BackendKind::Ollama => Backend::Ollama { model_tag: chosen },
    }
}

fn match_label(m: &ModelMatch) -> &'static str {
    if m.exact { "exact" } else { "fuzzy" }
}

fn annotate(
    mut decision: RouteDecision,
    registry: Option<&Registry>,
    model_id: &str,
) -> RouteDecision {
    if let Some(reg) = registry {
        if let Some(model) = reg.get(model_id) {
            decision.model_pricing = Some(model.pricing.clone());
            decision.rationale.push(format!(
                "registry pricing status: {}",
                model.pricing.status.as_str()
            ));
        }
    }
    decision
}

fn walk_fallback(
    machine: &Machine,
    primary_id: &str,
    candidates: &[(BackendKind, ModelMatch)],
    reg: &Registry,
    cap: &Capability,
) -> Option<RouteDecision> {
    let chain = kei_model::chain(primary_id, reg).ok()?;
    for next in chain.iter().skip(1) {
        if let Some(d) = pick_from_candidates(&next.id, candidates, cap) {
            let _ = machine; // reserved for future per-fallback re-probe
            return Some(d);
        }
    }
    None
}

fn candidate_kind_strs(candidates: &[(BackendKind, ModelMatch)]) -> Vec<String> {
    if candidates.is_empty() {
        return vec!["<none>".into()];
    }
    candidates.iter().map(|(k, _)| k.as_str().to_string()).collect()
}
