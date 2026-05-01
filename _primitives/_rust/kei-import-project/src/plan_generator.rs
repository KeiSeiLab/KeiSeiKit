//! plan_generator — cluster map entries into numbered migration phases.
//!
//! Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.
//! Rendering lives in plan_render.rs.

use crate::map_cmd::MapEntry;
use crate::trait_patterns::TraitKind;

// ─────────────────────────── public types ──────────────────────────────────

/// Status of a generated migration phase.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhaseStatus {
    /// Matcher confident; orchestrator + agent can implement.
    Scaffolding,
    /// Low confidence / ambiguous; needs human triage before porting.
    BlockedNeedsReview,
}

/// One cluster of modules sharing the same trait family.
#[derive(Debug, Clone)]
pub struct MigrationPhase {
    pub id: String,
    pub trait_family: String,
    pub modules: Vec<(String, f64)>, // (name, confidence)
    pub priority: u8,
    pub initial_status: PhaseStatus,
}

/// Full output of the plan generator.
#[derive(Debug, Clone)]
pub struct MigrationPlan {
    pub project_name: String,
    pub source_repo: String,
    pub generated_at: String,
    pub phases: Vec<MigrationPhase>,
    pub unmatched_modules: Vec<String>,
    pub total_confidence_avg: f64,
}

// ─────────────────────────── priority heuristic ────────────────────────────

fn priority_for(kind: TraitKind) -> u8 {
    match kind {
        TraitKind::MemoryBackend | TraitKind::AuthProvider | TraitKind::ServiceManager => 0,
        TraitKind::ComputeProvider | TraitKind::GitBackend | TraitKind::NetworkMode => 1,
        TraitKind::NotifyChannel | TraitKind::LlmBackend | TraitKind::Scheduler => 2,
        TraitKind::CostGuard | TraitKind::Backup | TraitKind::Observability => 3,
    }
}

fn tier_prefix(priority: u8) -> &'static str {
    match priority {
        0 => "P0",
        1 => "P1",
        2 => "P2",
        _ => "P3",
    }
}

pub(crate) fn family_name(kind: TraitKind) -> String {
    format!("{kind:?}")
}

// ─────────────────────────── build_plan ────────────────────────────────────

/// Cluster `map_entries` into numbered migration phases.
pub fn build_plan(
    project_name: &str,
    source_repo: &str,
    map_entries: &[MapEntry],
    confidence_threshold: f64,
) -> MigrationPlan {
    let wip_lower = 0.3_f64;
    let mut groups: std::collections::BTreeMap<String, (TraitKind, u8, Vec<(String, f64)>)> =
        std::collections::BTreeMap::new();
    let mut wip_groups: std::collections::BTreeMap<String, (TraitKind, Vec<(String, f64)>)> =
        std::collections::BTreeMap::new();
    let mut unmatched = Vec::new();
    let mut conf_sum = 0.0_f64;
    let mut conf_count = 0_usize;

    for entry in map_entries {
        let conf = entry.best_match.as_ref().map(|m| m.confidence).unwrap_or(0.0);
        if conf >= confidence_threshold {
            let kind = entry.best_match.as_ref().unwrap().kind;
            let pri = priority_for(kind);
            let fam = family_name(kind);
            groups
                .entry(fam)
                .or_insert_with(|| (kind, pri, Vec::new()))
                .2
                .push((entry.module.clone(), conf));
            conf_sum += conf;
            conf_count += 1;
        } else if conf >= wip_lower {
            let kind = entry.best_match.as_ref().map(|m| m.kind);
            if let Some(k) = kind {
                let fam = family_name(k);
                wip_groups
                    .entry(fam)
                    .or_insert_with(|| (k, Vec::new()))
                    .1
                    .push((entry.module.clone(), conf));
            } else {
                unmatched.push(entry.module.clone());
            }
        } else {
            unmatched.push(entry.module.clone());
        }
    }

    let phases = build_phase_list(groups, wip_groups);
    let avg = if conf_count > 0 { conf_sum / conf_count as f64 } else { 0.0 };

    MigrationPlan {
        project_name: project_name.to_owned(),
        source_repo: source_repo.to_owned(),
        generated_at: utc_now(),
        phases,
        unmatched_modules: unmatched,
        total_confidence_avg: avg,
    }
}

type ConfGroups = std::collections::BTreeMap<String, (TraitKind, u8, Vec<(String, f64)>)>;
type WipGroups = std::collections::BTreeMap<String, (TraitKind, Vec<(String, f64)>)>;

fn build_phase_list(groups: ConfGroups, wip_groups: WipGroups) -> Vec<MigrationPhase> {
    let mut sorted: Vec<(u8, String, Vec<(String, f64)>)> = groups
        .into_values()
        .map(|(_kind, pri, mods)| (pri, family_name(_kind), mods))
        .collect();
    sorted.sort_by(|a, b| a.0.cmp(&b.0).then(b.2.len().cmp(&a.2.len())));

    let mut tier_counters = [0u8; 4];
    let mut phases = Vec::new();
    for (pri, fam, mods) in sorted {
        let idx = pri as usize;
        tier_counters[idx] += 1;
        let id = format!("{}.{}", tier_prefix(pri), tier_counters[idx]);
        phases.push(MigrationPhase {
            id,
            trait_family: fam,
            modules: mods,
            priority: pri,
            initial_status: PhaseStatus::Scaffolding,
        });
    }

    let mut wip_counter = 0u8;
    for (_fam, (kind, mods)) in wip_groups {
        if mods.is_empty() {
            continue;
        }
        wip_counter += 1;
        phases.push(MigrationPhase {
            id: format!("Pwip.{wip_counter}"),
            trait_family: family_name(kind),
            modules: mods,
            priority: 99,
            initial_status: PhaseStatus::BlockedNeedsReview,
        });
    }
    phases
}

// ─────────────────────────── render_markdown (re-export) ───────────────────

/// Render a `MigrationPlan` to the HERMES-MIGRATION-PLAN.md format.
/// Delegates to `plan_render` to keep this file ≤200 LOC.
pub fn render_markdown(plan: &MigrationPlan) -> String {
    crate::plan_render::render_markdown(plan)
}

// ─────────────────────────── timestamp helper ──────────────────────────────

fn utc_now() -> String {
    // Lightweight UTC string without chrono dep (≤200 LOC budget).
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    crate::plan_render::epoch_secs_to_iso(secs)
}
