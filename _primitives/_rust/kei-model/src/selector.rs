//! `resolve` — pick cheapest active model for a (role, budget, caps) triple.
//!
//! Algorithm:
//!   1. Filter to `Status::Active`.
//!   2. Filter to models declaring all required `caps`.
//!   3. Filter to models matching the role tag (or, if no model carries the
//!      tag, fall back to `selectors.toml [defaults]` to pick a target id).
//!   4. Filter by budget (1k input + 1k output baseline cost ≤ budget_micro).
//!   5. Sort by input rate ASC, then output rate ASC.
//!   6. Return the cheapest.

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

use crate::model::{Capability, Model, Status};
use crate::pricing::estimate;
use crate::registry::Registry;

#[derive(Debug, Deserialize)]
struct SelectorsFile {
    #[serde(default)]
    defaults: std::collections::BTreeMap<String, String>,
}

/// Outcome of `resolve`.
#[derive(Debug, Clone)]
pub struct Resolution {
    pub model: Model,
    pub reason: String,
}

/// Pick the cheapest active model that satisfies role + caps + budget.
pub fn resolve(
    role: &str,
    budget_micro: Option<u64>,
    caps: &[Capability],
    registry: &Registry,
    selectors_path: Option<&Path>,
) -> Result<Resolution> {
    let candidates: Vec<&Model> = registry
        .list_all()
        .iter()
        .filter(|m| m.status == Status::Active)
        .filter(|m| m.has_all_caps(caps))
        .collect();

    let role_filtered = filter_by_role(&candidates, role, registry, selectors_path)?;
    let budget_filtered = filter_by_budget(&role_filtered, budget_micro);

    let chosen = cheapest(&budget_filtered)
        .ok_or_else(|| no_match_error(role, budget_micro, caps))?;
    let reason = build_reason(role, budget_micro, caps);
    Ok(Resolution { model: chosen.clone(), reason })
}

fn filter_by_role<'a>(
    pool: &[&'a Model],
    role: &str,
    registry: &'a Registry,
    selectors_path: Option<&Path>,
) -> Result<Vec<&'a Model>> {
    let direct: Vec<&Model> = pool.iter().filter(|m| m.has_role(role)).copied().collect();
    if !direct.is_empty() {
        return Ok(direct);
    }
    let path = resolve_selectors_path(selectors_path)?;
    if let Some(default_id) = lookup_default(&path, role)? {
        if let Some(m) = registry.get(&default_id) {
            return Ok(vec![m]);
        }
    }
    Ok(Vec::new())
}

fn filter_by_budget<'a>(pool: &[&'a Model], budget_micro: Option<u64>) -> Vec<&'a Model> {
    let cap = match budget_micro {
        Some(b) => b,
        None => return pool.to_vec(),
    };
    pool.iter()
        .copied()
        .filter(|m| estimate(&m.pricing, 1_000, 1_000) <= cap)
        .collect()
}

fn cheapest<'a>(pool: &[&'a Model]) -> Option<&'a Model> {
    pool.iter()
        .copied()
        .min_by(|a, b| compare_by_price(a, b))
}

fn compare_by_price(a: &Model, b: &Model) -> std::cmp::Ordering {
    a.pricing
        .input_per_mtok_micro
        .cmp(&b.pricing.input_per_mtok_micro)
        .then(a.pricing.output_per_mtok_micro.cmp(&b.pricing.output_per_mtok_micro))
        .then(a.id.cmp(&b.id))
}

fn lookup_default(path: &Path, role: &str) -> Result<Option<String>> {
    let txt = std::fs::read_to_string(path)
        .with_context(|| format!("read selectors.toml at {}", path.display()))?;
    let parsed: SelectorsFile = toml::from_str(&txt)
        .with_context(|| format!("parse selectors.toml at {}", path.display()))?;
    Ok(parsed.defaults.get(role).cloned())
}

/// Resolve selectors.toml: arg → env → compiled-in default.
pub fn resolve_selectors_path(arg: Option<&Path>) -> Result<PathBuf> {
    if let Some(p) = arg {
        return Ok(p.to_path_buf());
    }
    if let Ok(env_path) = std::env::var("KEI_MODEL_SELECTORS") {
        return Ok(PathBuf::from(env_path));
    }
    let default = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/selectors.toml");
    if !default.exists() {
        return Err(anyhow!("default selectors missing: {}", default.display()));
    }
    Ok(default)
}

fn build_reason(role: &str, budget_micro: Option<u64>, caps: &[Capability]) -> String {
    let cap_names: Vec<&str> = caps.iter().map(|c| c.as_str()).collect();
    let budget = budget_micro
        .map(|b| format!("budget≤{b} micro/Mtok"))
        .unwrap_or_else(|| "no budget cap".into());
    format!("role={role}, caps=[{}], {budget}, cheapest active match", cap_names.join(","))
}

fn no_match_error(role: &str, budget_micro: Option<u64>, caps: &[Capability]) -> anyhow::Error {
    let cap_names: Vec<&str> = caps.iter().map(|c| c.as_str()).collect();
    anyhow!(
        "no active model matches role={role}, caps=[{}], budget={:?}",
        cap_names.join(","),
        budget_micro
    )
}
