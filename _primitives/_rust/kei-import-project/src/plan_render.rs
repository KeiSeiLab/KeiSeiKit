//! plan_render — markdown renderer for MigrationPlan.
//!
//! Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.

use crate::plan_generator::{MigrationPlan, PhaseStatus};

/// Render a `MigrationPlan` to the HERMES-MIGRATION-PLAN.md format.
pub fn render_markdown(plan: &MigrationPlan) -> String {
    let mut out = String::new();
    push_header(&mut out, plan);
    push_status_banner(&mut out, plan);
    push_phase_table(&mut out, plan);
    push_per_phase_detail(&mut out, plan);
    push_unmatched(&mut out, plan);
    push_follow_up(&mut out);
    out
}

fn push_header(out: &mut String, plan: &MigrationPlan) {
    out.push_str(&format!("# {} — Migration Plan\n\n", plan.project_name));
    out.push_str(&format!("> Generated: {}\n", plan.generated_at));
    out.push_str(&format!("> Source: {}\n", plan.source_repo));
    out.push_str(&format!("> Average confidence: {:.2}\n\n", plan.total_confidence_avg));
}

fn push_status_banner(out: &mut String, plan: &MigrationPlan) {
    out.push_str("## STATUS BANNER\n\n");
    let blocked: usize = plan
        .phases
        .iter()
        .filter(|p| p.initial_status == PhaseStatus::BlockedNeedsReview)
        .count();
    if plan.phases.is_empty() {
        out.push_str("> **WARNING: no modules matched any trait at the given threshold.**\n");
        out.push_str("> Lower `--threshold` or check that the repo contains Rust crates.\n\n");
    } else if blocked > 0 {
        out.push_str(&format!(
            "> **AUTO-GENERATED plan. {blocked} phase(s) blocked — needs human triage.**\n"
        ));
        out.push_str("> Run `kei-import-project execute <plan.md>` (Phase 5) after triage.\n\n");
    } else {
        out.push_str("> **AUTO-GENERATED initial plan. All phases initial status: scaffolding.**\n");
        out.push_str(
            "> Run `kei-import-project execute <plan.md>` (Phase 5) to spawn agents per phase.\n",
        );
        out.push_str("> Each agent must finish with STATUS-TRUTH MARKER (RULE 0.16).\n\n");
    }
}

fn push_phase_table(out: &mut String, plan: &MigrationPlan) {
    out.push_str("| Phase | Trait family | Modules | Priority | Initial status |\n");
    out.push_str("|---|---|---:|---:|---|\n");
    for p in &plan.phases {
        let status = match p.initial_status {
            PhaseStatus::Scaffolding => "scaffolding",
            PhaseStatus::BlockedNeedsReview => "blocked-needs-review",
        };
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            p.id,
            p.trait_family,
            p.modules.len(),
            p.priority,
            status
        ));
    }
    out.push('\n');
}

fn push_per_phase_detail(out: &mut String, plan: &MigrationPlan) {
    out.push_str("## Per-phase detail\n\n");
    for p in &plan.phases {
        out.push_str(&format!("### {} — {}\n\n", p.id, p.trait_family));
        out.push_str("Modules to port:\n");
        let mut sorted_mods = p.modules.clone();
        sorted_mods.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        for (name, conf) in &sorted_mods {
            out.push_str(&format!("- {} (confidence {:.2})\n", name, conf));
        }
        out.push_str("\nVerification gate (RULE 0.13 + RULE 0.16):\n");
        out.push_str("- `cargo check --workspace` PASS\n");
        out.push_str("- `cargo test -p <crate>` PASS\n");
        out.push_str("- STATUS-TRUTH MARKER `shipped: functional`\n\n");
    }
}

fn push_unmatched(out: &mut String, plan: &MigrationPlan) {
    out.push_str("## Unmatched modules\n\n");
    if plan.unmatched_modules.is_empty() {
        out.push_str("All modules matched at the given threshold.\n\n");
    } else {
        out.push_str(&format!(
            "These do not match any KeiSeiKit-runtime-core trait at threshold {:.2}.\n",
            plan.total_confidence_avg
        ));
        out.push_str("Manual classification required before they can be ported.\n\n");
        for m in &plan.unmatched_modules {
            out.push_str(&format!("- {}\n", m));
        }
        out.push('\n');
    }
}

fn push_follow_up(out: &mut String) {
    out.push_str("## Follow-up\n\n");
    out.push_str(
        "- Apply skeletons: `kei-import-project skeleton --module <m> --trait-name <t>`\n",
    );
    out.push_str("- Execute phases: `kei-import-project execute plan.md` (Phase 5)\n");
}

// ─────────────────────────── timestamp helper ──────────────────────────────

/// Convert Unix epoch seconds to an ISO-8601 UTC string.
/// Used by plan_generator (avoids chrono dep; pulled here to respect ≤200 LOC budget).
pub(crate) fn epoch_secs_to_iso(secs: u64) -> String {
    let s = secs % 60;
    let m = (secs / 60) % 60;
    let h = (secs / 3600) % 24;
    let mut days = secs / 86400;
    let mut year = 1970u64;
    loop {
        let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
        let dy = if leap { 366 } else { 365 };
        if days < dy { break; }
        days -= dy;
        year += 1;
    }
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let months = if leap { [31u64,29,31,30,31,30,31,31,30,31,30,31] }
                 else    { [31,28,31,30,31,30,31,31,30,31,30,31] };
    let mut month = 1u64;
    for &dm in &months { if days < dm { break; } days -= dm; month += 1; }
    format!("{year:04}-{month:02}-{:02}T{h:02}:{m:02}:{s:02}Z", days + 1)
}
